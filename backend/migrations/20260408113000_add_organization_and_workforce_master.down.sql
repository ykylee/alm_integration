drop index if exists ix_workforce_master_employment_status;
drop index if exists ix_workforce_master_primary_organization;
drop table if exists workforce_master;

drop index if exists ix_organization_master_status;
drop index if exists ix_organization_master_parent;

alter table organization_master
  drop constraint if exists fk_organization_master_parent;

alter table organization_master
  drop column if exists effective_to,
  drop column if exists effective_from,
  drop column if exists parent_organization_id;
