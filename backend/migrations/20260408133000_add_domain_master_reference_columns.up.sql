alter table project
  add column if not exists project_owner_workforce_id uuid null;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'fk_project_owner_workforce'
  ) then
    alter table project
      add constraint fk_project_owner_workforce
      foreign key (project_owner_workforce_id)
      references workforce_master (workforce_id);
  end if;
end
$$;

create index if not exists ix_project_owner_workforce_id
  on project (project_owner_workforce_id);

alter table work_item
  add column if not exists owning_organization_id uuid null,
  add column if not exists assignee_workforce_id uuid null,
  add column if not exists reporter_workforce_id uuid null;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'fk_work_item_owning_organization'
  ) then
    alter table work_item
      add constraint fk_work_item_owning_organization
      foreign key (owning_organization_id)
      references organization_master (organization_id);
  end if;
end
$$;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'fk_work_item_assignee_workforce'
  ) then
    alter table work_item
      add constraint fk_work_item_assignee_workforce
      foreign key (assignee_workforce_id)
      references workforce_master (workforce_id);
  end if;
end
$$;

do $$
begin
  if not exists (
    select 1
    from pg_constraint
    where conname = 'fk_work_item_reporter_workforce'
  ) then
    alter table work_item
      add constraint fk_work_item_reporter_workforce
      foreign key (reporter_workforce_id)
      references workforce_master (workforce_id);
  end if;
end
$$;

create index if not exists ix_work_item_owning_organization_id
  on work_item (owning_organization_id);

create index if not exists ix_work_item_assignee_workforce_id
  on work_item (assignee_workforce_id);

create index if not exists ix_work_item_reporter_workforce_id
  on work_item (reporter_workforce_id);
