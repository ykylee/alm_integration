use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpsertIdentityMappingInput {
    pub source_system_code: String,
    pub source_identity_key: String,
    pub internal_entity_type: String,
    pub internal_entity_id: Uuid,
    pub mapping_status: String,
    pub verified_at: Option<String>,
}

#[derive(Debug)]
pub enum IdentityMappingServiceError {
    InvalidVerifiedAt(String),
    Database(sqlx::Error),
}

impl Display for IdentityMappingServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVerifiedAt(value) => write!(f, "invalid verified_at: {value}"),
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for IdentityMappingServiceError {}

impl From<sqlx::Error> for IdentityMappingServiceError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct IdentityMappingService {
    pool: PgPool,
}

impl IdentityMappingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(
        &self,
        input: UpsertIdentityMappingInput,
    ) -> Result<(), IdentityMappingServiceError> {
        let verified_at = parse_optional_timestamp(input.verified_at.as_deref())?;
        let now = Utc::now();

        sqlx::query(
            r#"
            insert into identity_mapping (
              identity_mapping_id,
              source_system_code,
              source_identity_key,
              internal_entity_type,
              internal_entity_id,
              mapping_status,
              verified_at,
              created_at,
              updated_at
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            on conflict (source_system_code, source_identity_key, internal_entity_type)
            do update set
              internal_entity_id = excluded.internal_entity_id,
              mapping_status = excluded.mapping_status,
              verified_at = excluded.verified_at,
              updated_at = excluded.updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&input.source_system_code)
        .bind(&input.source_identity_key)
        .bind(&input.internal_entity_type)
        .bind(input.internal_entity_id)
        .bind(&input.mapping_status)
        .bind(verified_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

fn parse_optional_timestamp(
    value: Option<&str>,
) -> Result<Option<DateTime<Utc>>, IdentityMappingServiceError> {
    match value {
        Some(raw) => {
            let parsed = DateTime::parse_from_rfc3339(raw)
                .map_err(|_| IdentityMappingServiceError::InvalidVerifiedAt(raw.to_string()))?;
            Ok(Some(parsed.with_timezone(&Utc)))
        }
        None => Ok(None),
    }
}
