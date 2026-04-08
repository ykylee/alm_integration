use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone, Serialize)]
pub struct ProjectSummaryRecord {
    pub project_code: String,
    pub project_name: String,
    pub project_status: String,
    pub owning_organization_code: Option<String>,
    pub owning_organization_name: Option<String>,
    pub project_owner_employee_number: Option<String>,
    pub project_owner_display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkItemSummaryRecord {
    pub project_code: String,
    pub work_item_key: String,
    pub title: String,
    pub current_common_status: String,
    pub current_detailed_status_code: String,
    pub owning_organization_code: Option<String>,
    pub assignee_employee_number: Option<String>,
    pub assignee_display_name: Option<String>,
    pub reporter_employee_number: Option<String>,
    pub reporter_display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProjectListFilter {
    pub project_status: Option<String>,
    pub project_code: Option<String>,
    pub owning_organization_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkItemListFilter {
    pub project_code: Option<String>,
    pub current_common_status: Option<String>,
    pub assignee_employee_number: Option<String>,
    pub owning_organization_code: Option<String>,
}

#[derive(Debug)]
pub enum DomainReadRepositoryError {
    Database(sqlx::Error),
}

impl std::fmt::Display for DomainReadRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for DomainReadRepositoryError {}

impl From<sqlx::Error> for DomainReadRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

pub struct DomainReadRepository {
    pool: PgPool,
}

impl DomainReadRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_projects(
        &self,
        filter: &ProjectListFilter,
    ) -> Result<Vec<ProjectSummaryRecord>, DomainReadRepositoryError> {
        let rows = sqlx::query(
            r#"
            select
              p.project_code,
              p.project_name,
              p.project_status,
              om.organization_code as owning_organization_code,
              om.organization_name as owning_organization_name,
              wm.employee_number as project_owner_employee_number,
              wm.display_name as project_owner_display_name
            from project p
            left join organization_master om on om.organization_id = p.owning_organization_id
            left join workforce_master wm on wm.workforce_id = p.project_owner_workforce_id
            where ($1::varchar is null or p.project_status = $1)
              and ($2::varchar is null or p.project_code = $2)
              and ($3::varchar is null or om.organization_code = $3)
            order by p.project_code
            "#,
        )
        .bind(filter.project_status.as_deref())
        .bind(filter.project_code.as_deref())
        .bind(filter.owning_organization_code.as_deref())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| ProjectSummaryRecord {
                project_code: row.get("project_code"),
                project_name: row.get("project_name"),
                project_status: row.get("project_status"),
                owning_organization_code: row.get("owning_organization_code"),
                owning_organization_name: row.get("owning_organization_name"),
                project_owner_employee_number: row.get("project_owner_employee_number"),
                project_owner_display_name: row.get("project_owner_display_name"),
            })
            .collect())
    }

    pub async fn list_work_items(
        &self,
        filter: &WorkItemListFilter,
    ) -> Result<Vec<WorkItemSummaryRecord>, DomainReadRepositoryError> {
        let rows = sqlx::query(
            r#"
            select
              p.project_code,
              wi.work_item_key,
              wi.title,
              wi.current_common_status,
              wi.current_detailed_status_code,
              org.organization_code as owning_organization_code,
              assignee.employee_number as assignee_employee_number,
              assignee.display_name as assignee_display_name,
              reporter.employee_number as reporter_employee_number,
              reporter.display_name as reporter_display_name
            from work_item wi
            join project p on p.project_id = wi.project_id
            left join organization_master org on org.organization_id = wi.owning_organization_id
            left join workforce_master assignee on assignee.workforce_id = wi.assignee_workforce_id
            left join workforce_master reporter on reporter.workforce_id = wi.reporter_workforce_id
            where ($1::varchar is null or p.project_code = $1)
              and ($2::varchar is null or wi.current_common_status = $2)
              and ($3::varchar is null or assignee.employee_number = $3)
              and ($4::varchar is null or org.organization_code = $4)
            order by p.project_code, wi.work_item_key
            "#,
        )
        .bind(filter.project_code.as_deref())
        .bind(filter.current_common_status.as_deref())
        .bind(filter.assignee_employee_number.as_deref())
        .bind(filter.owning_organization_code.as_deref())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| WorkItemSummaryRecord {
                project_code: row.get("project_code"),
                work_item_key: row.get("work_item_key"),
                title: row.get("title"),
                current_common_status: row.get("current_common_status"),
                current_detailed_status_code: row.get("current_detailed_status_code"),
                owning_organization_code: row.get("owning_organization_code"),
                assignee_employee_number: row.get("assignee_employee_number"),
                assignee_display_name: row.get("assignee_display_name"),
                reporter_employee_number: row.get("reporter_employee_number"),
                reporter_display_name: row.get("reporter_display_name"),
            })
            .collect())
    }
}
