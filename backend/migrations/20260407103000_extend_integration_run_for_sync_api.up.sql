alter table integration_run
  add column external_run_id varchar(100) null,
  add column source_system varchar(50) null,
  add column run_mode varchar(30) null,
  add column run_scope jsonb null default '{}'::jsonb,
  add column reason text null;

update integration_run
set
  external_run_id = coalesce(external_run_id, 'sync_' || replace(integration_run_id::text, '-', '')),
  source_system = coalesce(source_system, 'unknown'),
  run_mode = coalesce(run_mode, 'manual'),
  run_scope = coalesce(run_scope, '{}'::jsonb);

alter table integration_run
  alter column external_run_id set not null,
  alter column source_system set not null,
  alter column run_mode set not null,
  alter column run_scope set not null;

alter table integration_run
  add constraint ux_integration_run_external_run_id unique (external_run_id);

create index ix_integration_run_source_system
  on integration_run (source_system);
