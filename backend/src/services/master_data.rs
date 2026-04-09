use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationRecord {
    pub organization_id: String,
    pub organization_code: String,
    pub organization_name: String,
    pub parent_organization_code: Option<String>,
    pub organization_status: String,
    pub effective_from: Option<String>,
    pub effective_to: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkforceRecord {
    pub workforce_id: String,
    pub employee_number: String,
    pub display_name: String,
    pub employment_status: String,
    pub primary_organization_code: Option<String>,
    pub primary_organization_name: Option<String>,
    pub job_family: Option<String>,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationChangeLogRecord {
    pub log_id: String,
    pub organization_code: String,
    pub action_type: String,
    pub summary: String,
    pub changed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkforceChangeLogRecord {
    pub log_id: String,
    pub employee_number: String,
    pub action_type: String,
    pub from_organization_code: Option<String>,
    pub to_organization_code: Option<String>,
    pub summary: String,
    pub changed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationStructureNodeRecord {
    pub organization_code: String,
    pub organization_name: String,
    pub organization_status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationStructureSnapshotRecord {
    pub organization_code: String,
    pub organization_name: String,
    pub parent_organization_code: Option<String>,
    pub organization_status: String,
    pub ancestors: Vec<OrganizationStructureNodeRecord>,
    pub children: Vec<OrganizationStructureNodeRecord>,
    pub direct_member_count: usize,
    pub subtree_organization_count: usize,
    pub subtree_active_member_count: usize,
}

#[derive(Debug, Clone)]
pub struct UpsertOrganizationInput {
    pub organization_code: String,
    pub organization_name: String,
    pub parent_organization_code: Option<String>,
    pub organization_status: String,
    pub effective_from: Option<String>,
    pub effective_to: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpsertWorkforceInput {
    pub employee_number: String,
    pub display_name: String,
    pub employment_status: String,
    pub primary_organization_code: Option<String>,
    pub job_family: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateOrganizationInput {
    pub organization_name: Option<String>,
    pub parent_organization_code: Option<Option<String>>,
    pub organization_status: Option<String>,
    pub effective_from: Option<Option<String>>,
    pub effective_to: Option<Option<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateWorkforceInput {
    pub display_name: Option<String>,
    pub employment_status: Option<String>,
    pub primary_organization_code: Option<String>,
    pub job_family: Option<Option<String>>,
    pub email: Option<Option<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrganizationListFilter {
    pub organization_status: Option<String>,
    pub organization_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkforceListFilter {
    pub employment_status: Option<String>,
    pub primary_organization_code: Option<String>,
}

#[derive(Debug)]
pub enum MasterDataStoreError {
    InvalidTimestamp,
    InvalidReference,
    NotFound,
}

#[derive(Debug)]
pub enum MasterDataRepositoryError {
    InvalidTimestamp(String),
    InvalidReference(String),
    NotFound(String),
    Database(sqlx::Error),
}

impl Display for MasterDataRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTimestamp(value) => write!(f, "invalid timestamp: {value}"),
            Self::InvalidReference(value) => write!(f, "invalid reference: {value}"),
            Self::NotFound(value) => write!(f, "not found: {value}"),
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for MasterDataRepositoryError {}

impl From<sqlx::Error> for MasterDataRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Default)]
pub struct MasterDataStore {
    organizations: Vec<OrganizationRecord>,
    workforce: Vec<WorkforceRecord>,
    organization_logs: Vec<OrganizationChangeLogRecord>,
    workforce_logs: Vec<WorkforceChangeLogRecord>,
}

impl MasterDataStore {
    pub fn list_organizations(&self, filter: &OrganizationListFilter) -> Vec<OrganizationRecord> {
        let mut items: Vec<OrganizationRecord> = self
            .organizations
            .iter()
            .filter(|record| match_organization_filter(record, filter))
            .cloned()
            .collect();
        items.sort_by(|left, right| left.organization_code.cmp(&right.organization_code));
        items
    }

    pub fn list_organization_history(
        &self,
        organization_code: &str,
    ) -> Vec<OrganizationChangeLogRecord> {
        let mut items: Vec<OrganizationChangeLogRecord> = self
            .organization_logs
            .iter()
            .filter(|record| record.organization_code == organization_code)
            .cloned()
            .collect();
        items.sort_by(|left, right| right.changed_at.cmp(&left.changed_at));
        items
    }

    pub fn get_organization_structure(
        &self,
        organization_code: &str,
    ) -> Result<OrganizationStructureSnapshotRecord, MasterDataStoreError> {
        build_organization_structure_snapshot(
            &self.organizations,
            &self.workforce,
            organization_code,
        )
        .ok_or(MasterDataStoreError::NotFound)
    }

    pub fn upsert_organization(
        &mut self,
        input: UpsertOrganizationInput,
    ) -> Result<OrganizationRecord, MasterDataStoreError> {
        let effective_from = parse_optional_timestamp_store(&input.effective_from)?;
        let effective_to = parse_optional_timestamp_store(&input.effective_to)?;
        let parent_organization_code = input.parent_organization_code.clone();

        if parent_organization_code.as_deref() == Some(input.organization_code.as_str()) {
            return Err(MasterDataStoreError::InvalidReference);
        }

        if let Some(parent_code) = parent_organization_code.as_ref() {
            let parent_exists = self
                .organizations
                .iter()
                .any(|record| record.organization_code == *parent_code);
            if !parent_exists {
                return Err(MasterDataStoreError::InvalidReference);
            }
        }

        let now = Utc::now().to_rfc3339();
        let record = if let Some(existing) = self
            .organizations
            .iter_mut()
            .find(|record| record.organization_code == input.organization_code)
        {
            existing.organization_name = input.organization_name;
            existing.parent_organization_code = parent_organization_code;
            existing.organization_status = input.organization_status;
            existing.effective_from = effective_from;
            existing.effective_to = effective_to;
            existing.updated_at = now.clone();
            existing.clone()
        } else {
            let record = OrganizationRecord {
                organization_id: Uuid::new_v4().to_string(),
                organization_code: input.organization_code,
                organization_name: input.organization_name,
                parent_organization_code,
                organization_status: input.organization_status,
                effective_from,
                effective_to,
                created_at: now.clone(),
                updated_at: now,
            };
            self.organizations.push(record.clone());
            record
        };

        self.sync_workforce_organization_names();
        self.push_organization_log(
            &record.organization_code,
            "upsert",
            format!("조직 {} 저장", record.organization_name),
        );
        Ok(record)
    }

    pub fn update_organization(
        &mut self,
        organization_code: &str,
        input: UpdateOrganizationInput,
    ) -> Result<OrganizationRecord, MasterDataStoreError> {
        let index = self
            .organizations
            .iter()
            .position(|record| record.organization_code == organization_code)
            .ok_or(MasterDataStoreError::NotFound)?;

        let current = self.organizations[index].clone();
        let next_parent = input
            .parent_organization_code
            .unwrap_or(current.parent_organization_code.clone());

        if next_parent.as_deref() == Some(organization_code) {
            return Err(MasterDataStoreError::InvalidReference);
        }

        if let Some(parent_code) = next_parent.as_ref() {
            let parent_exists = self
                .organizations
                .iter()
                .any(|record| record.organization_code == *parent_code);
            if !parent_exists || self.store_has_hierarchy_cycle(organization_code, parent_code) {
                return Err(MasterDataStoreError::InvalidReference);
            }
        }

        let effective_from = match input.effective_from {
            Some(value) => parse_optional_timestamp_store(&value)?,
            None => current.effective_from.clone(),
        };
        let effective_to = match input.effective_to {
            Some(value) => parse_optional_timestamp_store(&value)?,
            None => current.effective_to.clone(),
        };

        {
            let existing = &mut self.organizations[index];
            if let Some(name) = input.organization_name {
                existing.organization_name = name;
            }
            existing.parent_organization_code = next_parent;
            if let Some(status) = input.organization_status {
                existing.organization_status = status;
            }
            existing.effective_from = effective_from;
            existing.effective_to = effective_to;
            existing.updated_at = Utc::now().to_rfc3339();
        }

        self.sync_workforce_organization_names();
        let updated = self.organizations[index].clone();
        self.push_organization_log(
            &updated.organization_code,
            "update",
            format!("조직 {} 수정", updated.organization_name),
        );
        Ok(updated)
    }

    pub fn soft_delete_organization(
        &mut self,
        organization_code: &str,
    ) -> Result<OrganizationRecord, MasterDataStoreError> {
        let index = self
            .organizations
            .iter()
            .position(|record| record.organization_code == organization_code)
            .ok_or(MasterDataStoreError::NotFound)?;

        let has_child = self.organizations.iter().any(|record| {
            record.parent_organization_code.as_deref() == Some(organization_code)
                && record.organization_status != "deleted"
        });
        if has_child {
            return Err(MasterDataStoreError::InvalidReference);
        }

        let affected_members: Vec<(String, String)> = self
            .workforce
            .iter()
            .filter(|record| record.primary_organization_code.as_deref() == Some(organization_code))
            .map(|record| (record.employee_number.clone(), record.display_name.clone()))
            .collect();

        for workforce in self
            .workforce
            .iter_mut()
            .filter(|record| record.primary_organization_code.as_deref() == Some(organization_code))
        {
            workforce.primary_organization_code = None;
            workforce.primary_organization_name = None;
            workforce.updated_at = Utc::now().to_rfc3339();
        }

        let existing = &mut self.organizations[index];
        existing.organization_status = "deleted".to_string();
        existing.effective_to = Some(Utc::now().to_rfc3339());
        existing.updated_at = Utc::now().to_rfc3339();
        let deleted = existing.clone();
        self.push_organization_log(
            &deleted.organization_code,
            "delete",
            format!("조직 {} 삭제", deleted.organization_name),
        );
        for (employee_number, display_name) in affected_members {
            self.push_workforce_log(
                &employee_number,
                "unassign",
                Some(organization_code.to_string()),
                None,
                format!(
                    "조직 {} 삭제로 구성원 {} 이(가) 미배정 상태로 전환",
                    deleted.organization_name, display_name
                ),
            );
        }
        Ok(deleted)
    }

    pub fn list_workforce(&self, filter: &WorkforceListFilter) -> Vec<WorkforceRecord> {
        let mut items: Vec<WorkforceRecord> = self
            .workforce
            .iter()
            .filter(|record| match_workforce_filter(record, filter))
            .cloned()
            .collect();
        items.sort_by(|left, right| left.employee_number.cmp(&right.employee_number));
        items
    }

    pub fn list_organization_member_history(
        &self,
        organization_code: &str,
    ) -> Vec<WorkforceChangeLogRecord> {
        let mut items: Vec<WorkforceChangeLogRecord> = self
            .workforce_logs
            .iter()
            .filter(|record| {
                record.from_organization_code.as_deref() == Some(organization_code)
                    || record.to_organization_code.as_deref() == Some(organization_code)
            })
            .cloned()
            .collect();
        items.sort_by(|left, right| right.changed_at.cmp(&left.changed_at));
        items
    }

    pub fn upsert_workforce(
        &mut self,
        input: UpsertWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataStoreError> {
        let existed = self
            .workforce
            .iter()
            .any(|record| record.employee_number == input.employee_number);
        let organization = self
            .organizations
            .iter()
            .find(|record| Some(record.organization_code.clone()) == input.primary_organization_code)
            .cloned();

        let now = Utc::now().to_rfc3339();
        let record = if let Some(existing) = self
            .workforce
            .iter_mut()
            .find(|record| record.employee_number == input.employee_number)
        {
            existing.display_name = input.display_name;
            existing.employment_status = input.employment_status;
            existing.primary_organization_code =
                organization.as_ref().map(|item| item.organization_code.clone());
            existing.primary_organization_name =
                organization.as_ref().map(|item| item.organization_name.clone());
            existing.job_family = input.job_family;
            existing.email = input.email;
            existing.updated_at = now.clone();
            existing.clone()
        } else {
            let record = WorkforceRecord {
                workforce_id: Uuid::new_v4().to_string(),
                employee_number: input.employee_number,
                display_name: input.display_name,
                employment_status: input.employment_status,
                primary_organization_code: organization.as_ref().map(|item| item.organization_code.clone()),
                primary_organization_name: organization.as_ref().map(|item| item.organization_name.clone()),
                job_family: input.job_family,
                email: input.email,
                created_at: now.clone(),
                updated_at: now,
            };
            self.workforce.push(record.clone());
            record
        };

        self.push_workforce_log(
            &record.employee_number,
            if existed { "update" } else { "create" },
            None,
            record.primary_organization_code.clone(),
            format!("구성원 {} 저장", record.display_name),
        );
        Ok(record)
    }

    pub fn update_workforce(
        &mut self,
        employee_number: &str,
        input: UpdateWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataStoreError> {
        let index = self
            .workforce
            .iter()
            .position(|record| record.employee_number == employee_number)
            .ok_or(MasterDataStoreError::NotFound)?;

        let previous_organization_code = self.workforce[index].primary_organization_code.clone();
        let next_org_code = input
            .primary_organization_code
            .clone()
            .or_else(|| self.workforce[index].primary_organization_code.clone());
        let organization = next_org_code.as_ref().and_then(|organization_code| {
            self.organizations
                .iter()
                .find(|record| record.organization_code == *organization_code)
                .cloned()
        });
        if next_org_code.is_some() && organization.is_none() {
            return Err(MasterDataStoreError::InvalidReference);
        }

        let existing = &mut self.workforce[index];
        if let Some(display_name) = input.display_name {
            existing.display_name = display_name;
        }
        if let Some(status) = input.employment_status {
            existing.employment_status = status;
        }
        existing.primary_organization_code = organization.as_ref().map(|item| item.organization_code.clone());
        existing.primary_organization_name = organization.as_ref().map(|item| item.organization_name.clone());
        if let Some(job_family) = input.job_family {
            existing.job_family = job_family;
        }
        if let Some(email) = input.email {
            existing.email = email;
        }
        existing.updated_at = Utc::now().to_rfc3339();

        let updated = existing.clone();
        self.push_workforce_log(
            &updated.employee_number,
            "update",
            previous_organization_code,
            updated.primary_organization_code.clone(),
            format!("구성원 {} 수정", updated.display_name),
        );
        Ok(updated)
    }

    pub fn soft_delete_workforce(
        &mut self,
        employee_number: &str,
    ) -> Result<WorkforceRecord, MasterDataStoreError> {
        let index = self
            .workforce
            .iter()
            .position(|record| record.employee_number == employee_number)
            .ok_or(MasterDataStoreError::NotFound)?;

        let existing = &mut self.workforce[index];
        existing.employment_status = "inactive".to_string();
        existing.updated_at = Utc::now().to_rfc3339();
        let deleted = existing.clone();
        self.push_workforce_log(
            &deleted.employee_number,
            "deactivate",
            deleted.primary_organization_code.clone(),
            deleted.primary_organization_code.clone(),
            format!("구성원 {} 비활성화", deleted.display_name),
        );
        Ok(deleted)
    }

    fn sync_workforce_organization_names(&mut self) {
        for workforce in &mut self.workforce {
            if let Some(organization) = self
                .organizations
                .iter()
                .find(|record| Some(record.organization_code.clone()) == workforce.primary_organization_code)
            {
                workforce.primary_organization_name = Some(organization.organization_name.clone());
            } else {
                workforce.primary_organization_name = None;
            }
        }
    }

    fn push_organization_log(
        &mut self,
        organization_code: &str,
        action_type: &str,
        summary: String,
    ) {
        self.organization_logs.push(OrganizationChangeLogRecord {
            log_id: Uuid::new_v4().to_string(),
            organization_code: organization_code.to_string(),
            action_type: action_type.to_string(),
            summary,
            changed_at: Utc::now().to_rfc3339(),
        });
    }

    fn push_workforce_log(
        &mut self,
        employee_number: &str,
        action_type: &str,
        from_organization_code: Option<String>,
        to_organization_code: Option<String>,
        summary: String,
    ) {
        self.workforce_logs.push(WorkforceChangeLogRecord {
            log_id: Uuid::new_v4().to_string(),
            employee_number: employee_number.to_string(),
            action_type: action_type.to_string(),
            from_organization_code,
            to_organization_code,
            summary,
            changed_at: Utc::now().to_rfc3339(),
        });
    }

    fn store_has_hierarchy_cycle(&self, organization_code: &str, parent_code: &str) -> bool {
        let mut cursor = Some(parent_code.to_string());
        while let Some(current_code) = cursor {
            if current_code == organization_code {
                return true;
            }
            cursor = self
                .organizations
                .iter()
                .find(|record| record.organization_code == current_code)
                .and_then(|record| record.parent_organization_code.clone());
        }
        false
    }
}

pub struct MasterDataRepository {
    pool: PgPool,
}

impl MasterDataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_organizations(
        &self,
        filter: &OrganizationListFilter,
    ) -> Result<Vec<OrganizationRecord>, MasterDataRepositoryError> {
        let rows = sqlx::query(
            r#"
            select
              om.organization_id,
              om.organization_code,
              om.organization_name,
              parent.organization_code as parent_organization_code,
              om.organization_status,
              om.effective_from,
              om.effective_to,
              om.created_at,
              om.updated_at
            from organization_master om
            left join organization_master parent
              on parent.organization_id = om.parent_organization_id
            where ($1::varchar is null or om.organization_status = $1)
              and ($2::varchar is null or om.organization_code = $2)
            order by om.organization_code
            "#,
        )
        .bind(filter.organization_status.as_deref())
        .bind(filter.organization_code.as_deref())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(map_organization_row).collect())
    }

    pub async fn list_organization_history(
        &self,
        organization_code: &str,
    ) -> Result<Vec<OrganizationChangeLogRecord>, MasterDataRepositoryError> {
        let rows = sqlx::query(
            r#"
            select log_id, organization_code, action_type, summary, changed_at
            from organization_change_log
            where organization_code = $1
            order by changed_at desc
            "#,
        )
        .bind(organization_code)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(map_organization_log_row).collect())
    }

    pub async fn get_organization_structure(
        &self,
        organization_code: &str,
    ) -> Result<OrganizationStructureSnapshotRecord, MasterDataRepositoryError> {
        let organizations = self
            .list_organizations(&OrganizationListFilter::default())
            .await?;
        let workforce = self
            .list_workforce(&WorkforceListFilter {
                employment_status: Some("active".to_string()),
                primary_organization_code: None,
            })
            .await?;

        build_organization_structure_snapshot(&organizations, &workforce, organization_code)
            .ok_or_else(|| {
                MasterDataRepositoryError::NotFound(format!(
                    "organization not found: {organization_code}"
                ))
            })
    }

    pub async fn upsert_organization(
        &self,
        input: UpsertOrganizationInput,
    ) -> Result<OrganizationRecord, MasterDataRepositoryError> {
        let effective_from = parse_optional_timestamp_repository(&input.effective_from)?;
        let effective_to = parse_optional_timestamp_repository(&input.effective_to)?;

        if input.parent_organization_code.as_deref() == Some(input.organization_code.as_str()) {
            return Err(MasterDataRepositoryError::InvalidReference(
                "parent organization cannot reference itself".to_string(),
            ));
        }

        let mut transaction = self.pool.begin().await?;
        let parent_organization_id: Option<Uuid> =
            if let Some(parent_code) = input.parent_organization_code.as_ref() {
                Some(
                sqlx::query_scalar::<_, Uuid>(
                    "select organization_id from organization_master where organization_code = $1",
                )
                .bind(parent_code)
                .fetch_optional(&mut *transaction)
                .await?
                .ok_or_else(|| {
                    MasterDataRepositoryError::InvalidReference(format!(
                        "organization not found: {parent_code}"
                    ))
                })?,
            )
            } else {
                None
            };

        let existing_organization_id = sqlx::query_scalar::<_, Uuid>(
            "select organization_id from organization_master where organization_code = $1",
        )
        .bind(&input.organization_code)
        .fetch_optional(&mut *transaction)
        .await?;

        let organization_id = if let Some(organization_id) = existing_organization_id {
            sqlx::query(
                r#"
                update organization_master
                set organization_name = $2,
                    parent_organization_id = $3,
                    organization_status = $4,
                    effective_from = $5,
                    effective_to = $6,
                    updated_at = $7
                where organization_id = $1
                "#,
            )
            .bind(organization_id)
            .bind(&input.organization_name)
            .bind(parent_organization_id)
            .bind(&input.organization_status)
            .bind(effective_from)
            .bind(effective_to)
            .bind(Utc::now())
            .execute(&mut *transaction)
            .await?;
            organization_id
        } else {
            let organization_id = Uuid::new_v4();
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
                "#,
            )
            .bind(organization_id)
            .bind(&input.organization_code)
            .bind(&input.organization_name)
            .bind(parent_organization_id)
            .bind(&input.organization_status)
            .bind(effective_from)
            .bind(effective_to)
            .bind(now)
            .execute(&mut *transaction)
            .await?;
            organization_id
        };

        let row = sqlx::query(
            r#"
            select
              om.organization_id,
              om.organization_code,
              om.organization_name,
              parent.organization_code as parent_organization_code,
              om.organization_status,
              om.effective_from,
              om.effective_to,
              om.created_at,
              om.updated_at
            from organization_master om
            left join organization_master parent
              on parent.organization_id = om.parent_organization_id
            where om.organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&mut *transaction)
        .await?;

        insert_organization_log(
            &mut transaction,
            &input.organization_code,
            "upsert",
            &format!("조직 {} 저장", input.organization_name),
        )
        .await?;

        transaction.commit().await?;
        Ok(map_organization_row(&row))
    }

    pub async fn update_organization(
        &self,
        organization_code: &str,
        input: UpdateOrganizationInput,
    ) -> Result<OrganizationRecord, MasterDataRepositoryError> {
        let UpdateOrganizationInput {
            organization_name,
            parent_organization_code,
            organization_status,
            effective_from,
            effective_to,
        } = input;
        let mut transaction = self.pool.begin().await?;
        let current_row = sqlx::query(
            r#"
            select organization_id, organization_name, organization_status, effective_from, effective_to
            from organization_master
            where organization_code = $1
            "#,
        )
        .bind(organization_code)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or_else(|| MasterDataRepositoryError::NotFound(format!("organization not found: {organization_code}")))?;

        let organization_id: Uuid = current_row.get("organization_id");
        let current_name: String = current_row.get("organization_name");
        let current_status: String = current_row.get("organization_status");
        let current_effective_from: Option<DateTime<Utc>> = current_row.get("effective_from");
        let current_effective_to: Option<DateTime<Utc>> = current_row.get("effective_to");

        let next_parent_code = match parent_organization_code {
            Some(value) => value,
            None => sqlx::query_scalar::<_, Option<String>>(
                r#"
                select parent.organization_code
                from organization_master om
                left join organization_master parent on parent.organization_id = om.parent_organization_id
                where om.organization_code = $1
                "#,
            )
            .bind(organization_code)
            .fetch_one(&mut *transaction)
            .await?,
        };

        if next_parent_code.as_deref() == Some(organization_code) {
            return Err(MasterDataRepositoryError::InvalidReference(
                "parent organization cannot reference itself".to_string(),
            ));
        }

        let parent_organization_id = if let Some(parent_code) = next_parent_code.as_ref() {
            let parent_id = sqlx::query_scalar::<_, Uuid>(
                "select organization_id from organization_master where organization_code = $1",
            )
            .bind(parent_code)
            .fetch_optional(&mut *transaction)
            .await?
            .ok_or_else(|| {
                MasterDataRepositoryError::InvalidReference(format!(
                    "organization not found: {parent_code}"
                ))
            })?;

            if self
                .repository_has_hierarchy_cycle(&mut transaction, organization_code, parent_code)
                .await?
            {
                return Err(MasterDataRepositoryError::InvalidReference(
                    "organization hierarchy cycle detected".to_string(),
                ));
            }

            Some(parent_id)
        } else {
            None
        };

        let effective_from = match effective_from {
            Some(value) => parse_optional_timestamp_repository(&value)?,
            None => current_effective_from,
        };
        let effective_to = match effective_to {
            Some(value) => parse_optional_timestamp_repository(&value)?,
            None => current_effective_to,
        };
        let next_name = organization_name.unwrap_or(current_name);
        let next_status = organization_status.unwrap_or(current_status);

        sqlx::query(
            r#"
            update organization_master
            set organization_name = $2,
                parent_organization_id = $3,
                organization_status = $4,
                effective_from = $5,
                effective_to = $6,
                updated_at = $7
            where organization_id = $1
            "#,
        )
        .bind(organization_id)
        .bind(&next_name)
        .bind(parent_organization_id)
        .bind(&next_status)
        .bind(effective_from)
        .bind(effective_to)
        .bind(Utc::now())
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query(
            r#"
            select
              om.organization_id,
              om.organization_code,
              om.organization_name,
              parent.organization_code as parent_organization_code,
              om.organization_status,
              om.effective_from,
              om.effective_to,
              om.created_at,
              om.updated_at
            from organization_master om
            left join organization_master parent
              on parent.organization_id = om.parent_organization_id
            where om.organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&mut *transaction)
        .await?;

        insert_organization_log(
            &mut transaction,
            organization_code,
            "update",
            &format!("조직 {} 수정", next_name),
        )
        .await?;

        transaction.commit().await?;
        Ok(map_organization_row(&row))
    }

    pub async fn soft_delete_organization(
        &self,
        organization_code: &str,
    ) -> Result<OrganizationRecord, MasterDataRepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let organization_id = sqlx::query_scalar::<_, Uuid>(
            "select organization_id from organization_master where organization_code = $1",
        )
        .bind(organization_code)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or_else(|| {
            MasterDataRepositoryError::NotFound(format!(
                "organization not found: {organization_code}"
            ))
        })?;

        let child_count: i64 = sqlx::query_scalar(
            r#"
            select count(*)
            from organization_master
            where parent_organization_id = $1
              and organization_status <> 'deleted'
            "#,
        )
        .bind(organization_id)
        .fetch_one(&mut *transaction)
        .await?;

        if child_count > 0 {
            return Err(MasterDataRepositoryError::InvalidReference(
                "organization has active children".to_string(),
            ));
        }

        let affected_members = sqlx::query(
            r#"
            select employee_number, display_name
            from workforce_master
            where primary_organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_all(&mut *transaction)
        .await?;

        sqlx::query(
            r#"
            update workforce_master
            set primary_organization_id = null,
                updated_at = $2
            where primary_organization_id = $1
            "#,
        )
        .bind(organization_id)
        .bind(Utc::now())
        .execute(&mut *transaction)
        .await?;

        sqlx::query(
            r#"
            update organization_master
            set organization_status = 'deleted',
                effective_to = coalesce(effective_to, $2),
                updated_at = $2
            where organization_id = $1
            "#,
        )
        .bind(organization_id)
        .bind(Utc::now())
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query(
            r#"
            select
              om.organization_id,
              om.organization_code,
              om.organization_name,
              parent.organization_code as parent_organization_code,
              om.organization_status,
              om.effective_from,
              om.effective_to,
              om.created_at,
              om.updated_at
            from organization_master om
            left join organization_master parent
              on parent.organization_id = om.parent_organization_id
            where om.organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&mut *transaction)
        .await?;

        insert_organization_log(
            &mut transaction,
            organization_code,
            "delete",
            &format!("조직 {} 삭제", organization_code),
        )
        .await?;
        for member in affected_members {
            let employee_number: String = member.get("employee_number");
            let display_name: String = member.get("display_name");
            insert_workforce_log(
                &mut transaction,
                &employee_number,
                "unassign",
                Some(organization_code.to_string()),
                None,
                &format!(
                    "조직 {} 삭제로 구성원 {} 이(가) 미배정 상태로 전환",
                    organization_code, display_name
                ),
            )
            .await?;
        }

        transaction.commit().await?;
        Ok(map_organization_row(&row))
    }

    pub async fn list_workforce(
        &self,
        filter: &WorkforceListFilter,
    ) -> Result<Vec<WorkforceRecord>, MasterDataRepositoryError> {
        let rows = sqlx::query(
            r#"
            select
              wm.workforce_id,
              wm.employee_number,
              wm.display_name,
              wm.employment_status,
              om.organization_code as primary_organization_code,
              om.organization_name as primary_organization_name,
              wm.job_family,
              wm.email,
              wm.created_at,
              wm.updated_at
            from workforce_master wm
            left join organization_master om
              on om.organization_id = wm.primary_organization_id
            where ($1::varchar is null or wm.employment_status = $1)
              and ($2::varchar is null or om.organization_code = $2)
            order by wm.employee_number
            "#,
        )
        .bind(filter.employment_status.as_deref())
        .bind(filter.primary_organization_code.as_deref())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(map_workforce_row).collect())
    }

    pub async fn list_organization_member_history(
        &self,
        organization_code: &str,
    ) -> Result<Vec<WorkforceChangeLogRecord>, MasterDataRepositoryError> {
        let rows = sqlx::query(
            r#"
            select
              log_id,
              employee_number,
              action_type,
              from_organization_code,
              to_organization_code,
              summary,
              changed_at
            from workforce_change_log
            where from_organization_code = $1
               or to_organization_code = $1
            order by changed_at desc
            "#,
        )
        .bind(organization_code)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(map_workforce_log_row).collect())
    }

    pub async fn upsert_workforce(
        &self,
        input: UpsertWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataRepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let primary_organization_row = if let Some(primary_organization_code) =
            input.primary_organization_code.as_ref()
        {
            Some(
                sqlx::query(
                    r#"
                    select organization_id, organization_code, organization_name
                    from organization_master
                    where organization_code = $1
                    "#,
                )
                .bind(primary_organization_code)
                .fetch_optional(&mut *transaction)
                .await?
                .ok_or_else(|| {
                    MasterDataRepositoryError::InvalidReference(format!(
                        "organization not found: {primary_organization_code}"
                    ))
                })?,
            )
        } else {
            None
        };

        let primary_organization_id: Option<Uuid> = primary_organization_row
            .as_ref()
            .map(|row| row.get("organization_id"));

        let existing_workforce_id = sqlx::query_scalar::<_, Uuid>(
            "select workforce_id from workforce_master where employee_number = $1",
        )
        .bind(&input.employee_number)
        .fetch_optional(&mut *transaction)
        .await?;

        let workforce_id = if let Some(workforce_id) = existing_workforce_id {
            sqlx::query(
                r#"
                update workforce_master
                set display_name = $2,
                    employment_status = $3,
                    primary_organization_id = $4,
                    job_family = $5,
                    email = $6,
                    updated_at = $7
                where workforce_id = $1
                "#,
            )
            .bind(workforce_id)
            .bind(&input.display_name)
            .bind(&input.employment_status)
            .bind(primary_organization_id)
            .bind(&input.job_family)
            .bind(&input.email)
            .bind(Utc::now())
            .execute(&mut *transaction)
            .await?;
            workforce_id
        } else {
            let workforce_id = Uuid::new_v4();
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
                "#,
            )
            .bind(workforce_id)
            .bind(&input.employee_number)
            .bind(&input.display_name)
            .bind(&input.employment_status)
            .bind(primary_organization_id)
            .bind(&input.job_family)
            .bind(&input.email)
            .bind(now)
            .execute(&mut *transaction)
            .await?;
            workforce_id
        };

        let row = sqlx::query(
            r#"
            select
              wm.workforce_id,
              wm.employee_number,
              wm.display_name,
              wm.employment_status,
              om.organization_code as primary_organization_code,
              om.organization_name as primary_organization_name,
              wm.job_family,
              wm.email,
              wm.created_at,
              wm.updated_at
            from workforce_master wm
            left join organization_master om
              on om.organization_id = wm.primary_organization_id
            where wm.workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .fetch_one(&mut *transaction)
        .await?;

        insert_workforce_log(
            &mut transaction,
            &input.employee_number,
            "upsert",
            None,
            input.primary_organization_code.clone(),
            &format!("구성원 {} 저장", input.display_name),
        )
        .await?;

        transaction.commit().await?;
        Ok(map_workforce_row(&row))
    }

    pub async fn update_workforce(
        &self,
        employee_number: &str,
        input: UpdateWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataRepositoryError> {
        let UpdateWorkforceInput {
            display_name,
            employment_status,
            primary_organization_code,
            job_family,
            email,
        } = input;
        let mut transaction = self.pool.begin().await?;
        let current_row = sqlx::query(
            r#"
            select
              wm.workforce_id,
              wm.display_name,
              wm.employment_status,
              om.organization_code as primary_organization_code,
              wm.job_family,
              wm.email
            from workforce_master wm
            left join organization_master om on om.organization_id = wm.primary_organization_id
            where wm.employee_number = $1
            "#,
        )
        .bind(employee_number)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or_else(|| {
            MasterDataRepositoryError::NotFound(format!("workforce not found: {employee_number}"))
        })?;

        let workforce_id: Uuid = current_row.get("workforce_id");
        let current_display_name: String = current_row.get("display_name");
        let current_employment_status: String = current_row.get("employment_status");
        let current_primary_organization_code: Option<String> =
            current_row.get("primary_organization_code");
        let current_job_family: Option<String> = current_row.get("job_family");
        let current_email: Option<String> = current_row.get("email");

        let next_organization_code =
            primary_organization_code.or_else(|| current_primary_organization_code.clone());
        let primary_organization_id = if let Some(next_organization_code) =
            next_organization_code.as_ref()
        {
            Some(
                sqlx::query_scalar::<_, Uuid>(
                    "select organization_id from organization_master where organization_code = $1",
                )
                .bind(next_organization_code)
                .fetch_optional(&mut *transaction)
                .await?
                .ok_or_else(|| {
                    MasterDataRepositoryError::InvalidReference(format!(
                        "organization not found: {next_organization_code}"
                    ))
                })?,
            )
        } else {
            None
        };

        let next_display_name = display_name.unwrap_or(current_display_name.clone());
        let next_employment_status = employment_status.unwrap_or(current_employment_status);
        let next_job_family = job_family.unwrap_or(current_job_family);
        let next_email = email.unwrap_or(current_email);

        sqlx::query(
            r#"
            update workforce_master
            set display_name = $2,
                employment_status = $3,
                primary_organization_id = $4,
                job_family = $5,
                email = $6,
                updated_at = $7
            where workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .bind(&next_display_name)
        .bind(&next_employment_status)
        .bind(primary_organization_id)
        .bind(&next_job_family)
        .bind(&next_email)
        .bind(Utc::now())
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query(
            r#"
            select
              wm.workforce_id,
              wm.employee_number,
              wm.display_name,
              wm.employment_status,
              om.organization_code as primary_organization_code,
              om.organization_name as primary_organization_name,
              wm.job_family,
              wm.email,
              wm.created_at,
              wm.updated_at
            from workforce_master wm
            left join organization_master om
              on om.organization_id = wm.primary_organization_id
            where wm.workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .fetch_one(&mut *transaction)
        .await?;

        insert_workforce_log(
            &mut transaction,
            employee_number,
            "update",
            current_primary_organization_code,
            next_organization_code,
            &format!("구성원 {} 수정", next_display_name),
        )
        .await?;

        transaction.commit().await?;
        Ok(map_workforce_row(&row))
    }

    pub async fn soft_delete_workforce(
        &self,
        employee_number: &str,
    ) -> Result<WorkforceRecord, MasterDataRepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let workforce_id = sqlx::query_scalar::<_, Uuid>(
            "select workforce_id from workforce_master where employee_number = $1",
        )
        .bind(employee_number)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or_else(|| {
            MasterDataRepositoryError::NotFound(format!("workforce not found: {employee_number}"))
        })?;

        sqlx::query(
            r#"
            update workforce_master
            set employment_status = 'inactive',
                updated_at = $2
            where workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .bind(Utc::now())
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query(
            r#"
            select
              wm.workforce_id,
              wm.employee_number,
              wm.display_name,
              wm.employment_status,
              om.organization_code as primary_organization_code,
              om.organization_name as primary_organization_name,
              wm.job_family,
              wm.email,
              wm.created_at,
              wm.updated_at
            from workforce_master wm
            left join organization_master om
              on om.organization_id = wm.primary_organization_id
            where wm.workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .fetch_one(&mut *transaction)
        .await?;

        let current_row = sqlx::query(
            r#"
            select
              wm.display_name,
              om.organization_code as primary_organization_code
            from workforce_master wm
            left join organization_master om on om.organization_id = wm.primary_organization_id
            where wm.workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .fetch_one(&mut *transaction)
        .await?;
        let display_name: String = current_row.get("display_name");
        let organization_code: Option<String> = current_row.get("primary_organization_code");

        insert_workforce_log(
            &mut transaction,
            employee_number,
            "deactivate",
            organization_code.clone(),
            organization_code,
            &format!("구성원 {} 비활성화", display_name),
        )
        .await?;

        transaction.commit().await?;
        Ok(map_workforce_row(&row))
    }

    async fn repository_has_hierarchy_cycle(
        &self,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        organization_code: &str,
        parent_code: &str,
    ) -> Result<bool, MasterDataRepositoryError> {
        let mut cursor = Some(parent_code.to_string());
        while let Some(current_code) = cursor {
            if current_code == organization_code {
                return Ok(true);
            }
            cursor = sqlx::query_scalar::<_, Option<String>>(
                r#"
                select parent.organization_code
                from organization_master om
                left join organization_master parent on parent.organization_id = om.parent_organization_id
                where om.organization_code = $1
                "#,
            )
            .bind(&current_code)
            .fetch_one(&mut **transaction)
            .await?;
        }
        Ok(false)
    }
}

fn parse_optional_timestamp_store(
    value: &Option<String>,
) -> Result<Option<String>, MasterDataStoreError> {
    value
        .as_ref()
        .map(|value| {
            DateTime::parse_from_rfc3339(value)
                .map(|parsed| parsed.with_timezone(&Utc).to_rfc3339())
                .map_err(|_| MasterDataStoreError::InvalidTimestamp)
        })
        .transpose()
}

fn parse_optional_timestamp_repository(
    value: &Option<String>,
) -> Result<Option<DateTime<Utc>>, MasterDataRepositoryError> {
    value
        .as_ref()
        .map(|value| {
            DateTime::parse_from_rfc3339(value)
                .map(|parsed| parsed.with_timezone(&Utc))
                .map_err(|_| MasterDataRepositoryError::InvalidTimestamp(value.clone()))
        })
        .transpose()
}

fn map_organization_row(row: &sqlx::postgres::PgRow) -> OrganizationRecord {
    let organization_id: Uuid = row.get("organization_id");
    let effective_from: Option<DateTime<Utc>> = row.get("effective_from");
    let effective_to: Option<DateTime<Utc>> = row.get("effective_to");
    let created_at: DateTime<Utc> = row.get("created_at");
    let updated_at: DateTime<Utc> = row.get("updated_at");

    OrganizationRecord {
        organization_id: organization_id.to_string(),
        organization_code: row.get("organization_code"),
        organization_name: row.get("organization_name"),
        parent_organization_code: row.get("parent_organization_code"),
        organization_status: row.get("organization_status"),
        effective_from: effective_from.map(|value| value.to_rfc3339()),
        effective_to: effective_to.map(|value| value.to_rfc3339()),
        created_at: created_at.to_rfc3339(),
        updated_at: updated_at.to_rfc3339(),
    }
}

fn map_workforce_row(row: &sqlx::postgres::PgRow) -> WorkforceRecord {
    let workforce_id: Uuid = row.get("workforce_id");
    let created_at: DateTime<Utc> = row.get("created_at");
    let updated_at: DateTime<Utc> = row.get("updated_at");

    WorkforceRecord {
        workforce_id: workforce_id.to_string(),
        employee_number: row.get("employee_number"),
        display_name: row.get("display_name"),
        employment_status: row.get("employment_status"),
        primary_organization_code: row.get("primary_organization_code"),
        primary_organization_name: row.get("primary_organization_name"),
        job_family: row.get("job_family"),
        email: row.get("email"),
        created_at: created_at.to_rfc3339(),
        updated_at: updated_at.to_rfc3339(),
    }
}

fn map_organization_log_row(row: &sqlx::postgres::PgRow) -> OrganizationChangeLogRecord {
    let log_id: Uuid = row.get("log_id");
    let changed_at: DateTime<Utc> = row.get("changed_at");
    OrganizationChangeLogRecord {
        log_id: log_id.to_string(),
        organization_code: row.get("organization_code"),
        action_type: row.get("action_type"),
        summary: row.get("summary"),
        changed_at: changed_at.to_rfc3339(),
    }
}

fn map_workforce_log_row(row: &sqlx::postgres::PgRow) -> WorkforceChangeLogRecord {
    let log_id: Uuid = row.get("log_id");
    let changed_at: DateTime<Utc> = row.get("changed_at");
    WorkforceChangeLogRecord {
        log_id: log_id.to_string(),
        employee_number: row.get("employee_number"),
        action_type: row.get("action_type"),
        from_organization_code: row.get("from_organization_code"),
        to_organization_code: row.get("to_organization_code"),
        summary: row.get("summary"),
        changed_at: changed_at.to_rfc3339(),
    }
}

async fn insert_organization_log(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    organization_code: &str,
    action_type: &str,
    summary: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into organization_change_log (
          log_id,
          organization_code,
          action_type,
          summary,
          changed_at
        )
        values ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(organization_code)
    .bind(action_type)
    .bind(summary)
    .bind(Utc::now())
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn insert_workforce_log(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    employee_number: &str,
    action_type: &str,
    from_organization_code: Option<String>,
    to_organization_code: Option<String>,
    summary: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into workforce_change_log (
          log_id,
          employee_number,
          action_type,
          from_organization_code,
          to_organization_code,
          summary,
          changed_at
        )
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(employee_number)
    .bind(action_type)
    .bind(from_organization_code)
    .bind(to_organization_code)
    .bind(summary)
    .bind(Utc::now())
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

fn match_organization_filter(record: &OrganizationRecord, filter: &OrganizationListFilter) -> bool {
    filter
        .organization_status
        .as_ref()
        .map_or(true, |value| &record.organization_status == value)
        && filter
            .organization_code
            .as_ref()
            .map_or(true, |value| &record.organization_code == value)
}

fn match_workforce_filter(record: &WorkforceRecord, filter: &WorkforceListFilter) -> bool {
    filter
        .employment_status
        .as_ref()
        .map_or(true, |value| &record.employment_status == value)
        && filter
            .primary_organization_code
            .as_ref()
            .map_or(true, |value| record.primary_organization_code.as_deref() == Some(value.as_str()))
}

fn build_organization_structure_snapshot(
    organizations: &[OrganizationRecord],
    workforce: &[WorkforceRecord],
    organization_code: &str,
) -> Option<OrganizationStructureSnapshotRecord> {
    let selected = organizations
        .iter()
        .find(|record| record.organization_code == organization_code)?
        .clone();

    let mut ancestors = Vec::new();
    let mut cursor = selected.parent_organization_code.clone();
    while let Some(parent_code) = cursor {
        let parent = organizations
            .iter()
            .find(|record| record.organization_code == parent_code)?;
        ancestors.push(OrganizationStructureNodeRecord {
            organization_code: parent.organization_code.clone(),
            organization_name: parent.organization_name.clone(),
            organization_status: parent.organization_status.clone(),
        });
        cursor = parent.parent_organization_code.clone();
    }
    ancestors.reverse();

    let children: Vec<OrganizationStructureNodeRecord> = organizations
        .iter()
        .filter(|record| {
            record.parent_organization_code.as_deref() == Some(organization_code)
                && record.organization_status != "deleted"
        })
        .map(|record| OrganizationStructureNodeRecord {
            organization_code: record.organization_code.clone(),
            organization_name: record.organization_name.clone(),
            organization_status: record.organization_status.clone(),
        })
        .collect();

    let subtree_codes = collect_subtree_codes(organizations, organization_code);
    let direct_member_count = workforce
        .iter()
        .filter(|record| record.primary_organization_code.as_deref() == Some(organization_code))
        .count();
    let subtree_active_member_count = workforce
        .iter()
        .filter(|record| {
            record
                .primary_organization_code
                .as_ref()
                .is_some_and(|organization_code| subtree_codes.contains(organization_code))
        })
        .count();

    Some(OrganizationStructureSnapshotRecord {
        organization_code: selected.organization_code,
        organization_name: selected.organization_name,
        parent_organization_code: selected.parent_organization_code,
        organization_status: selected.organization_status,
        ancestors,
        children,
        direct_member_count,
        subtree_organization_count: subtree_codes.len(),
        subtree_active_member_count,
    })
}

fn collect_subtree_codes(
    organizations: &[OrganizationRecord],
    root_organization_code: &str,
) -> Vec<String> {
    let mut codes = vec![root_organization_code.to_string()];
    let mut index = 0;

    while index < codes.len() {
        let current = codes[index].clone();
        let children: Vec<String> = organizations
            .iter()
            .filter(|record| {
                record.parent_organization_code.as_deref() == Some(current.as_str())
                    && record.organization_status != "deleted"
            })
            .map(|record| record.organization_code.clone())
            .collect();

        for child in children {
            if !codes.contains(&child) {
                codes.push(child);
            }
        }
        index += 1;
    }

    codes
}
