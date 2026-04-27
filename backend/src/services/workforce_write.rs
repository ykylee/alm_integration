use chrono::Utc;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::collections::{HashMap, HashSet};
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

        let mut identity_keys_to_fetch = HashSet::new();
        let mut parsed_rows = Vec::with_capacity(rows.len());

        for row in rows {
            let workforce_id: Uuid = row.get("target_entity_id");
            let source_system: String = row.get("source_system");
            let employee_number: String = row.get("source_object_id");
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            if let Some(primary_organization_code) = payload
                .get("primary_organization_code")
                .or_else(|| payload.get("organization_code"))
                .and_then(|value| value.as_str())
            {
                identity_keys_to_fetch.insert((
                    source_system.clone(),
                    format!("organization:{primary_organization_code}"),
                ));
            }

            parsed_rows.push((
                workforce_id,
                source_system,
                employee_number,
                payload,
            ));
        }

        let mut identity_mappings: HashMap<(String, String), Uuid> = HashMap::new();
        if !identity_keys_to_fetch.is_empty() {
            let mut source_systems = Vec::with_capacity(identity_keys_to_fetch.len());
            let mut source_identity_keys = Vec::with_capacity(identity_keys_to_fetch.len());
            for (sys, key) in identity_keys_to_fetch {
                source_systems.push(sys);
                source_identity_keys.push(key);
            }

            let mapping_rows = sqlx::query(
                r#"
                select distinct on (source_system_code, source_identity_key)
                  source_system_code,
                  source_identity_key,
                  internal_entity_id
                from identity_mapping
                where internal_entity_type = 'organization'
                  and mapping_status in ('active', 'verified')
                  and (source_system_code, source_identity_key) in (
                    select * from unnest($1::text[], $2::text[])
                  )
                order by source_system_code, source_identity_key, updated_at desc
                "#,
            )
            .bind(&source_systems)
            .bind(&source_identity_keys)
            .fetch_all(&self.pool)
            .await?;

            for m_row in mapping_rows {
                let sys: String = m_row.get("source_system_code");
                let key: String = m_row.get("source_identity_key");
                let entity_id: Uuid = m_row.get("internal_entity_id");
                identity_mappings.insert((sys, key), entity_id);
            }
        }

        for (workforce_id, source_system, employee_number, payload) in parsed_rows {
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

            let identity_key = format!("organization:{primary_organization_code}");
            let primary_organization_id = identity_mappings
                .get(&(source_system.clone(), identity_key))
                .copied();

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
