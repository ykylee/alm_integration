create table integration_system (
  integration_system_id uuid primary key,
  system_code varchar(50) not null,
  system_name varchar(100) not null,
  system_type varchar(50) not null,
  authentication_type varchar(30) not null,
  connection_status varchar(30) not null,
  owner_team varchar(100) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_integration_system_code unique (system_code)
);

create table integration_endpoint (
  integration_endpoint_id uuid primary key,
  integration_system_id uuid not null,
  endpoint_type varchar(30) not null,
  endpoint_name varchar(100) not null,
  base_url varchar(500) not null,
  resource_path varchar(500) null,
  request_method varchar(20) null,
  credential_binding_mode varchar(30) not null,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_integration_endpoint_name
    unique (integration_system_id, endpoint_name),
  constraint fk_integration_endpoint_system
    foreign key (integration_system_id)
    references integration_system (integration_system_id)
);

create index ix_integration_endpoint_system_id
  on integration_endpoint (integration_system_id);

create index ix_integration_endpoint_active
  on integration_endpoint (is_active);

create table integration_credential (
  integration_credential_id uuid primary key,
  integration_system_id uuid not null,
  integration_endpoint_id uuid not null,
  credential_type varchar(50) not null,
  principal_id varchar(200) null,
  secret_ciphertext text not null,
  secret_key_version varchar(50) not null,
  secret_fingerprint varchar(100) null,
  rotation_status varchar(30) not null,
  effective_from timestamptz not null,
  effective_to timestamptz null,
  last_validated_at timestamptz null,
  last_updated_by varchar(100) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint fk_integration_credential_system
    foreign key (integration_system_id)
    references integration_system (integration_system_id),
  constraint fk_integration_credential_endpoint
    foreign key (integration_endpoint_id)
    references integration_endpoint (integration_endpoint_id)
);

create index ix_integration_credential_endpoint_id
  on integration_credential (integration_endpoint_id);

create index ix_integration_credential_effective_period
  on integration_credential (effective_from, effective_to);
