use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::services::identity_mapping::{
    IdentityMappingService, IdentityMappingServiceError, UpsertIdentityMappingInput,
};

#[derive(Debug, Clone, Serialize)]
pub struct NormalizationResult {
    pub processed_count: i32,
    pub normalized_count: i32,
    pub skipped_count: i32,
}

#[derive(Debug)]
pub enum NormalizationPipelineError {
    Database(sqlx::Error),
    IdentityMapping(IdentityMappingServiceError),
}

impl Display for NormalizationPipelineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
            Self::IdentityMapping(error) => write!(f, "identity mapping error: {error}"),
        }
    }
}

impl std::error::Error for NormalizationPipelineError {}

impl From<sqlx::Error> for NormalizationPipelineError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<IdentityMappingServiceError> for NormalizationPipelineError {
    fn from(error: IdentityMappingServiceError) -> Self {
        Self::IdentityMapping(error)
    }
}

pub struct NormalizationPipeline {
    pool: PgPool,
    identity_mapping_service: IdentityMappingService,
}

impl NormalizationPipeline {
    pub fn new(pool: PgPool) -> Self {
        Self {
            identity_mapping_service: IdentityMappingService::new(pool.clone()),
            pool,
        }
    }

    #[allow(dead_code)]
    pub async fn normalize_pending(
        &self,
        limit: i64,
    ) -> Result<NormalizationResult, NormalizationPipelineError> {
        let rows = sqlx::query(
            r#"
            select
              raw_ingestion_event_id,
              source_system,
              source_object_type,
              source_object_id,
              payload_reference,
              normalization_status
            from raw_ingestion_event
            where normalization_status = 'pending'
            order by ingested_at asc
            limit $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        self.normalize_rows(rows).await
    }

    pub async fn normalize_pending_for_run(
        &self,
        run_id: &str,
        limit: i64,
    ) -> Result<NormalizationResult, NormalizationPipelineError> {
        let rows = sqlx::query(
            r#"
            select
              rie.raw_ingestion_event_id,
              rie.source_system,
              rie.source_object_type,
              rie.source_object_id,
              rie.payload_reference,
              rie.normalization_status
            from raw_ingestion_event rie
            join integration_run ir on ir.integration_run_id = rie.integration_run_id
            where ir.external_run_id = $1
              and rie.normalization_status = 'pending'
            order by rie.ingested_at asc
            limit $2
            "#,
        )
        .bind(run_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        self.normalize_rows(rows).await
    }

    async fn normalize_rows(
        &self,
        rows: Vec<sqlx::postgres::PgRow>,
    ) -> Result<NormalizationResult, NormalizationPipelineError> {
        let mut normalized_count = 0;

        for row in rows {
            let raw_ingestion_event_id: Uuid = row.get("raw_ingestion_event_id");
            let source_system: String = row.get("source_system");
            let source_object_type: String = row.get("source_object_type");
            let source_object_id: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            let normalized = normalize_record(
                raw_ingestion_event_id,
                &source_system,
                &source_object_type,
                &source_object_id,
                payload,
            );

            let now = Utc::now();
            let mut tx = self.pool.begin().await?;

            sqlx::query(
                r#"
                insert into normalized_record_reference (
                  normalized_record_reference_id,
                  raw_ingestion_event_id,
                  target_entity_type,
                  target_entity_id,
                  normalization_version,
                  normalized_at,
                  created_at
                )
                values ($1, $2, $3, $4, $5, $6, $6)
                on conflict (raw_ingestion_event_id, target_entity_type, target_entity_id)
                do update set
                  normalization_version = excluded.normalization_version,
                  normalized_at = excluded.normalized_at
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(normalized.raw_ingestion_event_id)
            .bind(&normalized.target_entity_type)
            .bind(normalized.target_entity_id)
            .bind(&normalized.normalization_version)
            .bind(now)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                r#"
                update raw_ingestion_event
                set normalization_status = 'normalized'
                where raw_ingestion_event_id = $1
                "#,
            )
            .bind(normalized.raw_ingestion_event_id)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;

            self.identity_mapping_service
                .upsert(UpsertIdentityMappingInput {
                    source_system_code: source_system.clone(),
                    source_identity_key: format!("{source_object_type}:{source_object_id}"),
                    internal_entity_type: normalized.target_entity_type.clone(),
                    internal_entity_id: normalized.target_entity_id,
                    mapping_status: "active".to_string(),
                    verified_at: None,
                })
                .await?;

            normalized_count += 1;
        }

        Ok(NormalizationResult {
            processed_count: normalized_count,
            normalized_count,
            skipped_count: 0,
        })
    }
}

struct NormalizedRecord {
    raw_ingestion_event_id: Uuid,
    target_entity_type: String,
    target_entity_id: Uuid,
    normalization_version: String,
}

fn normalize_record(
    raw_ingestion_event_id: Uuid,
    source_system: &str,
    source_object_type: &str,
    source_object_id: &str,
    _payload: serde_json::Value,
) -> NormalizedRecord {
    let target_entity_type = map_target_entity_type(source_object_type).to_string();
    let target_entity_id = Uuid::new_v5(
        &Uuid::NAMESPACE_URL,
        format!("{source_system}:{source_object_type}:{source_object_id}").as_bytes(),
    );

    NormalizedRecord {
        raw_ingestion_event_id,
        target_entity_type,
        target_entity_id,
        normalization_version: "v1".to_string(),
    }
}

fn map_target_entity_type(source_object_type: &str) -> &str {
    match source_object_type {
        "issue" => "work_item",
        "project" => "project",
        "organization" => "organization",
        "employee" | "workforce" => "workforce",
        other => other,
    }
}
