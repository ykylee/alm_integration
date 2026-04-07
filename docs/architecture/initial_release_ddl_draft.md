# 초기 릴리스 DDL 초안

- 문서 목적: 초기 릴리스 필수 엔터티 중 핵심 축에 대해 `DDL` 스타일의 테이블 생성 초안을 제공한다.
- 범위: 프로젝트, 업무 항목, 프로세스 모델, 상태 모델, 계획 모델, 역할/감사 영역의 대표 테이블 초안
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, `DBA`
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/initial_release_physical_model_draft.md`, `docs/architecture/domain_entity_definition_draft.md`, `docs/architecture/logical_reference_rules_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 초기 릴리스 물리 모델 초안: [./initial_release_physical_model_draft.md](./initial_release_physical_model_draft.md)
- 초기 릴리스 `ERD`: [./initial_release_erd_draft.md](./initial_release_erd_draft.md)

## 1. 문서 사용 원칙

- 본 문서는 특정 `RDBMS` 최종 문법이 아니라 `PostgreSQL` 계열을 가정한 준-표준 `DDL` 초안이다.
- 실제 구현 시 `uuid` 생성 함수, partial index, exclusion constraint 같은 세부 문법은 선택한 `RDBMS`에 맞게 조정한다.
- 다형 참조는 정규 외래키로 닫지 않고 `CHECK`, 인덱스, 애플리케이션 검증을 조합하는 방향을 유지한다.
- 전체 테이블을 한 번에 모두 내리지 않고, 초기 릴리스 필수 범위 중 우선 구현 축만 먼저 다룬다.

## 2. 공통 가정

```sql
-- 공통 가정
-- 1) 모든 PK는 uuid 사용
-- 2) created_at/updated_at는 timestamptz 사용
-- 3) 코드값은 varchar(50) 내외로 시작
-- 4) 설명성 텍스트는 text 사용
```

## 3. 프로젝트 및 업무 항목

### 3.1 `organization_master`

```sql
create table organization_master (
  organization_id uuid primary key,
  organization_code varchar(50) not null,
  organization_name varchar(200) not null,
  parent_organization_id uuid null,
  organization_status varchar(30) not null,
  effective_from timestamptz null,
  effective_to timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_organization_master_code unique (organization_code),
  constraint fk_organization_master_parent
    foreign key (parent_organization_id)
    references organization_master (organization_id)
);

create index ix_organization_master_parent
  on organization_master (parent_organization_id);
```

### 3.2 `workforce_master`

```sql
create table workforce_master (
  workforce_id uuid primary key,
  employee_number varchar(50) not null,
  display_name varchar(200) not null,
  employment_status varchar(30) not null,
  primary_organization_id uuid not null,
  job_family varchar(100) null,
  email varchar(200) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_workforce_master_employee_number unique (employee_number),
  constraint ux_workforce_master_email unique (email),
  constraint fk_workforce_master_primary_organization
    foreign key (primary_organization_id)
    references organization_master (organization_id)
);

create index ix_workforce_master_primary_organization
  on workforce_master (primary_organization_id);
```

### 3.3 `project`

```sql
create table project (
  project_id uuid primary key,
  project_code varchar(50) not null,
  project_name varchar(200) not null,
  project_type varchar(30) not null,
  project_status varchar(30) not null,
  owning_organization_id uuid not null,
  project_owner_workforce_id uuid not null,
  start_date date null,
  target_end_date date null,
  actual_end_date date null,
  description text null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_project_code unique (project_code),
  constraint fk_project_owning_organization
    foreign key (owning_organization_id)
    references organization_master (organization_id),
  constraint fk_project_owner_workforce
    foreign key (project_owner_workforce_id)
    references workforce_master (workforce_id)
);

create index ix_project_status
  on project (project_status);

create index ix_project_owning_organization_id
  on project (owning_organization_id);
```

### 3.4 `work_item_type`

```sql
create table work_item_type (
  work_item_type_id uuid primary key,
  type_code varchar(50) not null,
  type_name varchar(100) not null,
  is_hierarchical boolean not null default true,
  default_hierarchy_level integer null,
  is_plannable boolean not null default true,
  is_release_scoped boolean not null default false,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_work_item_type_code unique (type_code)
);
```

### 3.5 `work_item`

