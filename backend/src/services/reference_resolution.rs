use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceResolutionResult {
    pub processed_count: i32,
    pub resolved_count: i32,
    pub pending_count: i32,
}

#[derive(Debug)]
pub enum ReferenceResolutionError {
    Database(sqlx::Error),
}

impl Display for ReferenceResolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for ReferenceResolutionError {}

impl From<sqlx::Error> for ReferenceResolutionError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct ReferenceResolutionService {
    pool: PgPool,
}

impl ReferenceResolutionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn resolve_pending_references(
        &self,
        limit: i64,
    ) -> Result<ReferenceResolutionResult, ReferenceResolutionError> {
        let rows = sqlx::query(
            r#"
            select
              raw_ingestion_event_id,
              source_system,
              payload_reference
            from raw_ingestion_event
            where normalization_status = 'pending_reference'
            order by ingested_at asc
            limit $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        self.resolve_rows(rows).await
    }

    pub async fn resolve_pending_references_for_run(
        &self,
        run_id: &str,
        limit: i64,
    ) -> Result<ReferenceResolutionResult, ReferenceResolutionError> {
        let rows = sqlx::query(
            r#"
            select
              rie.raw_ingestion_event_id,
              rie.source_system,
              rie.payload_reference
            from raw_ingestion_event rie
            join integration_run ir on ir.integration_run_id = rie.integration_run_id
            where ir.external_run_id = $1
              and rie.normalization_status = 'pending_reference'
            order by rie.ingested_at asc
            limit $2
            "#,
        )
        .bind(run_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        self.resolve_rows(rows).await
    }

    async fn resolve_rows(
        &self,
        rows: Vec<sqlx::postgres::PgRow>,
    ) -> Result<ReferenceResolutionResult, ReferenceResolutionError> {
        let mut resolved_count = 0;

        for row in &rows {
            let raw_ingestion_event_id: Uuid = row.get("raw_ingestion_event_id");
            let source_system: String = row.get("source_system");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));
            let missing_references = extract_missing_references(&payload);

            if missing_references.is_empty() {
                continue;
            }

            let mut all_resolved = true;
            for reference_code in &missing_references {
                let exists = sqlx::query_scalar::<_, bool>(
                    r#"
                    select exists(
                      select 1
                      from identity_mapping
                      where source_system_code = $1
                        and source_identity_key = $2
                        and mapping_status in ('active', 'verified')
                    )
                    "#,
                )
                .bind(&source_system)
                .bind(reference_code)
                .fetch_one(&self.pool)
                .await?;

                if !exists {
                    all_resolved = false;
                    break;
                }
            }

            if all_resolved {
                sqlx::query(
                    r#"
                    update raw_ingestion_event
                    set normalization_status = 'pending'
                    where raw_ingestion_event_id = $1
                    "#,
                )
                .bind(raw_ingestion_event_id)
                .execute(&self.pool)
                .await?;

                resolved_count += 1;
            }
        }

        let processed_count = rows.len() as i32;
        Ok(ReferenceResolutionResult {
            processed_count,
            resolved_count,
            pending_count: processed_count - resolved_count,
        })
    }
}

fn extract_missing_references(payload: &serde_json::Value) -> Vec<String> {
    payload
        .get("references")
        .and_then(|references| references.get("missing"))
        .and_then(|missing| missing.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(ToString::to_string))
                .collect()
        })
        .unwrap_or_default()
}
