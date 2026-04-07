use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use sqlx::Row;
use sqlx::types::Json;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateRawIngestionEventInput {
    pub source_system: String,
    pub source_object_type: String,
    pub source_object_id: String,
    pub source_event_key: String,
    pub source_version: Option<String>,
    pub source_updated_at: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct RawIngestionAcceptedRecord {
    pub request_id: String,
    pub accepted: bool,
    pub run_id: String,
    pub status: String,
    pub message: String,
    pub source_system: String,
}

#[derive(Debug)]
pub enum RawIngestionRepositoryError {
    InvalidSourceUpdatedAt(String),
    Database(sqlx::Error),
}

impl Display for RawIngestionRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSourceUpdatedAt(value) => {
                write!(f, "invalid source_updated_at: {value}")
            }
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for RawIngestionRepositoryError {}

impl From<sqlx::Error> for RawIngestionRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Default)]
pub struct RawIngestionStore {
    records: Vec<RawIngestionAcceptedRecord>,
}

impl RawIngestionStore {
    pub fn create(&mut self, input: CreateRawIngestionEventInput) -> RawIngestionAcceptedRecord {
        if let Some(existing) = self
            .records
            .iter()
            .find(|record| record.request_id == input.source_event_key)
            .cloned()
        {
            return RawIngestionAcceptedRecord {
                accepted: false,
                status: "duplicate".to_string(),
                message: "duplicate event ignored".to_string(),
                ..existing
            };
        }

        let record = RawIngestionAcceptedRecord {
            request_id: input.source_event_key.clone(),
            accepted: true,
            run_id: format!("ing_{}", Uuid::new_v4().simple()),
            status: "accepted".to_string(),
            message: "payload accepted for asynchronous processing".to_string(),
            source_system: input.source_system,
        };
        self.records.push(record.clone());
        record
    }
}

pub struct RawIngestionRepository {
    pool: PgPool,
}