```sql
create table work_item (
  work_item_id uuid primary key,
  project_id uuid not null,
  work_item_type_id uuid not null,
  work_item_key varchar(50) not null,
  title varchar(300) not null,
  description text null,
  priority varchar(30) null,
  current_common_status varchar(30) not null,
  current_detailed_status_code varchar(50) not null,
  owning_organization_id uuid not null,
  assignee_workforce_id uuid null,
  reporter_workforce_id uuid null,
  planned_start_at timestamptz null,
  planned_end_at timestamptz null,
  actual_start_at timestamptz null,
  actual_end_at timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_work_item_project_key unique (project_id, work_item_key),
  constraint fk_work_item_project
    foreign key (project_id)
    references project (project_id),
  constraint fk_work_item_type
    foreign key (work_item_type_id)
    references work_item_type (work_item_type_id),
  constraint fk_work_item_owning_organization
    foreign key (owning_organization_id)
    references organization_master (organization_id),
  constraint fk_work_item_assignee_workforce
    foreign key (assignee_workforce_id)
    references workforce_master (workforce_id),
  constraint fk_work_item_reporter_workforce
    foreign key (reporter_workforce_id)
    references workforce_master (workforce_id)
);

create index ix_work_item_project_id
  on work_item (project_id);

create index ix_work_item_type_id
  on work_item (work_item_type_id);

create index ix_work_item_common_status
  on work_item (current_common_status);

create index ix_work_item_project_status
  on work_item (project_id, current_common_status, current_detailed_status_code);

create index ix_work_item_assignee_workforce_id
  on work_item (assignee_workforce_id);
```

### 3.6 `work_item_hierarchy`

```sql
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
```

## 4. 프로세스 모델 및 상태 모델

### 4.1 `process_model_definition`

```sql
create table process_model_definition (
  process_model_definition_id uuid primary key,
  model_code varchar(50) not null,
  model_name varchar(100) not null,
  model_category varchar(50) null,
  description text null,
  is_builtin boolean not null default false,
  is_active boolean not null default true,
  version varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_process_model_definition_code unique (model_code)
);
```

### 4.2 `workflow_scheme`

```sql
create table workflow_scheme (
  workflow_scheme_id uuid primary key,
  process_model_definition_id uuid not null,
  scheme_code varchar(50) not null,
  scheme_name varchar(100) not null,
  description text null,
  initial_status_code varchar(50) not null,
  is_default boolean not null default false,
  version varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_workflow_scheme_code_version
    unique (process_model_definition_id, scheme_code, version),
  constraint fk_workflow_scheme_process_model
    foreign key (process_model_definition_id)
    references process_model_definition (process_model_definition_id)
);

create index ix_workflow_scheme_process_model
  on workflow_scheme (process_model_definition_id);
```

## 5. 연계 및 수집

### 5.1 `integration_system`

```sql
create table integration_system (
  integration_system_id uuid primary key,
  system_code varchar(50) not null,
  system_name varchar(100) not null,
  system_type varchar(50) not null,
  authentication_type varchar(50) not null,
  connection_status varchar(30) not null,
  owner_team varchar(100) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_integration_system_code unique (system_code)
);

create index ix_integration_system_type
  on integration_system (system_type);

create index ix_integration_system_status
  on integration_system (connection_status);
```

### 5.2 `integration_endpoint`

```sql
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
```

### 5.3 `integration_credential`

```sql
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

create index ix_integration_credential_rotation_status
  on integration_credential (rotation_status);

create index ix_integration_credential_effective_period
  on integration_credential (effective_from, effective_to);
```

### 5.4 `integration_job`

```sql
create table integration_job (
  integration_job_id uuid primary key,
  integration_system_id uuid not null,
  job_code varchar(50) not null,
  job_name varchar(100) not null,
  trigger_type varchar(30) not null,
  schedule_expression varchar(100) null,
  job_status varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_integration_job_code
    unique (integration_system_id, job_code),
  constraint fk_integration_job_system
    foreign key (integration_system_id)
    references integration_system (integration_system_id)
);
```

### 5.5 `integration_run`

```sql
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
```

### 5.6 `raw_ingestion_event`

```sql
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
```

### 5.7 `normalized_record_reference`

```sql
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
```

### 5.8 `sync_error`

```sql
create table sync_error (
  sync_error_id uuid primary key,
  integration_run_id uuid not null,
  error_code varchar(50) not null,
  error_message text not null,
  error_type varchar(50) not null,
  retry_status varchar(30) not null,
  resolved_at timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint fk_sync_error_run
    foreign key (integration_run_id)
    references integration_run (integration_run_id)
);

create index ix_sync_error_run_id
  on sync_error (integration_run_id);

create index ix_sync_error_retry_status
  on sync_error (retry_status);
```

### 4.3 `workflow_status_definition`

