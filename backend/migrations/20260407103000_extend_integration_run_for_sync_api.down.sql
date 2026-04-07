drop index if exists ix_integration_run_source_system;

alter table integration_run
  drop constraint if exists ux_integration_run_external_run_id;

alter table integration_run
  drop column if exists reason,
  drop column if exists run_scope,
  drop column if exists run_mode,
  drop column if exists source_system,
  drop column if exists external_run_id;
