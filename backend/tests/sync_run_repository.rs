use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::sync_runs::{
    CancelSyncRunInput, CreateSyncRunInput, SyncRunListFilter, SyncRunRepository,
    SyncRunRepositoryError,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use uuid::Uuid;

#[tokio::test]
async fn repository_persists_created_sync_run() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_persists_created_sync_run: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = SyncRunRepository::new(pool.clone());

    let created = repository
        .create(CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: Some("manual test".to_string()),
        })
        .await?;

    let loaded = repository
        .get(&created.run_id)
        .await?
        .expect("created run should exist");

    assert_eq!(loaded.source_system, "jira");
    assert_eq!(loaded.mode, "incremental");
    assert_eq!(loaded.run_status, "queued");
    assert_eq!(loaded.status_reason_code, "manual_run_requested");
    assert_eq!(loaded.reason.as_deref(), Some("manual test"));

    Ok(())
}

#[tokio::test]
async fn repository_retries_failed_run_with_retry_link() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_retries_failed_run_with_retry_link: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = SyncRunRepository::new(pool.clone());

    let created = repository
        .create(CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: None,
        })
        .await?;

    sqlx::query(
        "update integration_run set run_status = 'failed', status_reason_code = 'processing_failed' where external_run_id = $1",
    )
    .bind(&created.run_id)
    .execute(&pool)
    .await?;

    let retried = repository
        .retry(&created.run_id, Some("retry".to_string()))
        .await?;

    assert_eq!(retried.run_status, "queued");
    assert_eq!(retried.status_reason_code, "retry_enqueued");
    assert_eq!(
        retried.retry_of_run_id.as_deref(),
        Some(created.run_id.as_str())
    );

    Ok(())
}

#[tokio::test]
async fn repository_cancels_queued_run_with_audit_fields() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_cancels_queued_run_with_audit_fields: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = SyncRunRepository::new(pool.clone());

    let created = repository
        .create(CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: None,
        })
        .await?;

    let cancelled = repository
        .cancel(
            &created.run_id,
            CancelSyncRunInput {
                requested_by: "admin.user".to_string(),
                reason: Some("stop".to_string()),
                cancel_reason_code: Some("operator_manual_stop".to_string()),
            },
        )
        .await?;

    assert!(cancelled.accepted);
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

    Ok(())
}

#[tokio::test]
async fn repository_rejects_retry_for_non_retriable_run() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_rejects_retry_for_non_retriable_run: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = SyncRunRepository::new(pool.clone());

    let created = repository
        .create(CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: None,
        })
        .await?;

    let result = repository.retry(&created.run_id, None).await;

    assert!(matches!(result, Err(SyncRunRepositoryError::NotRetriable)));

    Ok(())
}

#[tokio::test]
async fn repository_lists_runs_with_source_system_and_status_filters() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_lists_runs_with_source_system_and_status_filters: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = SyncRunRepository::new(pool.clone());

    let jira_run = repository
        .create(CreateSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: None,
        })
        .await?;

    let bitbucket_run = repository
        .create(CreateSyncRunInput {
            source_system: "bitbucket".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["OPS"]}),
            reason: None,
        })
        .await?;

    sqlx::query("update integration_run set run_status = 'running' where external_run_id = $1")
        .bind(&bitbucket_run.run_id)
        .execute(&pool)
        .await?;

    let items = repository
        .list(&SyncRunListFilter {
            source_system: Some("bitbucket".to_string()),
            run_status: Some("running".to_string()),
            ..SyncRunListFilter::default()
        })
        .await?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].run_id, bitbucket_run.run_id);
    assert_ne!(items[0].run_id, jira_run.run_id);

    Ok(())
}

async fn connect_and_migrate(test_db: &TestDatabase) -> anyhow::Result<PgPool> {
    let settings = Settings {
        bind_address: "127.0.0.1:8080".to_string(),
        database_url: test_db.database_url(),
        database_max_connections: 5,
        auto_apply_migrations: true,
        cors_allowed_origins: vec![],
    };
    let pool = connect(&settings).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

struct TestDatabase {
    admin_pool: PgPool,
    database_name: String,
    database_url: String,
}

impl TestDatabase {
    async fn create() -> anyhow::Result<Option<Self>> {
        let Some(admin_url) = std::env::var("ALM_BACKEND_TEST_DATABASE_ADMIN_URL").ok() else {
            return Ok(None);
        };

        let admin_options = PgConnectOptions::from_str(&admin_url)?;
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(admin_options.clone())
            .await?;
        let database_name = format!("alm_test_{}", Uuid::new_v4().simple());
        let create_statement = format!("create database {database_name}");

        sqlx::query(&create_statement).execute(&admin_pool).await?;

        let database_url = admin_options
            .clone()
            .database(&database_name)
            .to_url_lossy()
            .to_string();

        Ok(Some(Self {
            admin_pool,
            database_name,
            database_url,
        }))
    }

    fn database_url(&self) -> String {
        self.database_url.clone()
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let admin_pool = self.admin_pool.clone();
        let database_name = self.database_name.clone();

        tokio::spawn(async move {
            let terminate_statement = format!(
                "select pg_terminate_backend(pid) from pg_stat_activity where datname = '{database_name}' and pid <> pg_backend_pid()"
            );
            let drop_statement = format!("drop database if exists {database_name}");

            let _ = sqlx::query(&terminate_statement).execute(&admin_pool).await;
            let _ = sqlx::query(&drop_statement).execute(&admin_pool).await;
            admin_pool.close().await;
        });
    }
}
