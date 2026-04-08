create table work_item_hierarchy (
  work_item_hierarchy_id uuid primary key,
  parent_work_item_id uuid not null,
  child_work_item_id uuid not null,
  relationship_type varchar(30) not null,
  sequence_no integer null,
  created_at timestamptz not null,
  constraint ux_work_item_hierarchy_child unique (child_work_item_id),
  constraint fk_work_item_hierarchy_parent
    foreign key (parent_work_item_id)
    references work_item (work_item_id),
  constraint fk_work_item_hierarchy_child
    foreign key (child_work_item_id)
    references work_item (work_item_id),
  constraint ck_work_item_hierarchy_not_self
    check (parent_work_item_id <> child_work_item_id)
);

create index ix_work_item_hierarchy_parent
  on work_item_hierarchy (parent_work_item_id);
