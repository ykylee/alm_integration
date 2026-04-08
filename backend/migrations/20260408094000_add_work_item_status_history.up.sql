create table work_item_status_history (
  work_item_status_history_id uuid primary key,
  work_item_id uuid not null,
  from_common_status varchar(30) null,
  from_detailed_status_code varchar(50) null,
  to_common_status varchar(30) not null,
  to_detailed_status_code varchar(50) not null,
  workflow_transition_definition_id uuid null,
  changed_by varchar(100) not null,
  changed_at timestamptz not null,
  change_reason text null,
  source_type varchar(30) not null,
  constraint fk_work_item_status_history_work_item
    foreign key (work_item_id)
    references work_item (work_item_id)
);

create index ix_work_item_status_history_work_item_changed_at
  on work_item_status_history (work_item_id, changed_at desc);
