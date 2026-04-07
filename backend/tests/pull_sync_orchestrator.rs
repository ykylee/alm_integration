use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::pull_sync::{PullRecordInput, PullSyncOrchestrator, PullSyncRunInput};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use uuid::Uuid;

#[tokio::test]
async fn orchestrator_creates_sync_run_and_raw_events() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip orchestrator_creates_sync_run_and_raw_events: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let orchestrator = PullSyncOrchestrator::new(pool.clone());

    let result = orchestrator
        .run(PullSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: Some("manual pull".to_string()),
            records: vec![
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "ALM-101".to_string(),
                    source_event_key: "jira-event-1001".to_string(),
                    source_version: Some("101".to_string()),
                    source_updated_at: Some("2026-04-07T09:00:00Z".to_string()),
                    payload: serde_json::json!({"summary": "first"}),
                },
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "ALM-102".to_string(),
                    source_event_key: "jira-event-1002".to_string(),
                    source_version: Some("102".to_string()),
                    source_updated_at: Some("2026-04-07T09:05:00Z".to_string()),
                    payload: serde_json::json!({"summary": "second"}),
                },
            ],
        })
        .await?;

    assert_eq!(result.run_status, "completed");
    assert_eq!(result.status_reason_code, "pull_completed");
    assert_eq!(result.processed_count, 2);
    assert_eq!(result.success_count, 2);
    assert_eq!(result.failure_count, 0);

    let counts = sqlx::query(
        r#"
        select ir.processed_count, ir.success_count, count(rie.raw_ingestion_event_id) as raw_count
        from integration_run ir
        left join raw_ingestion_event rie on rie.integration_run_id = ir.integration_run_id
        where ir.external_run_id = $1
        group by ir.processed_count, ir.success_count
        "#,
    )
    .bind(&result.run_id)
    .fetch_one(&pool)
    .await?;

    let processed_count: i32 = counts.get(0);
    let success_count: i32 = counts.get(1);
    let raw_count: i64 = counts.get(2);

    assert_eq!(processed_count, 2);
    assert_eq!(success_count, 2);
    assert_eq!(raw_count, 2);

    Ok(())
}

#[tokio::test]
async fn orchestrator_marks_partial_completion_when_record_fails() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip orchestrator_marks_partial_completion_when_record_fails: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let orchestrator = PullSyncOrchestrator::new(pool.clone());

    let result = orchestrator
        .run(PullSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["ALM"]}),
            reason: Some("manual pull".to_string()),
            records: vec![
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "ALM-101".to_string(),
                    source_event_key: "jira-event-1001".to_string(),
                    source_version: Some("101".to_string()),
                    source_updated_at: Some("2026-04-07T09:00:00Z".to_string()),
                    payload: serde_json::json!({"summary": "first"}),
                },
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "ALM-102".to_string(),
                    source_event_key: "jira-event-1002".to_string(),
                    source_version: Some("102".to_string()),
                    source_updated_at: Some("bad-timestamp".to_string()),
                    payload: serde_json::json!({"summary": "second"}),
                },
            ],
        })
        .await?;

    assert_eq!(result.run_status, "partially_completed");
    assert_eq!(result.status_reason_code, "pull_partially_completed");
    assert_eq!(result.processed_count, 2);
    assert_eq!(result.success_count, 1);
    assert_eq!(result.failure_count, 1);

    Ok(())
}

async fn connect_and_migrate(test_db: &TestDatabase) -> anyhow::Result<PgPool> {
    let settings = Settings {
        bind_address: "127.0.0.1:8080".to_string(),
        database_url: test_db.database_url(),
        database_max_connections: 5,
        auto_apply_migrations: true,
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
