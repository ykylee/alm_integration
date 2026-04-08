use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::master_data::{
    MasterDataRepository, OrganizationListFilter, UpsertOrganizationInput, UpsertWorkforceInput,
    WorkforceListFilter,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use uuid::Uuid;

#[tokio::test]
async fn master_data_repository_upserts_and_lists_organization_and_workforce() -> anyhow::Result<()>
{
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip master_data_repository_upserts_and_lists_organization_and_workforce: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let repository = MasterDataRepository::new(pool.clone());

    let parent = repository
        .upsert_organization(UpsertOrganizationInput {
            organization_code: "platform".to_string(),
            organization_name: "Platform Center".to_string(),
            parent_organization_code: None,
            organization_status: "active".to_string(),
            effective_from: Some("2026-04-08T00:00:00Z".to_string()),
            effective_to: None,
        })
        .await?;

    let child = repository
        .upsert_organization(UpsertOrganizationInput {
            organization_code: "payments".to_string(),
            organization_name: "Payments Team".to_string(),
            parent_organization_code: Some("platform".to_string()),
            organization_status: "active".to_string(),
            effective_from: Some("2026-04-08T00:00:00Z".to_string()),
            effective_to: None,
        })
        .await?;

    let workforce = repository
        .upsert_workforce(UpsertWorkforceInput {
            employee_number: "E9001".to_string(),
            display_name: "박연계".to_string(),
            employment_status: "active".to_string(),
            primary_organization_code: "payments".to_string(),
            job_family: Some("integration_engineering".to_string()),
            email: Some("integration@example.com".to_string()),
        })
        .await?;

    let organizations = repository
        .list_organizations(&OrganizationListFilter {
            organization_status: Some("active".to_string()),
            organization_code: None,
        })
        .await?;
    let workforce_items = repository
        .list_workforce(&WorkforceListFilter {
            employment_status: Some("active".to_string()),
            primary_organization_code: Some("payments".to_string()),
        })
        .await?;

    assert_eq!(parent.organization_code, "platform");
    assert_eq!(child.parent_organization_code.as_deref(), Some("platform"));
    assert_eq!(organizations.len(), 3);
    assert_eq!(workforce_items.len(), 1);
    assert_eq!(workforce.employee_number, "E9001");
    assert_eq!(workforce.primary_organization_code, "payments");
    assert_eq!(workforce_items[0].display_name, "박연계");

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
