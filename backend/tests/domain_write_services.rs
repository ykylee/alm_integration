use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::identity_mapping::{IdentityMappingService, UpsertIdentityMappingInput};
use backend::services::normalization::NormalizationPipeline;
use backend::services::project_write::ProjectWriteService;
use backend::services::raw_ingestion::{CreateRawIngestionEventInput, RawIngestionRepository};
use backend::services::work_item_write::WorkItemWriteService;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use uuid::Uuid;

#[tokio::test]
async fn domain_write_services_apply_project_and_work_item_for_normalized_records()
-> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip domain_write_services_apply_project_and_work_item_for_normalized_records: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let raw_repository = RawIngestionRepository::new(pool.clone());
    let normalization_pipeline = NormalizationPipeline::new(pool.clone());
    let identity_mapping_service = IdentityMappingService::new(pool.clone());
    let project_write_service = ProjectWriteService::new(pool.clone());
    let work_item_write_service = WorkItemWriteService::new(pool.clone());

    let project_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "project".to_string(),
            source_object_id: "OPS".to_string(),
            source_event_key: "jira-project-ops".to_string(),
            source_version: Some("1".to_string()),
            source_updated_at: Some("2026-04-07T09:00:00Z".to_string()),
            payload: serde_json::json!({"name": "Operations Platform"}),
        })
        .await?;

    normalization_pipeline.normalize_pending(10).await?;

    let project_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&project_record.run_id)
    .fetch_one(&pool)
    .await?;

    project_write_service
        .apply_for_run(project_run_internal_id)
        .await?;

    let project_row = sqlx::query("select project_id from project where project_code = $1")
        .bind("OPS")
        .fetch_one(&pool)
        .await?;
    let project_id: Uuid = project_row.get("project_id");

    identity_mapping_service
        .upsert(UpsertIdentityMappingInput {
            source_system_code: "jira".to_string(),
            source_identity_key: "project:OPS".to_string(),
            internal_entity_type: "project".to_string(),
            internal_entity_id: project_id,
            mapping_status: "verified".to_string(),
            verified_at: Some("2026-04-07T09:01:00Z".to_string()),
        })
        .await?;

    let work_item_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "OPS-101".to_string(),
            source_event_key: "jira-issue-ops-101".to_string(),
            source_version: Some("2".to_string()),
            source_updated_at: Some("2026-04-07T09:05:00Z".to_string()),
            payload: serde_json::json!({
                "summary": "Implement normalized write path",
                "project_key": "OPS",
                "issue_type": "task"
            }),
        })
        .await?;

    normalization_pipeline.normalize_pending(10).await?;

    let work_item_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&work_item_record.run_id)
    .fetch_one(&pool)
    .await?;

    work_item_write_service
        .apply_for_run(work_item_run_internal_id)
        .await?;

    let row = sqlx::query(
        r#"
        select p.project_code, w.work_item_key, w.title
        from work_item w
        join project p on p.project_id = w.project_id
        where w.work_item_key = $1
        "#,
    )
    .bind("OPS-101")
    .fetch_one(&pool)
    .await?;

    let project_code: String = row.get("project_code");
    let work_item_key: String = row.get("work_item_key");
    let title: String = row.get("title");

    assert_eq!(project_code, "OPS");
    assert_eq!(work_item_key, "OPS-101");
    assert_eq!(title, "Implement normalized write path");

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
