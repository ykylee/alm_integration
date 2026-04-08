use chrono::Utc;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationWriteResult {
    pub processed_count: i32,
    pub written_count: i32,
    pub skipped_count: i32,
}

#[derive(Debug)]
pub enum OrganizationWriteError {
    Database(sqlx::Error),
}

impl Display for OrganizationWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for OrganizationWriteError {}

impl From<sqlx::Error> for OrganizationWriteError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct OrganizationWriteService {
    pool: PgPool,
}

impl OrganizationWriteService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply_for_run(
        &self,
        integration_run_id: Uuid,
    ) -> Result<OrganizationWriteResult, OrganizationWriteError> {
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
              and nrr.target_entity_type = 'organization'
            "#,
        )
        .bind(integration_run_id)
        .fetch_all(&self.pool)
        .await?;

        let processed_count = rows.len() as i32;
        let mut written_count = 0;
        let mut skipped_count = 0;

        for row in &rows {
            let organization_id: Uuid = row.get("target_entity_id");
            let organization_code: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            let organization_name = payload
                .get("organization_name")
                .or_else(|| payload.get("name"))
                .and_then(|value| value.as_str())
                .unwrap_or(&organization_code);
            let organization_status = payload
                .get("organization_status")
                .or_else(|| payload.get("status"))
                .and_then(|value| value.as_str())
                .unwrap_or("active");
            let parent_organization_code = payload
                .get("parent_organization_code")
                .or_else(|| payload.get("parent_code"))
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let parent_organization_id =
                if let Some(parent_code) = parent_organization_code.as_ref() {
                    sqlx::query_scalar::<_, Uuid>(
                    "select organization_id from organization_master where organization_code = $1",
                )
                .bind(parent_code)
                .fetch_optional(&self.pool)
                .await?
                } else {
                    None
                };

            if parent_organization_code.is_some() && parent_organization_id.is_none() {
                skipped_count += 1;
                continue;
            }

            let effective_from = parse_optional_timestamp(
                payload
                    .get("effective_from")
                    .and_then(|value| value.as_str()),
            );
            let effective_to = parse_optional_timestamp(
                payload.get("effective_to").and_then(|value| value.as_str()),
            );
            let now = Utc::now();

            sqlx::query(
                r#"
                insert into organization_master (
                  organization_id,
                  organization_code,
                  organization_name,
                  parent_organization_id,
                  organization_status,
                  effective_from,
                  effective_to,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, $4, $5, $6, $7, $8, $8)
                on conflict (organization_code)
                do update set
                  organization_name = excluded.organization_name,
                  parent_organization_id = excluded.parent_organization_id,
                  organization_status = excluded.organization_status,
                  effective_from = excluded.effective_from,
                  effective_to = excluded.effective_to,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(organization_id)
            .bind(&organization_code)
            .bind(organization_name)
            .bind(parent_organization_id)
            .bind(organization_status)
            .bind(effective_from)
            .bind(effective_to)
            .bind(now)
            .execute(&self.pool)
            .await?;

            written_count += 1;
        }

        Ok(OrganizationWriteResult {
            processed_count,
            written_count,
            skipped_count,
        })
    }
}

fn parse_optional_timestamp(value: Option<&str>) -> Option<chrono::DateTime<Utc>> {
    value.and_then(|value| {
        chrono::DateTime::parse_from_rfc3339(value)
            .ok()
            .map(|parsed| parsed.with_timezone(&Utc))
    })
}
