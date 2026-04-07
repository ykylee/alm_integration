use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::raw_ingestion::{
    CreateRawIngestionEventInput, RawIngestionRepository, RawIngestionRepositoryError,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use uuid::Uuid;

#[tokio::test]
async fn repository_persists_raw_ingestion_event() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_persists_raw_ingestion_event: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = RawIngestionRepository::new(pool.clone());

    let created = repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "ALM-123".to_string(),
            source_event_key: "jira-event-8891".to_string(),
            source_version: Some("42".to_string()),
            source_updated_at: Some("2026-04-07T08:15:00Z".to_string()),
            payload: serde_json::json!({"summary": "Sync process update", "status": "In Progress"}),
        })
        .await?;

    assert!(created.accepted);
    assert_eq!(created.status, "accepted");
    assert_eq!(created.source_system, "jira");

    let row = sqlx::query(
        "select source_system, normalization_status from raw_ingestion_event where source_event_key = $1",
    )
    .bind("jira-event-8891")
    .fetch_one(&pool)
    .await?;

    let source_system: String = row.get(0);
    let normalization_status: String = row.get(1);

    assert_eq!(source_system, "jira");
    assert_eq!(normalization_status, "pending");

    Ok(())
}

#[tokio::test]
async fn repository_returns_duplicate_result_for_same_event_key() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_returns_duplicate_result_for_same_event_key: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = RawIngestionRepository::new(pool.clone());
    let input = CreateRawIngestionEventInput {
        source_system: "jira".to_string(),
        source_object_type: "issue".to_string(),
        source_object_id: "ALM-123".to_string(),
        source_event_key: "jira-event-8891".to_string(),
        source_version: Some("42".to_string()),
        source_updated_at: Some("2026-04-07T08:15:00Z".to_string()),
        payload: serde_json::json!({"summary": "Sync process update", "status": "In Progress"}),
    };

    let first = repository.create(input.clone()).await?;
    let second = repository.create(input).await?;

    assert!(first.accepted);
    assert!(!second.accepted);
    assert_eq!(second.status, "duplicate");
    assert_eq!(first.run_id, second.run_id);

    let count: i64 =
        sqlx::query_scalar("select count(*) from raw_ingestion_event where source_event_key = $1")
            .bind("jira-event-8891")
            .fetch_one(&pool)
            .await?;

    assert_eq!(count, 1);

    Ok(())
}

#[tokio::test]
async fn repository_rejects_invalid_timestamp() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip repository_rejects_invalid_timestamp: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = RawIngestionRepository::new(pool);

    let result = repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "ALM-123".to_string(),
            source_event_key: "jira-event-8891".to_string(),
            source_version: Some("42".to_string()),
            source_updated_at: Some("invalid".to_string()),
            payload: serde_json::json!({"summary": "Sync process update"}),
        })
        .await;

    assert!(matches!(
        result,
        Err(RawIngestionRepositoryError::InvalidSourceUpdatedAt(_))
    ));

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
