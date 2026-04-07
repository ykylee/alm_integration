#[test]
fn integration_runtime_migration_files_exist() {
    let up_sql =
        include_str!("../migrations/20260407032000_create_integration_runtime_tables.up.sql");
    let down_sql =
        include_str!("../migrations/20260407032000_create_integration_runtime_tables.down.sql");
    let sync_api_up_sql =
        include_str!("../migrations/20260407103000_extend_integration_run_for_sync_api.up.sql");
    let sync_api_down_sql =
        include_str!("../migrations/20260407103000_extend_integration_run_for_sync_api.down.sql");
    let connection_up_sql =
        include_str!("../migrations/20260407120000_create_integration_connection_tables.up.sql");
    let connection_down_sql =
        include_str!("../migrations/20260407120000_create_integration_connection_tables.down.sql");

    assert!(up_sql.contains("create table integration_job"));
    assert!(up_sql.contains("create table integration_run"));
    assert!(up_sql.contains("create table raw_ingestion_event"));
    assert!(up_sql.contains("constraint ck_integration_run_status"));
    assert!(down_sql.contains("drop table if exists raw_ingestion_event"));
    assert!(down_sql.contains("drop table if exists integration_run"));
    assert!(down_sql.contains("drop table if exists integration_job"));
    assert!(sync_api_up_sql.contains("add column external_run_id"));
    assert!(sync_api_up_sql.contains("add column source_system"));
    assert!(sync_api_up_sql.contains("add column run_mode"));
    assert!(sync_api_up_sql.contains("add column run_scope jsonb"));
    assert!(sync_api_up_sql.contains("add column reason text"));
    assert!(sync_api_down_sql.contains("drop column if exists external_run_id"));
    assert!(connection_up_sql.contains("create table integration_system"));
    assert!(connection_up_sql.contains("create table integration_endpoint"));
    assert!(connection_up_sql.contains("create table integration_credential"));
    assert!(connection_down_sql.contains("drop table if exists integration_credential"));
}
