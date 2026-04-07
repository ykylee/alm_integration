use std::str::FromStr;

use std::sync::Arc;

use backend::adapters::{HttpTransport, build_registry_from_endpoint_configs};
use backend::adapters::config_loader::DbAdapterConfigLoader;
use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::security::secrets::EnvSecretCipher;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use uuid::Uuid;

struct NullTransport;

#[async_trait::async_trait]
impl HttpTransport for NullTransport {
    async fn get_json(
        &self,
        _request: backend::adapters::AdapterHttpRequest,
    ) -> Result<serde_json::Value, backend::adapters::AdapterError> {
        Ok(serde_json::json!({}))
    }
}

#[tokio::test]
async fn loader_returns_endpoint_configs_from_database() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip loader_returns_endpoint_configs_from_database: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    seed_endpoint_config(&pool).await?;

    let loader = DbAdapterConfigLoader::new(pool);
    let configs = loader.load_endpoint_configs().await?;

    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].source_system, "jira");
    assert_eq!(
        configs[0].base_url.as_deref(),
        Some("https://jira.example.com")
    );
    assert_eq!(configs[0].bearer_token.as_deref(), Some("jira-live-token"));
    assert_eq!(
        configs[0].push_signing_secret.as_deref(),
        Some("jira-live-token")
    );
    assert!(configs[0].enable_pull);
    assert!(configs[0].enable_push);

    Ok(())
}

#[tokio::test]
async fn loader_builds_registry_from_database_configs() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip loader_builds_registry_from_database_configs: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    seed_endpoint_config(&pool).await?;

    let loader = DbAdapterConfigLoader::new(pool);
    let endpoint_configs = loader.load_endpoint_configs().await?;
    let registry = build_registry_from_endpoint_configs(&endpoint_configs, Arc::new(NullTransport))
        .expect("registry should be created from DB configs");

    assert!(registry.get_pull_adapter("jira").is_some());
    assert!(registry.get_push_adapter("jira").is_some());

    Ok(())
}

async fn seed_endpoint_config(pool: &PgPool) -> anyhow::Result<()> {
    unsafe {
        std::env::set_var(
            "ALM_BACKEND_SECRET_KEY_K1",
            "adapter-config-loader-test-key-material",
        );
    }
    let system_id = Uuid::new_v4();
    let endpoint_id = Uuid::new_v4();
    let credential_id = Uuid::new_v4();
    let secret_cipher = EnvSecretCipher::new();
    let encrypted_secret = secret_cipher.encrypt("jira-live-token", Some("k1"))?;

    sqlx::query(
        r#"
        insert into integration_system (
          integration_system_id,
          system_code,
          system_name,
          system_type,
          authentication_type,
          connection_status,
          owner_team,
          created_at,
          updated_at
        ) values ($1, 'jira_primary', 'Jira Primary', 'jira', 'bearer_token', 'active', 'alm', now(), now())
        "#,
    )
    .bind(system_id)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        insert into integration_endpoint (
          integration_endpoint_id,
          integration_system_id,
          endpoint_type,
          endpoint_name,
          base_url,
          resource_path,
          request_method,
          credential_binding_mode,
          is_active,
          created_at,
          updated_at
        ) values ($1, $2, 'both', 'primary', 'https://jira.example.com', '/rest/api/2/search', 'GET', 'endpoint', true, now(), now())
        "#,
    )
    .bind(endpoint_id)
    .bind(system_id)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        insert into integration_credential (
          integration_credential_id,
          integration_system_id,
          integration_endpoint_id,
          credential_type,
          principal_id,
          secret_ciphertext,
          secret_key_version,
          secret_fingerprint,
          rotation_status,
          effective_from,
          effective_to,
          last_validated_at,
          last_updated_by,
          created_at,
          updated_at
        ) values (
          $1, $2, $3, 'token', 'jira-bot', $4, 'k1', 'fp-1',
          'active', now(), null, now(), 'tester', now(), now()
        )
        "#,
    )
    .bind(credential_id)
    .bind(system_id)
    .bind(endpoint_id)
    .bind(encrypted_secret)
    .execute(pool)
    .await?;

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
