use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::normalization::NormalizationPipeline;
use backend::services::raw_ingestion::{CreateRawIngestionEventInput, RawIngestionRepository};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use uuid::Uuid;

#[tokio::test]
async fn normalization_pipeline_creates_reference_and_marks_event_normalized() -> anyhow::Result<()>
{
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip normalization_pipeline_creates_reference_and_marks_event_normalized: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let raw_repository = RawIngestionRepository::new(pool.clone());
    raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "ALM-123".to_string(),
            source_event_key: "jira-event-8891".to_string(),
            source_version: Some("42".to_string()),
            source_updated_at: Some("2026-04-07T08:15:00Z".to_string()),
            payload: serde_json::json!({"summary": "Sync process update"}),
        })
        .await?;

    let pipeline = NormalizationPipeline::new(pool.clone());
    let result = pipeline.normalize_pending(10).await?;

    assert_eq!(result.processed_count, 1);
    assert_eq!(result.normalized_count, 1);
    assert_eq!(result.skipped_count, 0);

    let row = sqlx::query(
        r#"
        select
          rie.normalization_status,
          nrr.target_entity_type,
          im.source_system_code,
          im.source_identity_key,
          im.internal_entity_type,
          im.mapping_status
        from raw_ingestion_event rie
        join normalized_record_reference nrr
          on nrr.raw_ingestion_event_id = rie.raw_ingestion_event_id
        join identity_mapping im
          on im.internal_entity_type = nrr.target_entity_type
         and im.internal_entity_id = nrr.target_entity_id
        where rie.source_event_key = $1
        "#,
    )
    .bind("jira-event-8891")
    .fetch_one(&pool)
    .await?;

    let normalization_status: String = row.get(0);
    let target_entity_type: String = row.get(1);
    let source_system_code: String = row.get(2);
    let source_identity_key: String = row.get(3);
    let internal_entity_type: String = row.get(4);
    let mapping_status: String = row.get(5);

    assert_eq!(normalization_status, "normalized");
    assert_eq!(target_entity_type, "work_item");
    assert_eq!(source_system_code, "jira");
    assert_eq!(source_identity_key, "issue:ALM-123");
    assert_eq!(internal_entity_type, "work_item");
    assert_eq!(mapping_status, "active");

    Ok(())
}

#[tokio::test]
async fn normalization_pipeline_skips_non_pending_events() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip normalization_pipeline_skips_non_pending_events: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let raw_repository = RawIngestionRepository::new(pool.clone());
    raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "ALM-999".to_string(),
            source_event_key: "jira-event-9999".to_string(),
            source_version: Some("7".to_string()),
            source_updated_at: Some("2026-04-07T11:15:00Z".to_string()),
            payload: serde_json::json!({
                "summary": "Child task before parent",
                "references": { "missing": ["project:OPS"] }
            }),
        })
        .await?;

    let pipeline = NormalizationPipeline::new(pool.clone());
    let result = pipeline.normalize_pending(10).await?;

    assert_eq!(result.processed_count, 0);
    assert_eq!(result.normalized_count, 0);

    let count: i64 = sqlx::query_scalar("select count(*) from normalized_record_reference")
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 0);

    let mapping_count: i64 = sqlx::query_scalar("select count(*) from identity_mapping")
        .fetch_one(&pool)
        .await?;
    assert_eq!(mapping_count, 0);

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
