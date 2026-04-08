create table organization_change_log (
  log_id uuid primary key,
  organization_code varchar(100) not null,
  action_type varchar(40) not null,
  summary text not null,
  changed_at timestamptz not null default now()
);

create index ix_organization_change_log_organization_code
  on organization_change_log (organization_code, changed_at desc);

create table workforce_change_log (
  log_id uuid primary key,
  employee_number varchar(100) not null,
  action_type varchar(40) not null,
  from_organization_code varchar(100),
  to_organization_code varchar(100),
  summary text not null,
  changed_at timestamptz not null default now()
);

create index ix_workforce_change_log_employee_number
  on workforce_change_log (employee_number, changed_at desc);

create index ix_workforce_change_log_organization_scope
  on workforce_change_log (to_organization_code, from_organization_code, changed_at desc);
