use backend::config::Settings;
use backend::db::pool::{connect, run_migrations};
use backend::services::normalization::NormalizationPipeline;
use backend::services::project_write::ProjectWriteService;
use backend::services::raw_ingestion::{CreateRawIngestionEventInput, RawIngestionRepository};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::str::FromStr;
use std::time::Instant;
use uuid::Uuid;

#[tokio::test]
async fn bench_project_write_n_plus_1() -> anyhow::Result<()> {
    let Some(test_db) = TestDatabase::create().await? else {
        eprintln!("skip bench: ALM_BACKEND_TEST_DATABASE_ADMIN_URL not set");
        return Ok(());
    };

    let pool = connect_and_migrate(&test_db).await?;
    let raw_repository = RawIngestionRepository::new(pool.clone());
    let normalization_pipeline = NormalizationPipeline::new(pool.clone());

    // Create master data
    let mut org_codes = vec![];
    let mut emp_numbers = vec![];

    let mut raw_records = vec![];

    // Create 5000 projects
    for i in 0..5000 {
        let org_code = format!("org_{}", i % 100);
        let emp_num = format!("emp_{}", i % 100);
        org_codes.push(org_code.clone());
        emp_numbers.push(emp_num.clone());

        let project_record = raw_repository
            .create(CreateRawIngestionEventInput {
                source_system: "test_sys".to_string(),
                source_object_type: "project".to_string(),
                source_object_id: format!("PROJ-{}", i),
                source_event_key: format!("test_sys-project-PROJ-{}", i),
                source_version: Some("1".to_string()),
                source_updated_at: Some("2026-04-07T08:55:00Z".to_string()),
                payload: serde_json::json!({
                    "name": format!("Project {}", i),
                    "owning_organization_code": org_code,
                    "project_owner_employee_number": emp_num,
                    "description": format!("Desc {}", i)
                }),
            })
            .await?;
        raw_records.push(project_record);
    }

    normalization_pipeline.normalize_pending(10000).await?;

    let run_id: Uuid = sqlx::query_scalar(
        "select integration_run_id from integration_run where external_run_id = $1",
    )
    .bind(&raw_records[0].run_id)
    .fetch_one(&pool)
    .await?;

    let project_write_service = ProjectWriteService::new(pool.clone());

    let start = Instant::now();
    project_write_service.apply_for_run(run_id).await?;
    let duration = start.elapsed();

    println!(
        "Benchmark apply_for_run (5000 records) took: {:?}",
        duration
    );

    Ok(())
}

async fn connect_and_migrate(test_db: &TestDatabase) -> anyhow::Result<PgPool> {
    let settings = Settings {
        bind_address: "127.0.0.1:8080".to_string(),
        database_url: test_db.database_url(),
        database_max_connections: 50,
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
        let database_name = format!("alm_bench_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("create database {database_name}"))
            .execute(&admin_pool)
            .await?;
        let database_url = admin_options
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
            let _ = sqlx::query(&format!("select pg_terminate_backend(pid) from pg_stat_activity where datname = '{database_name}' and pid <> pg_backend_pid()")).execute(&admin_pool).await;
            let _ = sqlx::query(&format!("drop database if exists {database_name}"))
                .execute(&admin_pool)
                .await;
            admin_pool.close().await;
        });
    }
}
