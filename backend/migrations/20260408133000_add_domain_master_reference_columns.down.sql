drop index if exists ix_work_item_reporter_workforce_id;
drop index if exists ix_work_item_assignee_workforce_id;
drop index if exists ix_work_item_owning_organization_id;

alter table work_item
  drop constraint if exists fk_work_item_reporter_workforce;

alter table work_item
  drop constraint if exists fk_work_item_assignee_workforce;

alter table work_item
  drop constraint if exists fk_work_item_owning_organization;

alter table work_item
  drop column if exists reporter_workforce_id,
  drop column if exists assignee_workforce_id,
  drop column if exists owning_organization_id;

drop index if exists ix_project_owner_workforce_id;

alter table project
  drop constraint if exists fk_project_owner_workforce;

alter table project
  drop column if exists project_owner_workforce_id;
