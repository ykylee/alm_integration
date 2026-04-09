use std::str::FromStr;

use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::identity_mapping::{IdentityMappingService, UpsertIdentityMappingInput};
use backend::services::normalization::NormalizationPipeline;
use backend::services::organization_write::OrganizationWriteService;
use backend::services::project_write::ProjectWriteService;
use backend::services::raw_ingestion::{CreateRawIngestionEventInput, RawIngestionRepository};
use backend::services::work_item_write::WorkItemWriteService;
use backend::services::workforce_write::WorkforceWriteService;
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
    let organization_write_service = OrganizationWriteService::new(pool.clone());
    let workforce_write_service = WorkforceWriteService::new(pool.clone());
    let project_write_service = ProjectWriteService::new(pool.clone());
    let work_item_write_service = WorkItemWriteService::new(pool.clone());

    let organization_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "hr".to_string(),
            source_object_type: "organization".to_string(),
            source_object_id: "platform".to_string(),
            source_event_key: "hr-org-platform".to_string(),
            source_version: Some("1".to_string()),
            source_updated_at: Some("2026-04-07T08:55:00Z".to_string()),
            payload: serde_json::json!({
                "organization_name": "Platform Center",
                "organization_status": "active"
            }),
        })
        .await?;

    let workforce_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "hr".to_string(),
            source_object_type: "workforce".to_string(),
            source_object_id: "E9001".to_string(),
            source_event_key: "hr-workforce-e9001".to_string(),
            source_version: Some("1".to_string()),
            source_updated_at: Some("2026-04-07T08:56:00Z".to_string()),
            payload: serde_json::json!({
                "display_name": "박연계",
                "employment_status": "active",
                "primary_organization_code": "platform",
                "job_family": "integration_engineering",
                "email": "integration@example.com"
            }),
        })
        .await?;

    let project_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "project".to_string(),
            source_object_id: "OPS".to_string(),
            source_event_key: "jira-project-ops".to_string(),
            source_version: Some("1".to_string()),
            source_updated_at: Some("2026-04-07T09:00:00Z".to_string()),
            payload: serde_json::json!({
                "name": "Operations Platform",
                "owning_organization_code": "platform",
                "project_owner_employee_number": "E9001"
            }),
        })
        .await?;

    normalization_pipeline.normalize_pending(10).await?;

    let organization_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&organization_record.run_id)
    .fetch_one(&pool)
    .await?;

    organization_write_service
        .apply_for_run(organization_run_internal_id)
        .await?;

    let workforce_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&workforce_record.run_id)
    .fetch_one(&pool)
    .await?;

    workforce_write_service
        .apply_for_run(workforce_run_internal_id)
        .await?;

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

    let parent_work_item_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
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
                "issue_type": "task",
                "status": {
                    "common": "open",
                    "detailed": "new"
                }
            }),
        })
        .await?;

    let child_work_item_record = raw_repository
        .create(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: "OPS-101".to_string(),
            source_event_key: "jira-issue-ops-101".to_string(),
            source_version: Some("3".to_string()),
            source_updated_at: Some("2026-04-07T09:05:00Z".to_string()),
            payload: serde_json::json!({
                "summary": "Implement normalized write path",
                "project_key": "OPS",
                "issue_type": "task",
                "parent_key": "OPS-100",
                "iteration_name": "Sprint 1",
                "owning_organization_code": "platform",
                "assignee_employee_number": "E9001",
                "reporter_employee_number": "E9001",
                "status": {
                    "common": "in_progress",
                    "detailed": "doing"
                }
            }),
        })
        .await?;

    normalization_pipeline.normalize_pending(10).await?;

    let work_item_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&parent_work_item_record.run_id)
    .fetch_one(&pool)
    .await?;

    work_item_write_service
        .apply_for_run(work_item_run_internal_id)
        .await?;

    let child_work_item_run_internal_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&child_work_item_record.run_id)
    .fetch_one(&pool)
    .await?;

    work_item_write_service
        .apply_for_run(child_work_item_run_internal_id)
        .await?;

    let row = sqlx::query(
        r#"
        select p.project_code, w.work_item_key, w.title, w.current_common_status, w.current_detailed_status_code
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
    let current_common_status: String = row.get("current_common_status");
    let current_detailed_status_code: String = row.get("current_detailed_status_code");

    assert_eq!(project_code, "OPS");
    assert_eq!(work_item_key, "OPS-101");
    assert_eq!(title, "Implement normalized write path");
    assert_eq!(current_common_status, "in_progress");
    assert_eq!(current_detailed_status_code, "doing");

    let history_row = sqlx::query(
        r#"
        select to_common_status, to_detailed_status_code, source_type
        from work_item_status_history h
        join work_item w on w.work_item_id = h.work_item_id
        where w.work_item_key = $1
        order by changed_at desc
        limit 1
        "#,
    )
    .bind("OPS-101")
    .fetch_one(&pool)
    .await?;

    let to_common_status: String = history_row.get("to_common_status");
    let to_detailed_status_code: String = history_row.get("to_detailed_status_code");
    let source_type: String = history_row.get("source_type");

    assert_eq!(to_common_status, "in_progress");
    assert_eq!(to_detailed_status_code, "doing");
    assert_eq!(source_type, "integration");

    let hierarchy_row = sqlx::query(
        r#"
        select parent.work_item_key as parent_key, child.work_item_key as child_key, h.relationship_type
        from work_item_hierarchy h
        join work_item parent on parent.work_item_id = h.parent_work_item_id
        join work_item child on child.work_item_id = h.child_work_item_id
        where child.work_item_key = $1
        "#,
    )
    .bind("OPS-101")
    .fetch_one(&pool)
    .await?;

    let parent_key: String = hierarchy_row.get("parent_key");
    let child_key: String = hierarchy_row.get("child_key");
    let relationship_type: String = hierarchy_row.get("relationship_type");

    assert_eq!(parent_key, "OPS-100");
    assert_eq!(child_key, "OPS-101");
    assert_eq!(relationship_type, "parent_child");

    let plan_link_row = sqlx::query(
        r#"
        select i.name as iteration_name, l.plan_type, l.link_role, l.is_primary
        from work_item_plan_link l
        join work_item w on w.work_item_id = l.work_item_id
        join iteration i on i.iteration_id = l.plan_id
        where w.work_item_key = $1
        "#,
    )
    .bind("OPS-101")
    .fetch_one(&pool)
    .await?;

    let iteration_name: String = plan_link_row.get("iteration_name");
    let plan_type: String = plan_link_row.get("plan_type");
    let link_role: String = plan_link_row.get("link_role");
    let is_primary: bool = plan_link_row.get("is_primary");

    assert_eq!(iteration_name, "Sprint 1");
    assert_eq!(plan_type, "iteration");
    assert_eq!(link_role, "planned");
    assert!(is_primary);

    let organization_row = sqlx::query(
        r#"
        select organization_code, organization_name, organization_status
        from organization_master
        where organization_code = $1
        "#,
    )
    .bind("platform")
    .fetch_one(&pool)
    .await?;

    let organization_code: String = organization_row.get("organization_code");
    let organization_name: String = organization_row.get("organization_name");
    let organization_status: String = organization_row.get("organization_status");

    assert_eq!(organization_code, "platform");
    assert_eq!(organization_name, "Platform Center");
    assert_eq!(organization_status, "active");

    let workforce_row = sqlx::query(
        r#"
        select
          wm.employee_number,
          wm.display_name,
          wm.employment_status,
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
    let employment_status: String = workforce_row.get("employment_status");
    let primary_organization_code: String = workforce_row.get("primary_organization_code");

    assert_eq!(employee_number, "E9001");
    assert_eq!(display_name, "박연계");
    assert_eq!(employment_status, "active");
    assert_eq!(primary_organization_code, "platform");

    let project_master_ref_row = sqlx::query(
        r#"
        select
          om.organization_code as owning_organization_code,
          wm.employee_number as project_owner_employee_number
        from project p
        left join organization_master om on om.organization_id = p.owning_organization_id
        left join workforce_master wm on wm.workforce_id = p.project_owner_workforce_id
        where p.project_code = $1
        "#,
    )
    .bind("OPS")
    .fetch_one(&pool)
    .await?;

    let project_owning_organization_code: String =
        project_master_ref_row.get("owning_organization_code");
    let project_owner_employee_number: String =
        project_master_ref_row.get("project_owner_employee_number");

    assert_eq!(project_owning_organization_code, "platform");
    assert_eq!(project_owner_employee_number, "E9001");

    let work_item_master_ref_row = sqlx::query(
        r#"
        select
          org.organization_code as owning_organization_code,
          assignee.employee_number as assignee_employee_number,
          reporter.employee_number as reporter_employee_number
        from work_item wi
        left join organization_master org on org.organization_id = wi.owning_organization_id
        left join workforce_master assignee on assignee.workforce_id = wi.assignee_workforce_id
        left join workforce_master reporter on reporter.workforce_id = wi.reporter_workforce_id
        where wi.work_item_key = $1
        "#,
    )
    .bind("OPS-101")
    .fetch_one(&pool)
    .await?;

    let work_item_owning_organization_code: String =
        work_item_master_ref_row.get("owning_organization_code");
    let assignee_employee_number: String = work_item_master_ref_row.get("assignee_employee_number");
    let reporter_employee_number: String = work_item_master_ref_row.get("reporter_employee_number");

    assert_eq!(work_item_owning_organization_code, "platform");
    assert_eq!(assignee_employee_number, "E9001");
    assert_eq!(reporter_employee_number, "E9001");

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