impl RawIngestionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        input: CreateRawIngestionEventInput,
    ) -> Result<RawIngestionAcceptedRecord, RawIngestionRepositoryError> {
        let source_updated_at = parse_optional_timestamp(input.source_updated_at.as_deref())?;
        let payload_text = serde_json::to_string(&input.payload).expect("payload should serialize");
        let payload_hash = hash_payload(&payload_text);
        let job_id = self.ensure_push_ingestion_job().await?;
        let run_external_id = format!("ing_{}", Uuid::new_v4().simple());
        let run_scope = serde_json::json!({
            "source_object_type": input.source_object_type,
            "source_object_id": input.source_object_id,
        });

        let result = self
            .insert_raw_event(
                &run_external_id,
                Some(job_id),
                &input.source_system,
                "push",
                run_scope,
                None,
                &input,
                source_updated_at,
                payload_text,
                payload_hash,
            )
            .await?;

        Ok(result)
    }

    #[allow(dead_code)]
    pub async fn create_for_run(
        &self,
        run_id: &str,
        input: CreateRawIngestionEventInput,
    ) -> Result<RawIngestionAcceptedRecord, RawIngestionRepositoryError> {
        let source_updated_at = parse_optional_timestamp(input.source_updated_at.as_deref())?;
        let payload_text = serde_json::to_string(&input.payload).expect("payload should serialize");
        let payload_hash = hash_payload(&payload_text);
        let run_row = sqlx::query(
            "select source_system, run_mode, run_scope from integration_run where external_run_id = $1",
        )
        .bind(run_id)
        .fetch_one(&self.pool)
        .await?;
        let source_system: String = run_row.get("source_system");
        let run_mode: String = run_row.get("run_mode");
        let run_scope: Json<serde_json::Value> = run_row.get("run_scope");

        self.insert_raw_event(
            run_id,
            None,
            &source_system,
            &run_mode,
            run_scope.0,
            None,
            &input,
            source_updated_at,
            payload_text,
            payload_hash,
        )
        .await
    }

    async fn insert_raw_event(
        &self,
        run_external_id: &str,
        maybe_job_id: Option<Uuid>,
        run_source_system: &str,
        run_mode: &str,
        run_scope: serde_json::Value,
        run_reason: Option<String>,
        input: &CreateRawIngestionEventInput,
        source_updated_at: Option<DateTime<Utc>>,
        payload_text: String,
        payload_hash: String,
    ) -> Result<RawIngestionAcceptedRecord, RawIngestionRepositoryError> {
        let duplicate = sqlx::query(
            r#"
            select rie.raw_ingestion_event_id, ir.external_run_id
            from raw_ingestion_event rie
            join integration_run ir on ir.integration_run_id = rie.integration_run_id
            where rie.source_system = $1
              and rie.source_object_type = $2
              and rie.source_object_id = $3
              and rie.source_event_key = $4
            "#,
        )
        .bind(&input.source_system)
        .bind(&input.source_object_type)
        .bind(&input.source_object_id)
        .bind(&input.source_event_key)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = duplicate {
            let run_id: String = row.get("external_run_id");
            return Ok(RawIngestionAcceptedRecord {
                request_id: input.source_event_key.clone(),
                accepted: false,
                run_id,
                status: "duplicate".to_string(),
                message: "duplicate event ignored".to_string(),
                source_system: input.source_system.clone(),
            });
        }

        let run_internal_id = if let Some(existing_id) = sqlx::query_scalar::<_, Uuid>(
            "select integration_run_id from integration_run where external_run_id = $1",
        )
        .bind(run_external_id)
        .fetch_optional(&self.pool)
        .await?
        {
            existing_id
        } else {
            Uuid::new_v4()
        };
        let raw_event_id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.pool.begin().await?;

        if let Some(job_id) = maybe_job_id {
            sqlx::query(
                r#"
                insert into integration_run (
                  integration_run_id,
                  integration_job_id,
                  external_run_id,
                  source_system,
                  run_mode,
                  run_scope,
                  reason,
                  queued_at,
                  run_status,
                  status_reason_code,
                  processed_count,
                  success_count,
                  failure_count,
                  pending_count,
                  created_at
                )
                values (
                  $1, $2, $3, $4, $5, $6, $7, $8, 'queued', 'payload_accepted', 0, 0, 0, 0, $8
                )
                "#,
            )
            .bind(run_internal_id)
            .bind(job_id)
            .bind(run_external_id)
            .bind(run_source_system)
            .bind(run_mode)
            .bind(Json(&run_scope))
            .bind(&run_reason)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query(
            r#"
            insert into raw_ingestion_event (
              raw_ingestion_event_id,
              integration_run_id,
              source_system,
              source_object_type,
              source_object_id,
              source_event_key,
              source_version,
              source_sequence_no,
              source_updated_at,
              source_record_key,
              payload_reference,
              payload_hash,
              ingested_at,
              normalization_status,
              created_at
            )
            values (
              $1, $2, $3, $4, $5, $6, $7, null, $8, null, $9, $10, $11, 'pending', $11
            )
            "#,
        )
        .bind(raw_event_id)
        .bind(run_internal_id)
        .bind(&input.source_system)
        .bind(&input.source_object_type)
        .bind(&input.source_object_id)
        .bind(&input.source_event_key)
        .bind(&input.source_version)
        .bind(source_updated_at)
        .bind(payload_text)
        .bind(payload_hash)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(RawIngestionAcceptedRecord {
            request_id: input.source_event_key.clone(),
            accepted: true,
            run_id: run_external_id.to_string(),
            status: "accepted".to_string(),
            message: "payload accepted for asynchronous processing".to_string(),
            source_system: input.source_system.clone(),
        })
    }

    async fn ensure_push_ingestion_job(&self) -> Result<Uuid, RawIngestionRepositoryError> {
        if let Some(job_id) = sqlx::query_scalar::<_, Uuid>(
            "select integration_job_id from integration_job where job_code = 'push_ingestion'",
        )
        .fetch_optional(&self.pool)
        .await?
        {
            return Ok(job_id);
        }

        let job_id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(
            r#"
            insert into integration_job (
              integration_job_id,
              integration_system_id,
              integration_endpoint_id,
              job_code,
              job_name,
              job_type,
              schedule_expression,
              job_status,
              created_at,
              updated_at
            )
            values ($1, null, null, 'push_ingestion', 'Push Ingestion', 'push', null, 'active', $2, $2)
            on conflict (job_code) do nothing
            "#,
        )
        .bind(job_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        sqlx::query_scalar::<_, Uuid>(
            "select integration_job_id from integration_job where job_code = 'push_ingestion'",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}

fn parse_optional_timestamp(
    value: Option<&str>,
) -> Result<Option<DateTime<Utc>>, RawIngestionRepositoryError> {
    value
        .map(|raw| {
            DateTime::parse_from_rfc3339(raw)
                .map(|parsed| parsed.with_timezone(&Utc))
                .map_err(|_| RawIngestionRepositoryError::InvalidSourceUpdatedAt(raw.to_string()))
        })
        .transpose()
}

fn hash_payload(payload_text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload_text.as_bytes());
    format!("{:x}", hasher.finalize())
}
