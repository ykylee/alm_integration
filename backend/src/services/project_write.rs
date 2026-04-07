use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

const DEFAULT_ORGANIZATION_ID: Uuid = Uuid::from_u128(0x00000000000000000000000000000001);

#[derive(Debug, Clone, Serialize)]
pub struct ProjectWriteResult {
    pub processed_count: i32,
    pub written_count: i32,
}

#[derive(Debug)]
pub enum ProjectWriteError {
    Database(sqlx::Error),
}

impl Display for ProjectWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for ProjectWriteError {}

impl From<sqlx::Error> for ProjectWriteError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct ProjectWriteService {
    pool: PgPool,
}

impl ProjectWriteService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply_for_run(
        &self,
        integration_run_id: Uuid,
    ) -> Result<ProjectWriteResult, ProjectWriteError> {
        let rows = sqlx::query(
            r#"
            select
              nrr.target_entity_id,
              rie.source_object_id,
              rie.payload_reference
            from normalized_record_reference nrr
            join raw_ingestion_event rie on rie.raw_ingestion_event_id = nrr.raw_ingestion_event_id
            where rie.integration_run_id = $1
              and rie.normalization_status = 'normalized'
              and nrr.target_entity_type = 'project'
            "#,
        )
        .bind(integration_run_id)
        .fetch_all(&self.pool)
        .await?;

        let processed_count = rows.len() as i32;

        for row in &rows {
            let project_id: Uuid = row.get("target_entity_id");
            let project_code: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));
            let project_name = payload
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or(&project_code);
            let description = payload
                .get("description")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let now = Utc::now();

            sqlx::query(
                r#"
                insert into project (
                  project_id,
                  project_code,
                  project_name,
                  project_type,
                  project_status,
                  owning_organization_id,
                  description,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, 'delivery', 'active', $4, $5, $6, $6)
                on conflict (project_code)
                do update set
                  project_name = excluded.project_name,
                  description = excluded.description,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(project_id)
            .bind(&project_code)
            .bind(project_name)
            .bind(DEFAULT_ORGANIZATION_ID)
            .bind(description)
            .bind(now)
            .execute(&self.pool)
            .await?;
        }

        Ok(ProjectWriteResult {
            processed_count,
            written_count: processed_count,
        })
    }
}
