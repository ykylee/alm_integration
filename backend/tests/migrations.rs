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
    let normalization_up_sql =
        include_str!("../migrations/20260407234000_add_normalized_record_reference.up.sql");
    let normalization_down_sql =
        include_str!("../migrations/20260407234000_add_normalized_record_reference.down.sql");
    let identity_mapping_up_sql =
        include_str!("../migrations/20260408001000_add_identity_mapping.up.sql");
    let identity_mapping_down_sql =
        include_str!("../migrations/20260408001000_add_identity_mapping.down.sql");
    let domain_core_up_sql =
        include_str!("../migrations/20260408013000_add_domain_core_tables.up.sql");
    let domain_core_down_sql =
        include_str!("../migrations/20260408013000_add_domain_core_tables.down.sql");
    let work_item_status_history_up_sql =
        include_str!("../migrations/20260408094000_add_work_item_status_history.up.sql");
    let work_item_status_history_down_sql =
        include_str!("../migrations/20260408094000_add_work_item_status_history.down.sql");
    let work_item_hierarchy_up_sql =
        include_str!("../migrations/20260408101000_add_work_item_hierarchy.up.sql");
    let work_item_hierarchy_down_sql =
        include_str!("../migrations/20260408101000_add_work_item_hierarchy.down.sql");
    let work_item_plan_link_up_sql =
        include_str!("../migrations/20260408104000_add_iteration_and_work_item_plan_link.up.sql");
    let work_item_plan_link_down_sql =
        include_str!("../migrations/20260408104000_add_iteration_and_work_item_plan_link.down.sql");
    let master_data_up_sql =
        include_str!("../migrations/20260408113000_add_organization_and_workforce_master.up.sql");
    let master_data_down_sql =
        include_str!("../migrations/20260408113000_add_organization_and_workforce_master.down.sql");
    let domain_reference_up_sql =
        include_str!("../migrations/20260408133000_add_domain_master_reference_columns.up.sql");
    let domain_reference_down_sql =
        include_str!("../migrations/20260408133000_add_domain_master_reference_columns.down.sql");

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
    assert!(normalization_up_sql.contains("create table normalized_record_reference"));
    assert!(normalization_up_sql.contains("constraint fk_normalized_record_reference_raw"));
    assert!(normalization_down_sql.contains("drop table if exists normalized_record_reference"));
    assert!(identity_mapping_up_sql.contains("create table identity_mapping"));
    assert!(identity_mapping_up_sql.contains("constraint ux_identity_mapping_source"));
    assert!(identity_mapping_down_sql.contains("drop table if exists identity_mapping"));
    assert!(domain_core_up_sql.contains("create table project"));
    assert!(domain_core_up_sql.contains("create table work_item"));
    assert!(domain_core_up_sql.contains("insert into work_item_type"));
    assert!(domain_core_down_sql.contains("drop table if exists work_item"));
    assert!(work_item_status_history_up_sql.contains("create table work_item_status_history"));
    assert!(work_item_status_history_up_sql.contains("fk_work_item_status_history_work_item"));
    assert!(
        work_item_status_history_down_sql.contains("drop table if exists work_item_status_history")
    );
    assert!(work_item_hierarchy_up_sql.contains("create table work_item_hierarchy"));
    assert!(work_item_hierarchy_up_sql.contains("ux_work_item_hierarchy_child"));
    assert!(work_item_hierarchy_down_sql.contains("drop table if exists work_item_hierarchy"));
    assert!(work_item_plan_link_up_sql.contains("create table iteration"));
    assert!(work_item_plan_link_up_sql.contains("create table work_item_plan_link"));
    assert!(work_item_plan_link_up_sql.contains("ck_work_item_plan_link_plan_type"));
    assert!(work_item_plan_link_down_sql.contains("drop table if exists work_item_plan_link"));
    assert!(master_data_up_sql.contains("alter table organization_master"));
    assert!(master_data_up_sql.contains("create table workforce_master"));
    assert!(master_data_up_sql.contains("fk_workforce_master_primary_organization"));
    assert!(master_data_down_sql.contains("drop table if exists workforce_master"));
    assert!(
        master_data_down_sql.contains("drop constraint if exists fk_organization_master_parent")
    );
    assert!(
        domain_reference_up_sql.contains("add column if not exists project_owner_workforce_id")
    );
    assert!(domain_reference_up_sql.contains("add column if not exists owning_organization_id"));
    assert!(domain_reference_up_sql.contains("fk_work_item_assignee_workforce"));
    assert!(domain_reference_down_sql.contains("drop column if exists project_owner_workforce_id"));
    assert!(domain_reference_down_sql.contains("drop column if exists reporter_workforce_id"));
}
