use std::str::FromStr;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::app_state::AppState;
use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::http::router::build_router;
use backend::services::pull_sync::{PullRecordInput, PullSyncOrchestrator, PullSyncRunInput};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn domain_read_api_lists_projects_and_work_items_with_master_references() -> anyhow::Result<()>
{
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip domain_read_api_lists_projects_and_work_items_with_master_references: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    seed_domain_data(&pool).await?;
    let app = build_router(AppState::with_pool(pool.clone()));

    let project_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/projects?owning_organization_code=platform")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(project_response.status(), StatusCode::OK);
    let project_body = axum::body::to_bytes(project_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let project_json: serde_json::Value = serde_json::from_slice(&project_body).unwrap();
    let project_items = project_json["items"].as_array().unwrap();

    assert_eq!(project_items.len(), 1);
    assert_eq!(project_items[0]["project_code"], "OPS");
    assert_eq!(project_items[0]["owning_organization_code"], "platform");
    assert_eq!(project_items[0]["project_owner_employee_number"], "E9001");

    let work_item_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/work-items?project_code=OPS&assignee_employee_number=E9001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(work_item_response.status(), StatusCode::OK);
    let work_item_body = axum::body::to_bytes(work_item_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let work_item_json: serde_json::Value = serde_json::from_slice(&work_item_body).unwrap();
    let work_item_items = work_item_json["items"].as_array().unwrap();

    assert_eq!(work_item_items.len(), 2);
    assert_eq!(work_item_items[0]["project_code"], "OPS");
    assert_eq!(work_item_items[0]["owning_organization_code"], "platform");
    assert_eq!(work_item_items[0]["assignee_employee_number"], "E9001");

    Ok(())
}

async fn seed_domain_data(pool: &PgPool) -> anyhow::Result<()> {
    let orchestrator = PullSyncOrchestrator::new(pool.clone());

    orchestrator
        .run(PullSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["OPS"]}),
            reason: Some("domain read seed".to_string()),
            records: vec![
                PullRecordInput {
                    source_object_type: "organization".to_string(),
                    source_object_id: "platform".to_string(),
                    source_event_key: "jira-org-platform".to_string(),
                    source_version: Some("1".to_string()),
                    source_updated_at: Some("2026-04-08T03:00:00Z".to_string()),
                    payload: serde_json::json!({
                        "organization_name": "Platform Center",
                        "organization_status": "active"
                    }),
                },
                PullRecordInput {
                    source_object_type: "workforce".to_string(),
                    source_object_id: "E9001".to_string(),
                    source_event_key: "jira-workforce-e9001".to_string(),
                    source_version: Some("1".to_string()),
                    source_updated_at: Some("2026-04-08T03:01:00Z".to_string()),
                    payload: serde_json::json!({
                        "display_name": "박연계",
                        "employment_status": "active",
                        "primary_organization_code": "platform"
                    }),
                },
                PullRecordInput {
                    source_object_type: "project".to_string(),
                    source_object_id: "OPS".to_string(),
                    source_event_key: "jira-project-ops".to_string(),
                    source_version: Some("1".to_string()),
                    source_updated_at: Some("2026-04-08T03:02:00Z".to_string()),
                    payload: serde_json::json!({
                        "name": "Operations",
                        "owning_organization_code": "platform",
                        "project_owner_employee_number": "E9001"
                    }),
                },
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "OPS-100".to_string(),
                    source_event_key: "jira-issue-ops-100".to_string(),
                    source_version: Some("2".to_string()),
                    source_updated_at: Some("2026-04-08T03:03:00Z".to_string()),
                    payload: serde_json::json!({
                        "summary": "Parent task",
                        "project_key": "OPS",
                        "owning_organization_code": "platform",
                        "assignee_employee_number": "E9001",
                        "reporter_employee_number": "E9001",
                        "status": {
                            "common": "open",
                            "detailed": "new"
                        }
                    }),
                },
                PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: "OPS-101".to_string(),
                    source_event_key: "jira-issue-ops-101".to_string(),
                    source_version: Some("3".to_string()),
                    source_updated_at: Some("2026-04-08T03:04:00Z".to_string()),
                    payload: serde_json::json!({
                        "summary": "Child task after parent",
                        "project_key": "OPS",
                        "parent_key": "OPS-100",
                        "owning_organization_code": "platform",
                        "assignee_employee_number": "E9001",
                        "reporter_employee_number": "E9001",
                        "status": {
                            "common": "in_progress",
                            "detailed": "doing"
                        }
                    }),
                },
            ],
        })
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
