alter table organization_master
  add column if not exists parent_organization_id uuid null,
  add column if not exists effective_from timestamptz null,
  add column if not exists effective_to timestamptz null;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'fk_organization_master_parent'
  ) then
    alter table organization_master
      add constraint fk_organization_master_parent
      foreign key (parent_organization_id)
      references organization_master (organization_id);
  end if;
end
$$;

create index if not exists ix_organization_master_parent
  on organization_master (parent_organization_id);

create index if not exists ix_organization_master_status
  on organization_master (organization_status);

update organization_master
set effective_from = created_at
where effective_from is null;

create table workforce_master (
  workforce_id uuid primary key,
  employee_number varchar(50) not null,
  display_name varchar(120) not null,
  employment_status varchar(30) not null,
  primary_organization_id uuid not null,
  job_family varchar(100) null,
  email varchar(200) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_workforce_master_employee_number unique (employee_number),
  constraint fk_workforce_master_primary_organization
    foreign key (primary_organization_id)
    references organization_master (organization_id)
);

create index ix_workforce_master_primary_organization
  on workforce_master (primary_organization_id);

create index ix_workforce_master_employment_status
  on workforce_master (employment_status);
