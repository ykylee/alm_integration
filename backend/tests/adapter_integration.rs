use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::adapters::{
    AdapterError, AdapterRegistry, PullAdapterRequest, PullSourceAdapter, PushAdapterRequest,
    PushEventAdapter,
};
use backend::app_state::AppState;
use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::http::router::build_router;
use backend::services::pull_sync::{PullRecordInput, PullSyncOrchestrator, PullSyncRequestInput};
use backend::services::raw_ingestion::CreateRawIngestionEventInput;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use tower::ServiceExt;
use uuid::Uuid;

struct JiraPushAdapter;

impl PushEventAdapter for JiraPushAdapter {
    fn source_system(&self) -> &'static str {
        "jira"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        Ok(CreateRawIngestionEventInput {
            source_system: request.source_system,
            source_object_type: request.source_object_type,
            source_object_id: request.source_object_id,
            source_event_key: format!("adapted-{}", request.source_event_key),
            source_version: request.source_version,
            source_updated_at: request.source_updated_at,
            payload: request.payload,
        })
    }
}

struct JiraPullAdapter;

#[async_trait]
impl PullSourceAdapter for JiraPullAdapter {
    fn source_system(&self) -> &'static str {
        "jira"
    }

    async fn pull(
        &self,
        _request: PullAdapterRequest,
    ) -> Result<Vec<PullRecordInput>, AdapterError> {
        Ok(vec![PullRecordInput {
            source_object_type: "issue".to_string(),
            source_object_id: "ALM-777".to_string(),
            source_event_key: "jira-event-777".to_string(),
            source_version: Some("777".to_string()),
            source_updated_at: Some("2026-04-07T10:00:00Z".to_string()),
            payload: serde_json::json!({"summary": "adapter fetched"}),
        }])
    }
}

#[tokio::test]
async fn ingestion_route_uses_registered_push_adapter() {
    let mut registry = AdapterRegistry::new();
    registry.register_push_adapter(Arc::new(JiraPushAdapter));
    let app = build_router(AppState::with_adapters(registry));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ingestion/events")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"source_system":"jira","source_object_type":"issue","source_object_id":"ALM-123","source_event_key":"jira-event-8891","source_version":"42","source_updated_at":"2026-04-07T08:15:00Z","payload":{"summary":"Sync process update","status":"In Progress"}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["request_id"], "adapted-jira-event-8891");
}

#[tokio::test]
async fn pull_orchestrator_uses_registered_pull_adapter() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip pull_orchestrator_uses_registered_pull_adapter: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let orchestrator = PullSyncOrchestrator::new(pool.clone());
    let mut registry = AdapterRegistry::new();
    registry.register_pull_adapter(Arc::new(JiraPullAdapter));

    let result = orchestrator
        .run_from_adapter(
            &registry,
            PullSyncRequestInput {
                source_system: "jira".to_string(),
                mode: "incremental".to_string(),
                scope: serde_json::json!({"project_keys": ["ALM"]}),
                reason: Some("adapter pull".to_string()),
            },
        )
        .await?;

    assert_eq!(result.run_status, "completed");
    assert_eq!(result.success_count, 1);
    assert_eq!(result.failure_count, 0);

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
