use chrono::Utc;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct WorkforceWriteResult {
    pub processed_count: i32,
    pub written_count: i32,
    pub skipped_count: i32,
}

#[derive(Debug)]
pub enum WorkforceWriteError {
    Database(sqlx::Error),
}

impl Display for WorkforceWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for WorkforceWriteError {}

impl From<sqlx::Error> for WorkforceWriteError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct WorkforceWriteService {
    pool: PgPool,
}

impl WorkforceWriteService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply_for_run(
        &self,
        integration_run_id: Uuid,
    ) -> Result<WorkforceWriteResult, WorkforceWriteError> {
        let rows = sqlx::query(
            r#"
            select
              nrr.target_entity_id,
              rie.source_system,
              rie.source_object_id,
              rie.payload_reference
            from normalized_record_reference nrr
            join raw_ingestion_event rie on rie.raw_ingestion_event_id = nrr.raw_ingestion_event_id
            where rie.integration_run_id = $1
              and rie.normalization_status = 'normalized'
              and nrr.target_entity_type = 'workforce'
            "#,
        )
        .bind(integration_run_id)
        .fetch_all(&self.pool)
        .await?;

        let processed_count = rows.len() as i32;
        let mut written_count = 0;
        let mut skipped_count = 0;

        for row in &rows {
            let workforce_id: Uuid = row.get("target_entity_id");
            let source_system: String = row.get("source_system");
            let employee_number: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            let display_name = payload
                .get("display_name")
                .or_else(|| payload.get("name"))
                .and_then(|value| value.as_str())
                .unwrap_or(&employee_number);
            let employment_status = payload
                .get("employment_status")
                .and_then(|value| value.as_str())
                .unwrap_or("active");
            let primary_organization_code = payload
                .get("primary_organization_code")
                .or_else(|| payload.get("organization_code"))
                .and_then(|value| value.as_str());
            let Some(primary_organization_code) = primary_organization_code else {
                skipped_count += 1;
                continue;
            };

            let primary_organization_id = sqlx::query_scalar::<_, Uuid>(
                r#"
                select internal_entity_id
                from identity_mapping
                where source_system_code = $1
                  and source_identity_key = $2
                  and internal_entity_type = 'organization'
                  and mapping_status in ('active', 'verified')
                order by updated_at desc
                limit 1
                "#,
            )
            .bind(&source_system)
            .bind(format!("organization:{primary_organization_code}"))
            .fetch_optional(&self.pool)
            .await?;

            let Some(primary_organization_id) = primary_organization_id else {
                skipped_count += 1;
                continue;
            };

            let job_family = payload
                .get("job_family")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let email = payload
                .get("email")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let now = Utc::now();

            sqlx::query(
                r#"
                insert into workforce_master (
                  workforce_id,
                  employee_number,
                  display_name,
                  employment_status,
                  primary_organization_id,
                  job_family,
                  email,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, $4, $5, $6, $7, $8, $8)
                on conflict (employee_number)
                do update set
                  display_name = excluded.display_name,
                  employment_status = excluded.employment_status,
                  primary_organization_id = excluded.primary_organization_id,
                  job_family = excluded.job_family,
                  email = excluded.email,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(workforce_id)
            .bind(&employee_number)
            .bind(display_name)
            .bind(employment_status)
            .bind(primary_organization_id)
            .bind(job_family)
            .bind(email)
            .bind(now)
            .execute(&self.pool)
            .await?;

            written_count += 1;
        }

        Ok(WorkforceWriteResult {
            processed_count,
            written_count,
            skipped_count,
        })
    }
}
