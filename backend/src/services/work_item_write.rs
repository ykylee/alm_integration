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

struct PendingHierarchyLink {
    project_id: Uuid,
    parent_key: String,
    child_work_item_id: Uuid,
}

struct PendingPlanLink {
    project_id: Uuid,
    work_item_id: Uuid,
    iteration_name: String,
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
        let mut pending_hierarchy_links = Vec::new();
        let mut pending_plan_links = Vec::new();

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
            let (current_common_status, current_detailed_status_code) =
                resolve_status_fields(&payload);
            let owning_organization_id = resolve_organization_id(
                &self.pool,
                payload
                    .get("owning_organization_code")
                    .or_else(|| payload.get("organization_code"))
                    .and_then(|value| value.as_str()),
            )
            .await?;
            let assignee_workforce_id = resolve_workforce_id(
                &self.pool,
                payload
                    .get("assignee_employee_number")
                    .or_else(|| payload.get("assignee_id"))
                    .and_then(|value| value.as_str()),
            )
            .await?;
            let reporter_workforce_id = resolve_workforce_id(
                &self.pool,
                payload
                    .get("reporter_employee_number")
                    .or_else(|| payload.get("reporter_id"))
                    .and_then(|value| value.as_str()),
            )
            .await?;
            let title = payload
                .get("summary")
                .and_then(|value| value.as_str())
                .unwrap_or(&work_item_key);
            let description = payload
                .get("description")
                .and_then(|value| value.as_str())
                .map(ToString::to_string);
            let now = Utc::now();
            let previous_status = sqlx::query(
                r#"
                select current_common_status, current_detailed_status_code
                from work_item
                where project_id = $1
                  and work_item_key = $2
                "#,
            )
            .bind(project_id)
            .bind(&work_item_key)
            .fetch_optional(&self.pool)
            .await?;

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
                  owning_organization_id,
                  assignee_workforce_id,
                  reporter_workforce_id,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12)
                on conflict (project_id, work_item_key)
                do update set
                  title = excluded.title,
                  description = excluded.description,
                  work_item_type_id = excluded.work_item_type_id,
                  current_common_status = excluded.current_common_status,
                  current_detailed_status_code = excluded.current_detailed_status_code,
                  owning_organization_id = excluded.owning_organization_id,
                  assignee_workforce_id = excluded.assignee_workforce_id,
                  reporter_workforce_id = excluded.reporter_workforce_id,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(work_item_id)
            .bind(project_id)
            .bind(work_item_type_id)
            .bind(&work_item_key)
            .bind(title)
            .bind(description)
            .bind(&current_common_status)
            .bind(&current_detailed_status_code)
            .bind(owning_organization_id)
            .bind(assignee_workforce_id)
            .bind(reporter_workforce_id)
            .bind(now)
            .execute(&self.pool)
            .await?;

            let should_insert_history = previous_status
                .as_ref()
                .map(|row| {
                    let previous_common_status: String = row.get("current_common_status");
                    let previous_detailed_status_code: String =
                        row.get("current_detailed_status_code");

                    previous_common_status != current_common_status
                        || previous_detailed_status_code != current_detailed_status_code
                })
                .unwrap_or(true);

            if should_insert_history {
                let from_common_status = previous_status
                    .as_ref()
                    .map(|row| row.get::<String, _>("current_common_status"));
                let from_detailed_status_code = previous_status
                    .as_ref()
                    .map(|row| row.get::<String, _>("current_detailed_status_code"));

                sqlx::query(
                    r#"
                    insert into work_item_status_history (
                      work_item_status_history_id,
                      work_item_id,
                      from_common_status,
                      from_detailed_status_code,
                      to_common_status,
                      to_detailed_status_code,
                      workflow_transition_definition_id,
                      changed_by,
                      changed_at,
                      change_reason,
                      source_type
                    )
                    values ($1, $2, $3, $4, $5, $6, null, 'integration_pipeline', $7, null, 'integration')
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(work_item_id)
                .bind(from_common_status)
                .bind(from_detailed_status_code)
                .bind(&current_common_status)
                .bind(&current_detailed_status_code)
                .bind(now)
                .execute(&self.pool)
                .await?;
            }

            if let Some(parent_key) = payload.get("parent_key").and_then(|value| value.as_str()) {
                pending_hierarchy_links.push(PendingHierarchyLink {
                    project_id,
                    parent_key: parent_key.to_string(),
                    child_work_item_id: work_item_id,
                });
            }

            if let Some(iteration_name) = payload
                .get("iteration_name")
                .and_then(|value| value.as_str())
            {
                pending_plan_links.push(PendingPlanLink {
                    project_id,
                    work_item_id,
                    iteration_name: iteration_name.to_string(),
                });
            }

            written_count += 1;
        }

        for link in pending_hierarchy_links {
            let parent_work_item_id = sqlx::query_scalar::<_, Uuid>(
                r#"
                select work_item_id
                from work_item
                where project_id = $1
                  and work_item_key = $2
                "#,
            )
            .bind(link.project_id)
            .bind(&link.parent_key)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(parent_work_item_id) = parent_work_item_id {
                sqlx::query(
                    r#"
                    insert into work_item_hierarchy (
                      work_item_hierarchy_id,
                      parent_work_item_id,
                      child_work_item_id,
                      relationship_type,
                      sequence_no,
                      created_at
                    )
                    values ($1, $2, $3, 'parent_child', null, $4)
                    on conflict (child_work_item_id)
                    do update set
                      parent_work_item_id = excluded.parent_work_item_id,
                      relationship_type = excluded.relationship_type
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(parent_work_item_id)
                .bind(link.child_work_item_id)
                .bind(Utc::now())
                .execute(&self.pool)
                .await?;
            }
        }

        for link in pending_plan_links {
            let iteration_id = sqlx::query_scalar::<_, Uuid>(
                r#"
                insert into iteration (
                  iteration_id,
                  project_id,
                  name,
                  goal,
                  status,
                  start_date,
                  end_date,
                  capacity,
                  sequence_no,
                  created_at,
                  updated_at
                )
                values ($1, $2, $3, null, 'planned', null, null, null, null, $4, $4)
                on conflict (project_id, name)
                do update set
                  updated_at = excluded.updated_at
                returning iteration_id
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(link.project_id)
            .bind(&link.iteration_name)
            .bind(Utc::now())
            .fetch_one(&self.pool)
            .await?;

            sqlx::query(
                r#"
                insert into work_item_plan_link (
                  work_item_plan_link_id,
                  work_item_id,
                  plan_type,
                  plan_id,
                  link_role,
                  sequence_no,
                  is_primary,
                  linked_by_rule_ref,
                  effective_from,
                  effective_to,
                  created_at,
                  updated_at
                )
                values ($1, $2, 'iteration', $3, 'planned', null, true, 'integration_payload', null, null, $4, $4)
                on conflict (work_item_id, plan_type, plan_id, link_role)
                do update set
                  is_primary = excluded.is_primary,
                  linked_by_rule_ref = excluded.linked_by_rule_ref,
                  updated_at = excluded.updated_at
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(link.work_item_id)
            .bind(iteration_id)
            .bind(Utc::now())
            .execute(&self.pool)
            .await?;
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

async fn resolve_organization_id(
    pool: &PgPool,
    organization_code: Option<&str>,
) -> Result<Option<Uuid>, sqlx::Error> {
    if let Some(organization_code) = organization_code {
        sqlx::query_scalar::<_, Uuid>(
            "select organization_id from organization_master where organization_code = $1",
        )
        .bind(organization_code)
        .fetch_optional(pool)
        .await
    } else {
        Ok(None)
    }
}

async fn resolve_workforce_id(
    pool: &PgPool,
    employee_number: Option<&str>,
) -> Result<Option<Uuid>, sqlx::Error> {
    if let Some(employee_number) = employee_number {
        sqlx::query_scalar::<_, Uuid>(
            "select workforce_id from workforce_master where employee_number = $1",
        )
        .bind(employee_number)
        .fetch_optional(pool)
        .await
    } else {
        Ok(None)
    }
}

fn resolve_status_fields(payload: &serde_json::Value) -> (String, String) {
    let common_status = payload
        .get("status")
        .and_then(|status| status.get("common"))
        .and_then(|value| value.as_str())
        .unwrap_or("open")
        .to_string();
    let detailed_status_code = payload
        .get("status")
        .and_then(|status| status.get("detailed"))
        .and_then(|value| value.as_str())
        .unwrap_or("new")
        .to_string();

    (common_status, detailed_status_code)
}
