use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

const DEFAULT_WORK_ITEM_TYPE_ID: Uuid = Uuid::from_u128(0x00000000000000000000000000000101);

#[derive(Debug, Clone, Serialize)]
pub struct WorkItemWriteResult {
    pub processed_count: i32,
    pub written_count: i32,
    pub skipped_count: i32,
}

#[derive(Debug)]
pub enum WorkItemWriteError {
    Database(sqlx::Error),
}

impl Display for WorkItemWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for WorkItemWriteError {}

impl From<sqlx::Error> for WorkItemWriteError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct WorkItemWriteService {
    pool: PgPool,
}

impl WorkItemWriteService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply_for_run(
        &self,
        integration_run_id: Uuid,
    ) -> Result<WorkItemWriteResult, WorkItemWriteError> {
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
              and nrr.target_entity_type = 'work_item'
            "#,
        )
        .bind(integration_run_id)
        .fetch_all(&self.pool)
        .await?;

        let processed_count = rows.len() as i32;
        let mut written_count = 0;
        let mut skipped_count = 0;

        for row in &rows {
            let work_item_id: Uuid = row.get("target_entity_id");
            let source_system: String = row.get("source_system");
            let work_item_key: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            let Some(project_key) = payload
                .get("project_key")
                .and_then(|value| value.as_str())
                .or_else(|| infer_project_key(&work_item_key))
            else {
                skipped_count += 1;
                continue;
            };

            let project_id = sqlx::query_scalar::<_, Uuid>(
                r#"
                select internal_entity_id
                from identity_mapping
                where source_system_code = $1
                  and source_identity_key = $2
                  and internal_entity_type = 'project'
                  and mapping_status in ('active', 'verified')
                order by updated_at desc
                limit 1
                "#,
            )
            .bind(&source_system)
            .bind(format!("project:{project_key}"))
            .fetch_optional(&self.pool)
            .await?;

            let Some(project_id) = project_id else {
                skipped_count += 1;
                continue;
            };

            let work_item_type_id = resolve_work_item_type_id(&self.pool, &payload).await?;
            let title = payload
                .get("summary")
                .and_then(|value| value.as_str())
                .unwrap_or(&work_item_key);
            let description = payload
                .get("description")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let now = Utc::now();

            sqlx::query(
                r#"
                insert into work_item (
                  work_item_id,
                  project_id,
                  work_item_type_id,
                  work_item_key,
                  title,
                  description,
                  current_common_status,
                  current_detailed_status_code,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, $4, $5, $6, 'open', 'new', $7, $7)
                on conflict (project_id, work_item_key)
                do update set
                  title = excluded.title,
                  description = excluded.description,
                  work_item_type_id = excluded.work_item_type_id,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(work_item_id)
            .bind(project_id)
            .bind(work_item_type_id)
            .bind(&work_item_key)
            .bind(title)
            .bind(description)
            .bind(now)
            .execute(&self.pool)
            .await?;

            written_count += 1;
        }

        Ok(WorkItemWriteResult {
            processed_count,
            written_count,
            skipped_count,
        })
    }
}

async fn resolve_work_item_type_id(
    pool: &PgPool,
    payload: &serde_json::Value,
) -> Result<Uuid, WorkItemWriteError> {
    let Some(type_code) = payload
        .get("issue_type")
        .and_then(|value| value.as_str())
        .map(|value| value.to_lowercase())
    else {
        return Ok(DEFAULT_WORK_ITEM_TYPE_ID);
    };

    let work_item_type_id = sqlx::query_scalar::<_, Uuid>(
        "select work_item_type_id from work_item_type where type_code = $1 and is_active = true",
    )
    .bind(&type_code)
    .fetch_optional(pool)
    .await?;

    Ok(work_item_type_id.unwrap_or(DEFAULT_WORK_ITEM_TYPE_ID))
}

fn infer_project_key(work_item_key: &str) -> Option<&str> {
    work_item_key
        .split_once('-')
        .map(|(project_key, _)| project_key)
}
