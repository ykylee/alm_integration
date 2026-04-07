create table normalized_record_reference (
  normalized_record_reference_id uuid primary key,
  raw_ingestion_event_id uuid not null,
  target_entity_type varchar(50) not null,
  target_entity_id uuid not null,
  normalization_version varchar(50) not null,
  normalized_at timestamptz not null,
  created_at timestamptz not null,
  constraint ux_normalized_record_reference
    unique (raw_ingestion_event_id, target_entity_type, target_entity_id),
  constraint fk_normalized_record_reference_raw
    foreign key (raw_ingestion_event_id)
    references raw_ingestion_event (raw_ingestion_event_id)
);

create index ix_normalized_record_reference_raw_id
  on normalized_record_reference (raw_ingestion_event_id);

create index ix_normalized_record_reference_target
  on normalized_record_reference (target_entity_type, target_entity_id);