```sql
create table workflow_status_definition (
  workflow_status_definition_id uuid primary key,
  workflow_scheme_id uuid not null,
  status_code varchar(50) not null,
  status_name varchar(100) not null,
  mapped_common_status varchar(30) not null,
  sequence_no integer null,
  is_terminal boolean not null default false,
  created_at timestamptz not null,
  constraint ux_workflow_status_definition_code
    unique (workflow_scheme_id, status_code),
  constraint fk_workflow_status_definition_scheme
    foreign key (workflow_scheme_id)
    references workflow_scheme (workflow_scheme_id)
);
```

### 4.4 `workflow_transition_definition`

```sql
create table workflow_transition_definition (
  workflow_transition_definition_id uuid primary key,
  workflow_scheme_id uuid not null,
  from_status_code varchar(50) not null,
  to_status_code varchar(50) not null,
  transition_name varchar(100) null,
  requires_approval boolean not null default false,
  is_active boolean not null default true,
  created_at timestamptz not null,
  constraint ux_workflow_transition_definition_path
    unique (workflow_scheme_id, from_status_code, to_status_code),
  constraint fk_workflow_transition_definition_scheme
    foreign key (workflow_scheme_id)
    references workflow_scheme (workflow_scheme_id),
  constraint ck_workflow_transition_definition_not_self
    check (from_status_code <> to_status_code)
);
```

### 4.5 `work_item_status_history`

```sql
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
    references work_item (work_item_id),
  constraint fk_work_item_status_history_transition
    foreign key (workflow_transition_definition_id)
    references workflow_transition_definition (workflow_transition_definition_id)
);

create index ix_work_item_status_history_work_item_changed_at
  on work_item_status_history (work_item_id, changed_at desc);
```

## 5. 계획 모델

### 5.1 `planning_scheme`

```sql
create table planning_scheme (
  planning_scheme_id uuid primary key,
  process_model_definition_id uuid not null,
  scheme_code varchar(50) not null,
  scheme_name varchar(100) not null,
  default_plan_unit_type varchar(30) not null,
  uses_iteration boolean not null default false,
  uses_release boolean not null default false,
  uses_milestone boolean not null default false,
  uses_wbs boolean not null default false,
  allows_parallel_tracks boolean not null default false,
  is_default boolean not null default false,
  version varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_planning_scheme_code_version
    unique (process_model_definition_id, scheme_code, version),
  constraint fk_planning_scheme_process_model
    foreign key (process_model_definition_id)
    references process_model_definition (process_model_definition_id)
);
```

### 5.2 `view_scheme`

```sql
create table view_scheme (
  view_scheme_id uuid primary key,
  process_model_definition_id uuid not null,
  scheme_code varchar(50) not null,
  scheme_name varchar(100) not null,
  default_board_type varchar(30) null,
  supports_gantt boolean not null default false,
  supports_wbs boolean not null default false,
  supports_release_board boolean not null default false,
  default_grouping_rule varchar(50) null,
  default_sort_rule varchar(50) null,
  version varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_view_scheme_code_version
    unique (process_model_definition_id, scheme_code, version),
  constraint fk_view_scheme_process_model
    foreign key (process_model_definition_id)
    references process_model_definition (process_model_definition_id)
);
```

### 5.3 `project_process_model`

```sql
create table project_process_model (
  project_process_model_id uuid primary key,
  project_id uuid not null,
  process_model_definition_id uuid not null,
  workflow_scheme_id uuid not null,
  planning_scheme_id uuid not null,
  view_scheme_id uuid not null,
  assignment_scope varchar(30) not null,
  is_primary boolean not null default false,
  effective_from timestamptz not null,
  effective_to timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_project_process_model_effective
    unique (project_id, process_model_definition_id, effective_from),
  constraint fk_project_process_model_project
    foreign key (project_id)
    references project (project_id),
  constraint fk_project_process_model_definition
    foreign key (process_model_definition_id)
    references process_model_definition (process_model_definition_id),
  constraint fk_project_process_model_workflow_scheme
    foreign key (workflow_scheme_id)
    references workflow_scheme (workflow_scheme_id),
  constraint fk_project_process_model_planning_scheme
    foreign key (planning_scheme_id)
    references planning_scheme (planning_scheme_id),
  constraint fk_project_process_model_view_scheme
    foreign key (view_scheme_id)
    references view_scheme (view_scheme_id)
);

create index ix_project_process_model_project
  on project_process_model (project_id);

create index ix_project_process_model_primary
  on project_process_model (project_id, is_primary, effective_from);
```

### 5.4 `iteration`

```sql
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
```

### 5.5 `release`

