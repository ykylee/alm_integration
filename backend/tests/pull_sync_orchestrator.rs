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
        select
          ir.processed_count,
          ir.success_count,
          count(distinct rie.raw_ingestion_event_id) as raw_count,
          count(distinct nrr.normalized_record_reference_id) as normalized_count
        from integration_run ir
        left join raw_ingestion_event rie on rie.integration_run_id = ir.integration_run_id
        left join normalized_record_reference nrr
          on nrr.raw_ingestion_event_id = rie.raw_ingestion_event_id
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
    let normalized_count: i64 = counts.get(3);

    assert_eq!(processed_count, 2);
    assert_eq!(success_count, 2);
    assert_eq!(raw_count, 2);
    assert_eq!(normalized_count, 2);

    let normalized_status_count: i64 = sqlx::query_scalar(
        r#"
        select count(*)
        from raw_ingestion_event rie
        join integration_run ir on ir.integration_run_id = rie.integration_run_id
        where ir.external_run_id = $1
          and rie.normalization_status = 'normalized'
        "#,
    )
    .bind(&result.run_id)
    .fetch_one(&pool)
    .await?;

    assert_eq!(normalized_status_count, 2);

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

#[tokio::test]
async fn orchestrator_resolves_pending_reference_within_same_run() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip orchestrator_resolves_pending_reference_within_same_run: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let orchestrator = PullSyncOrchestrator::new(pool.clone());

    let result = orchestrator
        .run(PullSyncRunInput {
            source_system: "jira".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"project_keys": ["OPS"]}),
            reason: Some("reference resolution".to_string()),
            records: vec![
                PullRecordInput {
                    source_object_type: "organization".to_string(),
                    source_object_id: "platform".to_string(),
                    source_event_key: "jira-org-platform".to_string(),
                    source_version: Some("1".to_string()),
                    source_updated_at: Some("2026-04-07T08:55:00Z".to_string()),
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
                    source_updated_at: Some("2026-04-07T08:56:00Z".to_string()),
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
                    source_updated_at: Some("2026-04-07T09:00:00Z".to_string()),
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
                    source_updated_at: Some("2026-04-07T09:04:00Z".to_string()),
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
                    source_updated_at: Some("2026-04-07T09:05:00Z".to_string()),
                    payload: serde_json::json!({
                        "summary": "Child task after parent",
                        "parent_key": "OPS-100",
                        "iteration_name": "Sprint 1",
                        "owning_organization_code": "platform",
                        "assignee_employee_number": "E9001",
                        "reporter_employee_number": "E9001",
                        "references": {
                            "missing": ["project:OPS"]
                        },
                        "status": {
                            "common": "in_progress",
                            "detailed": "doing"
                        }
                    }),
                },
            ],
        })
        .await?;

    assert_eq!(result.run_status, "completed");
    assert_eq!(result.success_count, 5);

    let statuses = sqlx::query(
        r#"
        select source_object_id, normalization_status
        from raw_ingestion_event rie
        join integration_run ir on ir.integration_run_id = rie.integration_run_id
        where ir.external_run_id = $1
        order by source_object_id
        "#,
    )
    .bind(&result.run_id)
    .fetch_all(&pool)
    .await?;

    let normalized_statuses: Vec<(String, String)> = statuses
        .into_iter()
        .map(|row| (row.get("source_object_id"), row.get("normalization_status")))
        .collect();

    assert_eq!(
        normalized_statuses,
        vec![
            ("E9001".to_string(), "normalized".to_string()),
            ("OPS".to_string(), "normalized".to_string()),
            ("OPS-100".to_string(), "normalized".to_string()),
            ("OPS-101".to_string(), "normalized".to_string()),
            ("platform".to_string(), "normalized".to_string()),
        ]
    );

    let normalized_count: i64 = sqlx::query_scalar(
        r#"
        select count(*)
        from normalized_record_reference nrr
        join raw_ingestion_event rie on rie.raw_ingestion_event_id = nrr.raw_ingestion_event_id
        join integration_run ir on ir.integration_run_id = rie.integration_run_id
        where ir.external_run_id = $1
        "#,
    )
    .bind(&result.run_id)
    .fetch_one(&pool)
    .await?;

    assert_eq!(normalized_count, 5);

    let domain_counts = sqlx::query(
        r#"
        select
          (select count(*) from organization_master where organization_code in ('default_org', 'platform')) as organization_count,
          (select count(*) from workforce_master where employee_number = 'E9001') as workforce_count,
          (select count(*) from project where project_code = 'OPS') as project_count,
          (select count(*) from work_item where work_item_key in ('OPS-100', 'OPS-101')) as work_item_count
        "#,
    )
    .fetch_one(&pool)
    .await?;

    let organization_count: i64 = domain_counts.get("organization_count");
    let workforce_count: i64 = domain_counts.get("workforce_count");
    let project_count: i64 = domain_counts.get("project_count");
    let work_item_count: i64 = domain_counts.get("work_item_count");

    assert_eq!(organization_count, 2);
    assert_eq!(workforce_count, 1);
    assert_eq!(project_count, 1);
    assert_eq!(work_item_count, 2);

    let history_count: i64 = sqlx::query_scalar(
        r#"
        select count(*)
        from work_item_status_history h
        join work_item w on w.work_item_id = h.work_item_id
        where w.work_item_key in ('OPS-100', 'OPS-101')
        "#,
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(history_count, 2);

    let hierarchy_count: i64 = sqlx::query_scalar(
        r#"
        select count(*)
        from work_item_hierarchy h
        join work_item child on child.work_item_id = h.child_work_item_id
        join work_item parent on parent.work_item_id = h.parent_work_item_id
        where parent.work_item_key = 'OPS-100'
          and child.work_item_key = 'OPS-101'
        "#,
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(hierarchy_count, 1);

    let plan_link_count: i64 = sqlx::query_scalar(
        r#"
        select count(*)
        from work_item_plan_link l
        join work_item w on w.work_item_id = l.work_item_id
        join iteration i on i.iteration_id = l.plan_id
        where w.work_item_key = 'OPS-101'
          and l.plan_type = 'iteration'
          and i.name = 'Sprint 1'
        "#,
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(plan_link_count, 1);

    let project_master_ref_row = sqlx::query(
        r#"
        select
          om.organization_code as owning_organization_code,
          wm.employee_number as project_owner_employee_number
        from project p
        left join organization_master om on om.organization_id = p.owning_organization_id
        left join workforce_master wm on wm.workforce_id = p.project_owner_workforce_id
        where p.project_code = 'OPS'
        "#,
    )
    .fetch_one(&pool)
    .await?;

    let project_owning_organization_code: String =
        project_master_ref_row.get("owning_organization_code");
    let project_owner_employee_number: String =
        project_master_ref_row.get("project_owner_employee_number");

    assert_eq!(project_owning_organization_code, "platform");
    assert_eq!(project_owner_employee_number, "E9001");

    Ok(())
}

#[tokio::test]
async fn orchestrator_applies_organization_and_workforce_master_data() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!(
            "skip orchestrator_applies_organization_and_workforce_master_data: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set"
        );
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let orchestrator = PullSyncOrchestrator::new(pool.clone());

    let result = orchestrator
        .run(PullSyncRunInput {
            source_system: "hr".to_string(),
            mode: "incremental".to_string(),
            scope: serde_json::json!({"organizations": ["platform"]}),
            reason: Some("master data sync".to_string()),
            records: vec![
                PullRecordInput {
                    source_object_type: "organization".to_string(),
                    source_object_id: "platform".to_string(),
                    source_event_key: "hr-org-platform".to_string(),
                    source_version: Some("1".to_string()),
                    source_updated_at: Some("2026-04-08T01:00:00Z".to_string()),
                    payload: serde_json::json!({
                        "organization_name": "Platform Center",
                        "organization_status": "active"
                    }),
                },
                PullRecordInput {
                    source_object_type: "workforce".to_string(),
                    source_object_id: "E9001".to_string(),
                    source_event_key: "hr-workforce-e9001".to_string(),
                    source_version: Some("2".to_string()),
                    source_updated_at: Some("2026-04-08T01:05:00Z".to_string()),
                    payload: serde_json::json!({
                        "display_name": "박연계",
                        "employment_status": "active",
                        "primary_organization_code": "platform",
                        "job_family": "integration_engineering",
                        "email": "integration@example.com"
                    }),
                },
            ],
        })
        .await?;

    assert_eq!(result.run_status, "completed");
    assert_eq!(result.success_count, 2);

    let organization_row = sqlx::query(
        r#"
        select organization_name, organization_status
        from organization_master
        where organization_code = $1
        "#,
    )
    .bind("platform")
    .fetch_one(&pool)
    .await?;

    let organization_name: String = organization_row.get("organization_name");
    let organization_status: String = organization_row.get("organization_status");

    assert_eq!(organization_name, "Platform Center");
    assert_eq!(organization_status, "active");

    let workforce_row = sqlx::query(
        r#"
        select
          wm.employee_number,
          wm.display_name,
          om.organization_code as primary_organization_code
        from workforce_master wm
        join organization_master om on om.organization_id = wm.primary_organization_id
        where wm.employee_number = $1
        "#,
    )
    .bind("E9001")
    .fetch_one(&pool)
    .await?;

    let employee_number: String = workforce_row.get("employee_number");
    let display_name: String = workforce_row.get("display_name");
    let primary_organization_code: String = workforce_row.get("primary_organization_code");

    assert_eq!(employee_number, "E9001");
    assert_eq!(display_name, "박연계");
    assert_eq!(primary_organization_code, "platform");

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
