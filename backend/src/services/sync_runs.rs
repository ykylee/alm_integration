use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::Json;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct SyncRunRecord {
    pub run_id: String,
    pub source_system: String,
    pub mode: String,
    pub scope: serde_json::Value,
    pub run_status: String,
    pub queued_at: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub status_reason_code: String,
    pub cancel_requested_at: Option<String>,
    pub cancel_requested_by: Option<String>,
    pub cancel_reason_code: Option<String>,
    pub reason: Option<String>,
    pub processed_count: i32,
    pub success_count: i32,
    pub failure_count: i32,
    pub pending_count: i32,
    pub retry_of_run_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateSyncRunInput {
    pub source_system: String,
    pub mode: String,
    pub scope: serde_json::Value,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CancelSyncRunInput {
    pub requested_by: String,
    pub reason: Option<String>,
    pub cancel_reason_code: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CancelSyncRunResult {
    pub record: SyncRunRecord,
    pub accepted: bool,
    pub reason_code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum SyncRunStoreError {
    NotFound,
    NotRetriable,
    NotCancellable,
}

#[derive(Debug)]
pub enum SyncRunRepositoryError {
    NotFound,
    NotRetriable,
    NotCancellable,
    Database(sqlx::Error),
}

impl Display for SyncRunRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "sync run not found"),
            Self::NotRetriable => write!(f, "sync run is not retriable"),
            Self::NotCancellable => write!(f, "sync run is not cancellable"),
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl std::error::Error for SyncRunRepositoryError {}

impl From<sqlx::Error> for SyncRunRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Default)]
pub struct SyncRunStore {
    runs: Vec<SyncRunRecord>,
}

impl SyncRunStore {
    pub fn create(&mut self, input: CreateSyncRunInput) -> SyncRunRecord {
        let record = SyncRunRecord {
            run_id: format!("sync_{}", Uuid::new_v4().simple()),
            source_system: input.source_system,
            mode: input.mode,
            scope: input.scope,
            run_status: "queued".to_string(),
            queued_at: now().to_rfc3339(),
            started_at: None,
            ended_at: None,
            status_reason_code: "manual_run_requested".to_string(),
            cancel_requested_at: None,
            cancel_requested_by: None,
            cancel_reason_code: None,
            reason: input.reason,
            processed_count: 0,
            success_count: 0,
            failure_count: 0,
            pending_count: 0,
            retry_of_run_id: None,
        };
        self.runs.push(record.clone());
        record
    }

    pub fn list(&self) -> Vec<SyncRunRecord> {
        self.runs.clone()
    }

    pub fn get(&self, run_id: &str) -> Option<SyncRunRecord> {
        self.runs.iter().find(|run| run.run_id == run_id).cloned()
    }

    pub fn retry(
        &mut self,
        run_id: &str,
        reason: Option<String>,
    ) -> Result<SyncRunRecord, SyncRunStoreError> {
        let original = self
            .runs
            .iter()
            .find(|run| run.run_id == run_id)
            .cloned()
            .ok_or(SyncRunStoreError::NotFound)?;
        if !matches!(
            original.run_status.as_str(),
            "failed" | "partially_completed" | "cancelled"
        ) {
            return Err(SyncRunStoreError::NotRetriable);
        }
        let record = SyncRunRecord {
            run_id: format!("sync_{}", Uuid::new_v4().simple()),
            source_system: original.source_system,
            mode: original.mode,
            scope: original.scope,
            run_status: "queued".to_string(),
            queued_at: now().to_rfc3339(),
            started_at: None,
            ended_at: None,
            status_reason_code: "retry_enqueued".to_string(),
            cancel_requested_at: None,
            cancel_requested_by: None,
            cancel_reason_code: None,
            reason,
            processed_count: 0,
            success_count: 0,
            failure_count: 0,
            pending_count: 0,
            retry_of_run_id: Some(run_id.to_string()),
        };
        self.runs.push(record.clone());
        Ok(record)
    }

    pub fn cancel(
        &mut self,
        run_id: &str,
        input: CancelSyncRunInput,
    ) -> Result<CancelSyncRunResult, SyncRunStoreError> {
        let run = self
            .runs
            .iter_mut()
            .find(|run| run.run_id == run_id)
            .ok_or(SyncRunStoreError::NotFound)?;

        if run.cancel_requested_at.is_some() {
            return Ok(CancelSyncRunResult {
                record: run.clone(),
                accepted: false,
                reason_code: "run_already_cancel_requested".to_string(),
                message: "cancellation request already registered".to_string(),
            });
        }

        if matches!(
            run.run_status.as_str(),
            "completed" | "failed" | "cancelled" | "partially_completed"
        ) {
            return Ok(CancelSyncRunResult {
                record: run.clone(),
                accepted: false,
                reason_code: "run_already_finished".to_string(),
                message: "run already finished; cancellation not applied".to_string(),
            });
        }

        if run.run_status != "queued" && run.run_status != "running" {
            return Err(SyncRunStoreError::NotCancellable);
        }

        run.cancel_requested_at = Some(now().to_rfc3339());
        run.cancel_requested_by = Some(input.requested_by);
        run.cancel_reason_code = input.cancel_reason_code;
        run.reason = input.reason.or_else(|| run.reason.clone());
        run.status_reason_code = "cancel_requested".to_string();

        Ok(CancelSyncRunResult {
            record: run.clone(),
            accepted: true,
            reason_code: "cancel_requested".to_string(),
            message: "cancellation request registered".to_string(),
        })
    }
}

pub struct SyncRunRepository {
    pool: PgPool,
}

impl SyncRunRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        input: CreateSyncRunInput,
    ) -> Result<SyncRunRecord, SyncRunRepositoryError> {
        let job_id = self.ensure_manual_job().await?;
        let internal_id = Uuid::new_v4();
        let external_run_id = format!("sync_{}", Uuid::new_v4().simple());
        let queued_at = now();

        sqlx::query(
            r#"
            insert into integration_run (
              integration_run_id,
              integration_job_id,
              external_run_id,
              source_system,
              run_mode,
              run_scope,
              reason,
              queued_at,
              run_status,
              status_reason_code,
              processed_count,
              success_count,
              failure_count,
              pending_count,
              created_at
            )
            values (
              $1, $2, $3, $4, $5, $6, $7, $8, 'queued', 'manual_run_requested', 0, 0, 0, 0, $8
            )
            "#,
        )
        .bind(internal_id)
        .bind(job_id)
        .bind(&external_run_id)
        .bind(&input.source_system)
        .bind(&input.mode)
        .bind(Json(&input.scope))
        .bind(&input.reason)
        .bind(queued_at)
        .execute(&self.pool)
        .await?;

        self.get(&external_run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)
    }

    pub async fn list(&self) -> Result<Vec<SyncRunRecord>, SyncRunRepositoryError> {
        let rows = sqlx::query_as::<_, SyncRunRow>(
            r#"
            select
              external_run_id,
              source_system,
              run_mode,
              run_scope,
              run_status,
              queued_at,
              started_at,
              ended_at,
              status_reason_code,
              cancel_requested_at,
              cancel_requested_by,
              cancel_reason_code,
              reason,
              processed_count,
              success_count,
              failure_count,
              pending_count,
              (
                select original.external_run_id
                from integration_run original
                where original.integration_run_id = integration_run.retry_of_run_id
              ) as retry_of_external_run_id
            from integration_run
            order by queued_at desc
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(SyncRunRow::into_record).collect())
    }

    pub async fn get(&self, run_id: &str) -> Result<Option<SyncRunRecord>, SyncRunRepositoryError> {
        let row = sqlx::query_as::<_, SyncRunRow>(
            r#"
            select
              external_run_id,
              source_system,
              run_mode,
              run_scope,
              run_status,
              queued_at,
              started_at,
              ended_at,
              status_reason_code,
              cancel_requested_at,
              cancel_requested_by,
              cancel_reason_code,
              reason,
              processed_count,
              success_count,
              failure_count,
              pending_count,
              (
                select original.external_run_id
                from integration_run original
                where original.integration_run_id = integration_run.retry_of_run_id
              ) as retry_of_external_run_id
            from integration_run
            where external_run_id = $1
            "#,
        )
        .bind(run_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(SyncRunRow::into_record))
    }

    pub async fn retry(
        &self,
        run_id: &str,
        reason: Option<String>,
    ) -> Result<SyncRunRecord, SyncRunRepositoryError> {
        let original = self
            .get(run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)?;
        if !matches!(
            original.run_status.as_str(),
            "failed" | "partially_completed" | "cancelled"
        ) {
            return Err(SyncRunRepositoryError::NotRetriable);
        }

        let job_id = self.find_job_id_by_external_run_id(run_id).await?;
        let internal_id = Uuid::new_v4();
        let external_run_id = format!("sync_{}", Uuid::new_v4().simple());
        let queued_at = now();

        sqlx::query(
            r#"
            insert into integration_run (
              integration_run_id,
              integration_job_id,
              external_run_id,
              source_system,
              run_mode,
              run_scope,
              reason,
              queued_at,
              run_status,
              status_reason_code,
              processed_count,
              success_count,
              failure_count,
              pending_count,
              retry_of_run_id,
              created_at
            )
            values (
              $1, $2, $3, $4, $5, $6, $7, $8, 'queued', 'retry_enqueued', 0, 0, 0, 0,
              (select integration_run_id from integration_run where external_run_id = $9),
              $8
            )
            "#,
        )
        .bind(internal_id)
        .bind(job_id)
        .bind(&external_run_id)
        .bind(&original.source_system)
        .bind(&original.mode)
        .bind(Json(&original.scope))
        .bind(&reason)
        .bind(queued_at)
        .bind(run_id)
        .execute(&self.pool)
        .await?;

        self.get(&external_run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)
    }

    pub async fn cancel(
        &self,
        run_id: &str,
        input: CancelSyncRunInput,
    ) -> Result<CancelSyncRunResult, SyncRunRepositoryError> {
        let current = self
            .get(run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)?;

        if current.cancel_requested_at.is_some() {
            return Ok(CancelSyncRunResult {
                record: current,
                accepted: false,
                reason_code: "run_already_cancel_requested".to_string(),
                message: "cancellation request already registered".to_string(),
            });
        }

        if matches!(
            current.run_status.as_str(),
            "completed" | "failed" | "cancelled" | "partially_completed"
        ) {
            return Ok(CancelSyncRunResult {
                record: current,
                accepted: false,
                reason_code: "run_already_finished".to_string(),
                message: "run already finished; cancellation not applied".to_string(),
            });
        }

        if current.run_status != "queued" && current.run_status != "running" {
            return Err(SyncRunRepositoryError::NotCancellable);
        }

        let cancel_requested_at = now();
        sqlx::query(
            r#"
            update integration_run
            set
              cancel_requested_at = $2,
              cancel_requested_by = $3,
              cancel_reason_code = $4,
              reason = coalesce($5, reason),
              status_reason_code = 'cancel_requested'
            where external_run_id = $1
            "#,
        )
        .bind(run_id)
        .bind(cancel_requested_at)
        .bind(&input.requested_by)
        .bind(&input.cancel_reason_code)
        .bind(&input.reason)
        .execute(&self.pool)
        .await?;

        let record = self
            .get(run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)?;

        Ok(CancelSyncRunResult {
            record,
            accepted: true,
            reason_code: "cancel_requested".to_string(),
            message: "cancellation request registered".to_string(),
        })
    }

    #[allow(dead_code)]
    pub async fn complete_pull_run(
        &self,
        run_id: &str,
        processed_count: i32,
        success_count: i32,
        failure_count: i32,
    ) -> Result<SyncRunRecord, SyncRunRepositoryError> {
        let run_status = if failure_count > 0 {
            "partially_completed"
        } else {
            "completed"
        };
        let status_reason_code = if failure_count > 0 {
            "pull_partially_completed"
        } else {
            "pull_completed"
        };
        let ended_at = now();

        sqlx::query(
            r#"
            update integration_run
            set
              run_status = $2,
              status_reason_code = $3,
              ended_at = $4,
              processed_count = $5,
              success_count = $6,
              failure_count = $7,
              pending_count = 0
            where external_run_id = $1
            "#,
        )
        .bind(run_id)
        .bind(run_status)
        .bind(status_reason_code)
        .bind(ended_at)
        .bind(processed_count)
        .bind(success_count)
        .bind(failure_count)
        .execute(&self.pool)
        .await?;

        self.get(run_id)
            .await?
            .ok_or(SyncRunRepositoryError::NotFound)
    }

    async fn ensure_manual_job(&self) -> Result<Uuid, SyncRunRepositoryError> {
        if let Some(job_id) = sqlx::query_scalar::<_, Uuid>(
            "select integration_job_id from integration_job where job_code = 'manual_sync'",
        )
        .fetch_optional(&self.pool)
        .await?
        {
            return Ok(job_id);
        }

        let job_id = Uuid::new_v4();
        let now = now();
        sqlx::query(
            r#"
            insert into integration_job (
              integration_job_id,
              integration_system_id,
              integration_endpoint_id,
              job_code,
              job_name,
              job_type,
              schedule_expression,
              job_status,
              created_at,
              updated_at
            )
            values ($1, null, null, 'manual_sync', 'Manual Sync Run', 'manual', null, 'active', $2, $2)
            on conflict (job_code) do nothing
            "#,
        )
        .bind(job_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        sqlx::query_scalar::<_, Uuid>(
            "select integration_job_id from integration_job where job_code = 'manual_sync'",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn find_job_id_by_external_run_id(
        &self,
        run_id: &str,
    ) -> Result<Uuid, SyncRunRepositoryError> {
        sqlx::query_scalar::<_, Uuid>(
            "select integration_job_id from integration_run where external_run_id = $1",
        )
        .bind(run_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(SyncRunRepositoryError::NotFound)
    }
}

#[derive(sqlx::FromRow)]
struct SyncRunRow {
    external_run_id: String,
    source_system: String,
    run_mode: String,
    run_scope: Json<serde_json::Value>,
    run_status: String,
    queued_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    status_reason_code: Option<String>,
    cancel_requested_at: Option<DateTime<Utc>>,
    cancel_requested_by: Option<String>,
    cancel_reason_code: Option<String>,
    reason: Option<String>,
    processed_count: i32,
    success_count: i32,
    failure_count: i32,
    pending_count: i32,
    retry_of_external_run_id: Option<String>,
}

impl SyncRunRow {
    fn into_record(self) -> SyncRunRecord {
        SyncRunRecord {
            run_id: self.external_run_id,
            source_system: self.source_system,
            mode: self.run_mode,
            scope: self.run_scope.0,
            run_status: self.run_status,
            queued_at: self.queued_at.to_rfc3339(),
            started_at: self.started_at.map(|value| value.to_rfc3339()),
            ended_at: self.ended_at.map(|value| value.to_rfc3339()),
            status_reason_code: self.status_reason_code.unwrap_or_default(),
            cancel_requested_at: self.cancel_requested_at.map(|value| value.to_rfc3339()),
            cancel_requested_by: self.cancel_requested_by,
            cancel_reason_code: self.cancel_reason_code,
            reason: self.reason,
            processed_count: self.processed_count,
            success_count: self.success_count,
            failure_count: self.failure_count,
            pending_count: self.pending_count,
            retry_of_run_id: self.retry_of_external_run_id,
        }
    }
}

fn now() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::{CancelSyncRunInput, CreateSyncRunInput, SyncRunStore, SyncRunStoreError};

    fn create_input() -> CreateSyncRunInput {
        CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: Some("test".to_string()),
        }
    }

    #[test]
    fn create_sets_default_queued_state() {
        let mut store = SyncRunStore::default();

        let record = store.create(create_input());

        assert_eq!(record.run_status, "queued");
        assert_eq!(record.status_reason_code, "manual_run_requested");
        assert!(record.started_at.is_none());
        assert!(record.cancel_requested_at.is_none());
        assert_eq!(store.list().len(), 1);
    }

    #[test]
    fn retry_rejects_non_retriable_status() {
        let mut store = SyncRunStore::default();
        let record = store.create(create_input());

        let result = store.retry(&record.run_id, Some("retry".to_string()));

        assert!(matches!(result, Err(SyncRunStoreError::NotRetriable)));
    }

    #[test]
    fn retry_creates_new_run_from_failed_status() {
        let mut store = SyncRunStore::default();
        let record = store.create(create_input());
        store.runs[0].run_status = "failed".to_string();

        let retry = store
            .retry(&record.run_id, Some("retry".to_string()))
            .unwrap();

        assert_eq!(retry.run_status, "queued");
        assert_eq!(retry.status_reason_code, "retry_enqueued");
        assert_eq!(
            retry.retry_of_run_id.as_deref(),
            Some(record.run_id.as_str())
        );
        assert_eq!(store.list().len(), 2);
    }

    #[test]
    fn cancel_records_audit_fields_for_queued_run() {
        let mut store = SyncRunStore::default();
        let record = store.create(create_input());

        let cancelled = store
            .cancel(
                &record.run_id,
                CancelSyncRunInput {
                    requested_by: "admin.user".to_string(),
                    reason: Some("stop".to_string()),
                    cancel_reason_code: Some("operator_manual_stop".to_string()),
                },
            )
            .unwrap();

        assert!(cancelled.accepted);
        assert_eq!(cancelled.reason_code, "cancel_requested");
        assert_eq!(cancelled.record.status_reason_code, "cancel_requested");
        assert_eq!(
            cancelled.record.cancel_requested_by.as_deref(),
            Some("admin.user")
        );
        assert_eq!(
            cancelled.record.cancel_reason_code.as_deref(),
            Some("operator_manual_stop")
        );
        assert!(cancelled.record.cancel_requested_at.is_some());
    }

    #[test]
    fn cancel_returns_finished_response_for_completed_run() {
        let mut store = SyncRunStore::default();
        let record = store.create(create_input());
        store.runs[0].run_status = "completed".to_string();

        let result = store
            .cancel(
                &record.run_id,
                CancelSyncRunInput {
                    requested_by: "admin.user".to_string(),
                    reason: Some("stop".to_string()),
                    cancel_reason_code: Some("operator_manual_stop".to_string()),
                },
            )
            .unwrap();

        assert!(!result.accepted);
        assert_eq!(result.reason_code, "run_already_finished");
        assert_eq!(
            result.message,
            "run already finished; cancellation not applied"
        );
        assert_eq!(result.record.run_status, "completed");
    }
}