```sql
create table release (
  release_id uuid primary key,
  project_id uuid not null,
  release_code varchar(50) not null,
  release_name varchar(100) not null,
  status varchar(30) not null,
  planned_release_at timestamptz null,
  actual_release_at timestamptz null,
  description text null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_release_project_code unique (project_id, release_code),
  constraint fk_release_project
    foreign key (project_id)
    references project (project_id)
);
```

### 5.6 `milestone`

```sql
create table milestone (
  milestone_id uuid primary key,
  project_id uuid not null,
  milestone_code varchar(50) not null,
  milestone_name varchar(100) not null,
  milestone_type varchar(30) not null,
  target_at timestamptz null,
  actual_at timestamptz null,
  status varchar(30) not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_milestone_project_code unique (project_id, milestone_code),
  constraint fk_milestone_project
    foreign key (project_id)
    references project (project_id)
);
```

### 5.7 `work_item_plan_link`

```sql
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
```

## 6. 권한 및 감사

### 6.1 `role_policy`

```sql
create table role_policy (
  role_policy_id uuid primary key,
  role_code varchar(50) not null,
  role_name varchar(100) not null,
  description text null,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_role_policy_code unique (role_code)
);
```

### 6.2 `permission_scope`

```sql
create table permission_scope (
  permission_scope_id uuid primary key,
  role_policy_id uuid not null,
  resource_type varchar(50) not null,
  action_code varchar(50) not null,
  scope_code varchar(50) not null,
  constraint_expression text null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_permission_scope_rule
    unique (role_policy_id, resource_type, action_code, scope_code),
  constraint fk_permission_scope_role_policy
    foreign key (role_policy_id)
    references role_policy (role_policy_id)
);
```

### 6.3 `role_assignment`

```sql
create table role_assignment (
  role_assignment_id uuid primary key,
  subject_type varchar(30) not null,
  subject_id uuid not null,
  role_type varchar(50) not null,
  assignee_workforce_id uuid not null,
  effective_from timestamptz not null,
  effective_to timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_role_assignment_subject
    unique (subject_type, subject_id, role_type, assignee_workforce_id, effective_from),
  constraint fk_role_assignment_assignee_workforce
    foreign key (assignee_workforce_id)
    references workforce_master (workforce_id),
  constraint ck_role_assignment_subject_type
    check (subject_type in ('organization_master', 'project', 'work_item')),
  constraint ck_role_assignment_role_type
    check (role_type in (
      'organization_head',
      'approver',
      'delegate_approver',
      'project_owner',
      'work_item_owner',
      'quality_reviewer'
    ))
);

create index ix_role_assignment_subject_ref
  on role_assignment (subject_type, subject_id);

create index ix_role_assignment_assignee
  on role_assignment (assignee_workforce_id);
```

### 6.4 `audit_log`

```sql
create table audit_log (
  audit_log_id uuid primary key,
  actor_type varchar(30) not null,
  actor_id uuid null,
  target_entity_type varchar(50) not null,
  target_entity_id uuid null,
  event_type varchar(50) not null,
  event_result varchar(30) null,
  event_payload_ref varchar(200) null,
  occurred_at timestamptz not null,
  created_at timestamptz not null,
  constraint ck_audit_log_actor_type
    check (actor_type in ('workforce_master', 'system_account', 'integration_process')),
  constraint ck_audit_log_target_entity_type
    check (target_entity_type in ('project', 'work_item', 'governance_entity'))
);

create index ix_audit_log_actor_ref
  on audit_log (actor_type, actor_id);

create index ix_audit_log_target_ref
  on audit_log (target_entity_type, target_entity_id);

create index ix_audit_log_occurred_at
  on audit_log (occurred_at desc);
```

## 7. 보완 필요 항목

- `project_process_model` 의 동일 시점 `is_primary=true` 1건 제약은 `RDBMS`별 partial unique index 또는 exclusion constraint 검토가 필요하다.
- `work_item_hierarchy` 의 순환 금지는 단순 `CHECK`로 해결되지 않으므로 서비스 로직 또는 재귀 검증이 필요하다.
- `work_item_plan_link`, `role_assignment`, `audit_log` 의 다형 참조 무결성은 트리거 또는 서비스 계층 검증 방식 중 하나를 선택해야 한다.
- `release` 는 일부 `RDBMS`에서 예약어 충돌 가능성이 있어 실제 구현 시 `release_unit` 같은 이름으로 조정할 수 있다.

## 8. 다음 작업 후보

- `DDL` 초안을 특정 `RDBMS` 문법으로 구체화
- partial unique index, exclusion constraint 등 고급 제약 초안 추가
- 다형 참조 무결성 검증 방식을 별도 설계 문서로 분리
