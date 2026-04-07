create table identity_mapping (
  identity_mapping_id uuid primary key,
  source_system_code varchar(50) not null,
  source_identity_key varchar(255) not null,
  internal_entity_type varchar(50) not null,
  internal_entity_id uuid not null,
  mapping_status varchar(30) not null,
  verified_at timestamptz,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_identity_mapping_source
    unique (source_system_code, source_identity_key, internal_entity_type),
  constraint ck_identity_mapping_status
    check (mapping_status in ('active', 'verified', 'superseded', 'inactive'))
);

create index ix_identity_mapping_internal
  on identity_mapping (internal_entity_type, internal_entity_id);
