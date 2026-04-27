use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::{HashMap, HashSet};
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

        let mut org_codes = HashSet::new();
        let mut emp_numbers = HashSet::new();

        let mut parsed_data = Vec::with_capacity(rows.len());

        for row in &rows {
            let payload_text: String = row.get("payload_reference");
            let payload = serde_json::from_str::<serde_json::Value>(&payload_text)
                .unwrap_or_else(|_| serde_json::json!({}));

            let owning_organization_code = payload
                .get("owning_organization_code")
                .or_else(|| payload.get("organization_code"))
                .and_then(|value| value.as_str())
                .map(ToString::to_string);

            let project_owner_employee_number = payload
                .get("project_owner_employee_number")
                .or_else(|| payload.get("project_owner_id"))
                .and_then(|value| value.as_str())
                .map(ToString::to_string);

            if let Some(code) = &owning_organization_code {
                org_codes.insert(code.clone());
            }
            if let Some(num) = &project_owner_employee_number {
                emp_numbers.insert(num.clone());
            }

            parsed_data.push((
                payload,
                owning_organization_code,
                project_owner_employee_number,
            ));
        }

        let mut org_map: HashMap<String, Uuid> = HashMap::new();
        if !org_codes.is_empty() {
            let codes: Vec<String> = org_codes.into_iter().collect();
            let org_rows = sqlx::query("select organization_code, organization_id from organization_master where organization_code = ANY($1)")
                .bind(&codes)
                .fetch_all(&self.pool)
                .await?;
            for r in org_rows {
                org_map.insert(r.get("organization_code"), r.get("organization_id"));
            }
        }

        let mut emp_map: HashMap<String, Uuid> = HashMap::new();
        if !emp_numbers.is_empty() {
            let nums: Vec<String> = emp_numbers.into_iter().collect();
            let emp_rows = sqlx::query("select employee_number, workforce_id from workforce_master where employee_number = ANY($1)")
                .bind(&nums)
                .fetch_all(&self.pool)
                .await?;
            for r in emp_rows {
                emp_map.insert(r.get("employee_number"), r.get("workforce_id"));
            }
        }

        for (i, row) in rows.iter().enumerate() {
            let project_id: Uuid = row.get("target_entity_id");
            let project_code: String = row.get("source_object_id");
            let (payload, owning_organization_code, project_owner_employee_number) =
                &parsed_data[i];

            let project_name = payload
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or(&project_code);

            let description = payload
                .get("description")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);

            let owning_organization_id = owning_organization_code
                .as_ref()
                .and_then(|code| org_map.get(code).copied())
                .unwrap_or(DEFAULT_ORGANIZATION_ID);

            let project_owner_workforce_id = project_owner_employee_number
                .as_ref()
                .and_then(|num| emp_map.get(num).copied());

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
                  project_owner_workforce_id,
                  description,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, 'delivery', 'active', $4, $5, $6, $7, $7)
                on conflict (project_code)
                do update set
                  owning_organization_id = excluded.owning_organization_id,
                  project_owner_workforce_id = excluded.project_owner_workforce_id,
                  project_name = excluded.project_name,
                  description = excluded.description,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(project_id)
            .bind(&project_code)
            .bind(project_name)
            .bind(owning_organization_id)
            .bind(project_owner_workforce_id)
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
