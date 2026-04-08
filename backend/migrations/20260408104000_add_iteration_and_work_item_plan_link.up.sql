create table iteration (
  iteration_id uuid primary key,
  project_id uuid not null,
  name varchar(100) not null,
  goal text null,
  status varchar(30) not null,
  start_date date null,
  end_date date null,
  capacity numeric(12,2) null,
  sequence_no integer null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_iteration_project_name unique (project_id, name),
  constraint fk_iteration_project
    foreign key (project_id)
    references project (project_id)
);

create table work_item_plan_link (
  work_item_plan_link_id uuid primary key,
  work_item_id uuid not null,
  plan_type varchar(30) not null,
  plan_id uuid not null,
  link_role varchar(30) not null,
  sequence_no integer null,
  is_primary boolean not null default false,
  linked_by_rule_ref varchar(100) null,
  effective_from timestamptz null,
  effective_to timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_work_item_plan_link_ref
    unique (work_item_id, plan_type, plan_id, link_role),
  constraint fk_work_item_plan_link_work_item
    foreign key (work_item_id)
    references work_item (work_item_id),
  constraint ck_work_item_plan_link_plan_type
    check (plan_type in ('iteration', 'release', 'milestone', 'wbs_node'))
);

create index ix_work_item_plan_link_work_item
  on work_item_plan_link (work_item_id);

create index ix_work_item_plan_link_plan_ref
  on work_item_plan_link (plan_type, plan_id);
