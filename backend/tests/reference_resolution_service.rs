use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::identity_mapping::{IdentityMappingService, UpsertIdentityMappingInput};
use backend::services::raw_ingestion::{CreateRawIngestionEventInput, RawIngestionRepository};
use backend::services::reference_resolution::ReferenceResolutionService;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use uuid::Uuid;

#[tokio::test]
async fn reference_resolution_service_promotes_resolved_pending_reference() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip reference_resolution_service_promotes_resolved_pending_reference: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let raw_repository = RawIngestionRepository::new(pool.clone());
    let identity_mapping_service = IdentityMappingService::new(pool.clone());
    let resolution_service = ReferenceResolutionService::new(pool.clone());

    raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "OPS-101".to_string(),
            source_event_key: "jira-event-ops-101".to_string(),
            source_version: Some("2".to_string()),
            source_updated_at: Some("2026-04-07T09:05:00Z".to_string()),
            payload: serde_json::json!({
                "summary": "Blocked by project reference",
                "references": {
                    "missing": ["project:OPS"]
                }
            }),
        })
        .await?;

    identity_mapping_service
        .upsert(UpsertIdentityMappingInput {
            source_system_code: "jira".to_string(),
            source_identity_key: "project:OPS".to_string(),
            internal_entity_type: "project".to_string(),
            internal_entity_id: Uuid::new_v4(),
            mapping_status: "active".to_string(),
            verified_at: None,
        })
        .await?;

    let result = resolution_service.resolve_pending_references(10).await?;

    assert_eq!(result.processed_count, 1);
    assert_eq!(result.resolved_count, 1);
    assert_eq!(result.pending_count, 0);

    let status: String = sqlx::query_scalar(
        "select normalization_status from raw_ingestion_event where source_event_key = $1",
    )
    .bind("jira-event-ops-101")
    .fetch_one(&pool)
    .await?;

    assert_eq!(status, "pending");

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
