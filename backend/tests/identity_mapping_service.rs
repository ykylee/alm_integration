use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::identity_mapping::{IdentityMappingService, UpsertIdentityMappingInput};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Row};
use uuid::Uuid;

#[tokio::test]
async fn identity_mapping_service_creates_and_updates_mapping() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip identity_mapping_service_creates_and_updates_mapping: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let service = IdentityMappingService::new(pool.clone());
    let internal_entity_id = Uuid::new_v4();

    service
        .upsert(UpsertIdentityMappingInput {
            source_system_code: "jira".to_string(),
            source_identity_key: "issue:ALM-123".to_string(),
            internal_entity_type: "work_item".to_string(),
            internal_entity_id,
            mapping_status: "active".to_string(),
            verified_at: None,
        })
        .await?;

    service
        .upsert(UpsertIdentityMappingInput {
            source_system_code: "jira".to_string(),
            source_identity_key: "issue:ALM-123".to_string(),
            internal_entity_type: "work_item".to_string(),
            internal_entity_id,
            mapping_status: "verified".to_string(),
            verified_at: Some("2026-04-07T12:00:00Z".to_string()),
        })
        .await?;

    let row = sqlx::query(
        r#"
        select mapping_status, verified_at
        from identity_mapping
        where source_system_code = $1
          and source_identity_key = $2
          and internal_entity_type = $3
        "#,
    )
    .bind("jira")
    .bind("issue:ALM-123")
    .bind("work_item")
    .fetch_one(&pool)
    .await?;

    let mapping_status: String = row.get("mapping_status");
    let verified_at: chrono::DateTime<chrono::Utc> = row.get("verified_at");

    assert_eq!(mapping_status, "verified");
    assert_eq!(verified_at.to_rfc3339(), "2026-04-07T12:00:00+00:00");

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
