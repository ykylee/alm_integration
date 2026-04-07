create table integration_job (
  integration_job_id uuid primary key,
  integration_system_id uuid null,
  integration_endpoint_id uuid null,
  job_code varchar(100) not null,
  job_name varchar(200) not null,
  job_type varchar(30) not null,
  schedule_expression varchar(100) null,
  job_status varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_integration_job_code unique (job_code)
);

create table integration_run (
  integration_run_id uuid primary key,
  integration_job_id uuid not null,
  queued_at timestamptz not null,
  started_at timestamptz null,
  ended_at timestamptz null,
  run_status varchar(30) not null,
  status_reason_code varchar(50) null,
  status_reason_message text null,
  cancel_requested_at timestamptz null,
  cancel_requested_by varchar(100) null,
  cancel_reason_code varchar(50) null,
  processed_count integer not null default 0,
  success_count integer not null default 0,
  failure_count integer not null default 0,
  pending_count integer not null default 0,
  triggered_by varchar(100) null,
  retry_of_run_id uuid null,
  created_at timestamptz not null,
  constraint fk_integration_run_job
    foreign key (integration_job_id)
    references integration_job (integration_job_id),
  constraint fk_integration_run_retry_of
    foreign key (retry_of_run_id)
    references integration_run (integration_run_id),
  constraint ck_integration_run_status
    check (run_status in ('queued', 'running', 'partially_completed', 'completed', 'failed', 'cancelled'))
);

create index ix_integration_run_job_id
  on integration_run (integration_job_id);

create index ix_integration_run_status
  on integration_run (run_status);

create index ix_integration_run_started_at
  on integration_run (started_at);

create index ix_integration_run_cancel_requested_at
  on integration_run (cancel_requested_at);

create index ix_integration_run_retry_of_run_id
  on integration_run (retry_of_run_id);

create table raw_ingestion_event (
  raw_ingestion_event_id uuid primary key,
  integration_run_id uuid not null,
  source_system varchar(50) not null,
  source_object_type varchar(50) not null,
  source_object_id varchar(200) not null,
  source_event_key varchar(200) not null,
  source_version varchar(100) null,
  source_sequence_no bigint null,
  source_updated_at timestamptz null,
  source_record_key varchar(200) null,
  payload_reference text not null,
  payload_hash varchar(100) not null,
  ingested_at timestamptz not null,
  normalization_status varchar(30) not null,
  created_at timestamptz not null,
  constraint fk_raw_ingestion_event_run
    foreign key (integration_run_id)
    references integration_run (integration_run_id),
  constraint ux_raw_ingestion_event_idempotency
    unique (source_system, source_object_type, source_object_id, source_event_key)
);

create index ix_raw_ingestion_event_run_id
  on raw_ingestion_event (integration_run_id);

create index ix_raw_ingestion_event_normalization_status
  on raw_ingestion_event (normalization_status);
