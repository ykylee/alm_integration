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
    pub primary_organization_code: String,
    pub primary_organization_name: String,
    pub job_family: Option<String>,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
    pub primary_organization_code: String,
    pub job_family: Option<String>,
    pub email: Option<String>,
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
}

#[derive(Debug)]
pub enum MasterDataRepositoryError {
    InvalidTimestamp(String),
    InvalidReference(String),
    Database(sqlx::Error),
}

impl Display for MasterDataRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTimestamp(value) => write!(f, "invalid timestamp: {value}"),
            Self::InvalidReference(value) => write!(f, "invalid reference: {value}"),
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
        Ok(record)
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

    pub fn upsert_workforce(
        &mut self,
        input: UpsertWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataStoreError> {
        let organization = self
            .organizations
            .iter()
            .find(|record| record.organization_code == input.primary_organization_code)
            .cloned()
            .ok_or(MasterDataStoreError::InvalidReference)?;

        let now = Utc::now().to_rfc3339();
        let record = if let Some(existing) = self
            .workforce
            .iter_mut()
            .find(|record| record.employee_number == input.employee_number)
        {
            existing.display_name = input.display_name;
            existing.employment_status = input.employment_status;
            existing.primary_organization_code = organization.organization_code.clone();
            existing.primary_organization_name = organization.organization_name.clone();
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
                primary_organization_code: organization.organization_code,
                primary_organization_name: organization.organization_name,
                job_family: input.job_family,
                email: input.email,
                created_at: now.clone(),
                updated_at: now,
            };
            self.workforce.push(record.clone());
            record
        };

        Ok(record)
    }

    fn sync_workforce_organization_names(&mut self) {
        for workforce in &mut self.workforce {
            if let Some(organization) = self
                .organizations
                .iter()
                .find(|record| record.organization_code == workforce.primary_organization_code)
            {
                workforce.primary_organization_name = organization.organization_name.clone();
            }
        }
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
            join organization_master om
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

    pub async fn upsert_workforce(
        &self,
        input: UpsertWorkforceInput,
    ) -> Result<WorkforceRecord, MasterDataRepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let primary_organization_row = sqlx::query(
            r#"
            select organization_id, organization_code, organization_name
            from organization_master
            where organization_code = $1
            "#,
        )
        .bind(&input.primary_organization_code)
        .fetch_optional(&mut *transaction)
        .await?
        .ok_or_else(|| {
            MasterDataRepositoryError::InvalidReference(format!(
                "organization not found: {}",
                input.primary_organization_code
            ))
        })?;

        let primary_organization_id: Uuid = primary_organization_row.get("organization_id");

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
            join organization_master om
              on om.organization_id = wm.primary_organization_id
            where wm.workforce_id = $1
            "#,
        )
        .bind(workforce_id)
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(map_workforce_row(&row))
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
            .map_or(true, |value| &record.primary_organization_code == value)
}
