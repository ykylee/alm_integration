create table organization_master (
  organization_id uuid primary key,
  organization_code varchar(50) not null,
  organization_name varchar(200) not null,
  organization_status varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_organization_master_code unique (organization_code)
);

create table work_item_type (
  work_item_type_id uuid primary key,
  type_code varchar(50) not null,
  type_name varchar(100) not null,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_work_item_type_code unique (type_code)
);

create table project (
  project_id uuid primary key,
  project_code varchar(50) not null,
  project_name varchar(200) not null,
  project_type varchar(30) not null,
  project_status varchar(30) not null,
  owning_organization_id uuid not null,
  description text null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_project_code unique (project_code),
  constraint fk_project_owning_organization
    foreign key (owning_organization_id)
    references organization_master (organization_id)
);

create index ix_project_status
  on project (project_status);

create table work_item (
  work_item_id uuid primary key,
  project_id uuid not null,
  work_item_type_id uuid not null,
  work_item_key varchar(50) not null,
  title varchar(300) not null,
  description text null,
  current_common_status varchar(30) not null,
  current_detailed_status_code varchar(50) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_work_item_project_key unique (project_id, work_item_key),
  constraint fk_work_item_project
    foreign key (project_id)
    references project (project_id),
  constraint fk_work_item_type
    foreign key (work_item_type_id)
    references work_item_type (work_item_type_id)
);

create index ix_work_item_project_id
  on work_item (project_id);

create index ix_work_item_type_id
  on work_item (work_item_type_id);

insert into organization_master (
  organization_id,
  organization_code,
  organization_name,
  organization_status,
  created_at,
  updated_at
)
values (
  '00000000-0000-0000-0000-000000000001',
  'default_org',
  'Default Organization',
  'active',
  now(),
  now()
)
on conflict (organization_code) do nothing;

insert into work_item_type (
  work_item_type_id,
  type_code,
  type_name,
  is_active,
  created_at,
  updated_at
)
values
  (
    '00000000-0000-0000-0000-000000000101',
    'task',
    'Task',
    true,
    now(),
    now()
  ),
  (
    '00000000-0000-0000-0000-000000000102',
    'story',
    'Story',
    true,
    now(),
    now()
  ),
  (
    '00000000-0000-0000-0000-000000000103',
    'bug',
    'Bug',
    true,
    now(),
    now()
  )
on conflict (type_code) do nothing;
