# 초기 릴리스 물리 모델 초안

- 문서 목적: 초기 릴리스 필수 엔터티를 기준으로 물리 모델 관점의 저장 구조, 키, 제약, 인덱스 초안을 정리한다.
- 범위: 초기 릴리스 필수 엔터티의 물리 저장 기준, 직접 참조/다형 참조 처리 원칙, 초기 인덱스 전략
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, `DBA`
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/domain_entity_definition_draft.md`, `docs/architecture/initial_release_erd_draft.md`, `docs/architecture/logical_reference_rules_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 엔터티 정의 초안: [./domain_entity_definition_draft.md](./domain_entity_definition_draft.md)
- 초기 릴리스 `ERD`: [./initial_release_erd_draft.md](./initial_release_erd_draft.md)

## 1. 문서 사용 원칙

- 본 문서는 특정 `RDBMS` 문법 확정 전 단계의 물리 모델 초안이다.
- 컬럼명, 키, 제약, 인덱스 방향을 우선 정리하고 타입은 범주 수준으로만 정의한다.
- 다형 참조는 물리 `FK` 대신 코드값 검증, 애플리케이션 검증, 보조 인덱스로 관리하는 방향을 기본으로 한다.
- 읽기 모델은 본 문서 범위에서 제외하고, 원천 저장 모델만 다룬다.
- 초기 릴리스 필수 엔터티만 포함하고 선택 포함/후속 확장 엔터티는 별도 단계에서 추가한다.

## 2. 공통 물리 설계 원칙

### 2.1 식별자와 키

- 기본키는 모든 엔터티에서 내부 식별자 단일 컬럼을 사용한다.
- 업무 표시용 키와 내부 기본키는 분리한다.
- 외부 시스템 식별자는 별도 매핑 또는 참조 엔터티에서 관리한다.

### 2.2 타입 표현

- 식별자: `uuid` 또는 이에 준하는 고유 문자열
- 코드값: 길이가 제한된 `varchar`
- 이름/제목: 일반 `varchar`
- 설명/사유: `text`
- 상태/유형/역할: 코드값 컬럼
- 일시: `timestamp`
- 여부: `boolean`

세부 타입과 길이는 구현 기술 스택 확정 후 조정한다.

### 2.3 감사 컬럼

- 주요 원천 엔터티에는 `created_at`, `updated_at` 를 기본 포함한다.
- 상태 이력, 감사 로그처럼 append-only 성격이 강한 엔터티는 `updated_at` 없이 생성 시각만 둘 수 있다.

### 2.4 다형 참조 처리

- `work_item_plan_link.plan_type + plan_id`
- `role_assignment.subject_type + subject_id`
- `audit_log.actor_type + actor_id`
- `audit_log.target_entity_type + target_entity_id`

위 구조는 정규 `FK`가 아니라 논리 다형 참조로 본다. 물리 모델에서는 다음을 기본으로 한다.

- `type` 컬럼 허용값을 코드표 수준으로 제한
- `type + id` 조합 인덱스 생성
- 무결성은 애플리케이션 검증 또는 후속 제약 규칙 테이블로 보완

## 3. 초기 릴리스 필수 테이블 초안

### 3.1 `project`

- 목적: 프로젝트 기본 컨테이너 저장
- 기본키: `project_id`
- 후보 유니크키:
  - `project_code`
- 주요 컬럼:
  - `project_id`: 내부 기본키
  - `project_code`: 업무 식별 코드
  - `project_name`: 프로젝트 이름
  - `project_type`: 프로젝트 분류
  - `project_status`: 현재 상태
  - `owning_organization_id`: 주관 조직 식별자
  - `project_owner_workforce_id`: 프로젝트 책임자 식별자
  - `start_date`: 계획 시작일
  - `target_end_date`: 목표 종료일
  - `actual_end_date`: 실제 종료일
  - `description`: 설명
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `owning_organization_id -> organization_master.organization_id`
  - `project_owner_workforce_id -> workforce_master.workforce_id`
- 초기 인덱스:
  - `ux_project_code`
  - `ix_project_status`
  - `ix_project_owning_organization_id`

### 3.2 `work_item`

- 목적: 공통 업무 항목 저장
- 기본키: `work_item_id`
- 후보 유니크키:
  - `(project_id, work_item_key)`
- 주요 컬럼:
  - `work_item_id`: 내부 기본키
  - `project_id`: 소속 프로젝트 식별자
  - `work_item_type_id`: 업무 유형 식별자
  - `work_item_key`: 프로젝트 내 업무 표시 키
  - `title`: 제목
  - `description`: 설명
  - `priority`: 우선순위 코드
  - `current_common_status`: 공통 상태 코드
  - `current_detailed_status_code`: 상세 상태 코드
  - `owning_organization_id`: 담당 조직 식별자
  - `assignee_workforce_id`: 담당자 식별자
  - `reporter_workforce_id`: 등록자 식별자
  - `planned_start_at`: 계획 시작 시각
  - `planned_end_at`: 계획 종료 시각
  - `actual_start_at`: 실제 시작 시각
  - `actual_end_at`: 실제 종료 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `project_id -> project.project_id`
  - `work_item_type_id -> work_item_type.work_item_type_id`
  - `owning_organization_id -> organization_master.organization_id`
  - `assignee_workforce_id -> workforce_master.workforce_id`
  - `reporter_workforce_id -> workforce_master.workforce_id`
- 초기 인덱스:
  - `ux_work_item_project_key`
  - `ix_work_item_project_id`
  - `ix_work_item_type_id`
  - `ix_work_item_common_status`
  - `ix_work_item_assignee_workforce_id`
  - `ix_work_item_project_status`

### 3.3 `work_item_type`

- 목적: 업무 유형 메타데이터 저장
- 기본키: `work_item_type_id`
- 후보 유니크키:
  - `type_code`
- 주요 컬럼:
  - `work_item_type_id`: 내부 기본키
  - `type_code`: 유형 코드
  - `type_name`: 유형 이름
  - `is_hierarchical`: 계층 참여 여부
  - `default_hierarchy_level`: 기본 계층 레벨
  - `is_plannable`: 계획 연결 가능 여부
  - `is_release_scoped`: 릴리스 범위 포함 여부
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각

### 3.4 `work_item_hierarchy`

- 목적: 업무 계층 관계 저장
- 기본키: `work_item_hierarchy_id`
- 후보 유니크키:
  - `(child_work_item_id)` 초기 단일 부모 원칙
- 주요 컬럼:
  - `work_item_hierarchy_id`: 내부 기본키
  - `parent_work_item_id`: 상위 업무 식별자
  - `child_work_item_id`: 하위 업무 식별자
  - `relationship_type`: 관계 유형
  - `sequence_no`: 같은 부모 내 정렬 순번
  - `created_at`: 생성 시각
- 직접 참조:
  - `parent_work_item_id -> work_item.work_item_id`
  - `child_work_item_id -> work_item.work_item_id`
- 초기 인덱스:
  - `ux_work_item_hierarchy_child`
  - `ix_work_item_hierarchy_parent`

### 3.5 `process_model_definition`

- 목적: 프로세스 모델 정의 저장
- 기본키: `process_model_definition_id`
- 후보 유니크키:
  - `model_code`
- 주요 컬럼:
  - `process_model_definition_id`: 내부 기본키
  - `model_code`: 모델 코드
  - `model_name`: 모델 이름
  - `model_category`: 모델 분류
  - `description`: 설명
  - `is_builtin`: 기본 제공 여부
  - `is_active`: 활성 여부
  - `version`: 모델 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각

### 3.6 `workflow_scheme`

- 목적: 상태 스킴 저장
- 기본키: `workflow_scheme_id`
- 후보 유니크키:
  - `(process_model_definition_id, scheme_code, version)`
- 주요 컬럼:
  - `workflow_scheme_id`: 내부 기본키
  - `process_model_definition_id`: 프로세스 모델 식별자
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `description`: 설명
  - `initial_status_code`: 초기 상태 코드
  - `is_default`: 기본 스킴 여부
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `process_model_definition_id -> process_model_definition.process_model_definition_id`

### 3.7 `workflow_status_definition`

- 목적: 상세 상태와 공통 상태 매핑 저장
- 기본키: `workflow_status_definition_id`
- 후보 유니크키:
  - `(workflow_scheme_id, status_code)`
- 주요 컬럼:
  - `workflow_status_definition_id`: 내부 기본키
  - `workflow_scheme_id`: 상태 스킴 식별자
  - `status_code`: 상세 상태 코드
  - `status_name`: 상세 상태 이름
  - `mapped_common_status`: 공통 상태 매핑값
  - `sequence_no`: 표시 순서
  - `is_terminal`: 종료 상태 여부
  - `created_at`: 생성 시각
- 직접 참조:
  - `workflow_scheme_id -> workflow_scheme.workflow_scheme_id`

### 3.8 `workflow_transition_definition`

- 목적: 상태 전이 규칙 저장
- 기본키: `workflow_transition_definition_id`
- 후보 유니크키:
  - `(workflow_scheme_id, from_status_code, to_status_code)`
- 주요 컬럼:
  - `workflow_transition_definition_id`: 내부 기본키
  - `workflow_scheme_id`: 상태 스킴 식별자
  - `from_status_code`: 시작 상세 상태 코드
  - `to_status_code`: 종료 상세 상태 코드
  - `transition_name`: 전이 이름
  - `requires_approval`: 승인 필요 여부
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
- 직접 참조:
  - `workflow_scheme_id -> workflow_scheme.workflow_scheme_id`

### 3.9 `planning_scheme`

- 목적: 계획 단위 사용 규칙 저장
- 기본키: `planning_scheme_id`
- 후보 유니크키:
  - `(process_model_definition_id, scheme_code, version)`
- 주요 컬럼:
  - `planning_scheme_id`: 내부 기본키
  - `process_model_definition_id`: 프로세스 모델 식별자
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `default_plan_unit_type`: 기본 계획 단위
  - `uses_iteration`: 반복 단위 사용 여부
  - `uses_release`: 릴리스 사용 여부
  - `uses_milestone`: 마일스톤 사용 여부
  - `uses_wbs`: `WBS` 사용 여부
  - `allows_parallel_tracks`: 병렬 계획 허용 여부

## 4. 연계 및 수집 저장 모델 초안

### 4.1 `integration_system`

- 목적: 연계 대상 외부 시스템 정의 저장
- 기본키: `integration_system_id`
- 후보 유니크키:
  - `system_code`
- 주요 컬럼:
  - `integration_system_id`: 내부 기본키
  - `system_code`: 시스템 식별 코드
  - `system_name`: 시스템 이름
  - `system_type`: 시스템 유형
  - `authentication_type`: 기본 인증 방식
  - `connection_status`: 현재 연결 상태
  - `owner_team`: 운영 책임 팀 또는 조직
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 초기 인덱스:
  - `ux_integration_system_code`
  - `ix_integration_system_type`
  - `ix_integration_system_status`

### 4.2 `integration_endpoint`

- 목적: 외부 시스템 접속 대상 URL 및 자원 경로 저장
- 기본키: `integration_endpoint_id`
- 후보 유니크키:
  - `(integration_system_id, endpoint_name)`
- 주요 컬럼:
  - `integration_endpoint_id`: 내부 기본키
  - `integration_system_id`: 대상 시스템 식별자
  - `endpoint_type`: `api`, `db`, `file`, `webhook` 등 엔드포인트 유형
  - `endpoint_name`: 엔드포인트 이름
  - `base_url`: 접속 기본 URL
  - `resource_path`: 자원 경로
  - `request_method`: 기본 호출 방식
  - `credential_binding_mode`: 자격증명 연결 방식
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `integration_system_id -> integration_system.integration_system_id`
- 초기 인덱스:
  - `ux_integration_endpoint_name`
  - `ix_integration_endpoint_system_id`
  - `ix_integration_endpoint_active`

### 4.3 `integration_credential`

- 목적: 외부 시스템 자격증명과 암호화 저장 메타데이터 관리
- 기본키: `integration_credential_id`
- 후보 유니크키:
  - `(integration_endpoint_id, credential_type, effective_from)`
- 주요 컬럼:
  - `integration_credential_id`: 내부 기본키
  - `integration_system_id`: 대상 시스템 식별자
  - `integration_endpoint_id`: 연결 대상 엔드포인트 식별자
  - `credential_type`: `basic`, `token`, `oauth_client`, `api_key` 등 자격증명 유형
  - `principal_id`: 계정 `id`, `client id` 등 비민감 식별자
  - `secret_ciphertext`: 암호화된 secret
  - `secret_key_version`: 암호화 키 버전
  - `secret_fingerprint`: 변경 감지용 요약 값
  - `rotation_status`: 키 또는 자격증명 교체 상태
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `last_validated_at`: 마지막 검증 시각
  - `last_updated_by`: 마지막 변경 주체
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `integration_system_id -> integration_system.integration_system_id`
  - `integration_endpoint_id -> integration_endpoint.integration_endpoint_id`
- 초기 인덱스:
  - `ix_integration_credential_endpoint_id`
  - `ix_integration_credential_rotation_status`
  - `ix_integration_credential_effective_period`
- 비고:
  - 민감정보는 평문 보관하지 않고 `secret_ciphertext` 만 저장한다.
  - `principal_id` 는 필요 시 마스킹 저장 또는 최소 노출 정책을 적용한다.

### 4.4 `integration_job`

- 목적: 동기화 작업 정의 저장
- 기본키: `integration_job_id`
- 후보 유니크키:
  - `(integration_system_id, job_code)`
- 주요 컬럼:
  - `integration_job_id`: 내부 기본키
  - `integration_system_id`: 대상 시스템 식별자
  - `job_code`: 작업 코드
  - `job_name`: 작업 이름
  - `trigger_type`: `schedule`, `manual`, `event`
  - `schedule_expression`: 스케줄 표현식
  - `job_status`: 작업 상태
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `integration_system_id -> integration_system.integration_system_id`

### 4.5 `integration_run`

- 목적: 작업 실행 이력 저장
- 기본키: `integration_run_id`
- 후보 유니크키:
  - `external_run_key` 선택 적용 가능
- 주요 컬럼:
  - `integration_run_id`: 내부 기본키
  - `integration_job_id`: 실행된 작업 식별자
  - `queued_at`: 실행 요청이 접수된 시각
  - `started_at`: 실제 실행 시작 시각, `queued` 상태에서는 비어 있을 수 있음
  - `ended_at`: 종료 시각
  - `run_status`: 실행 상태
  - `status_reason_code`: 현재 상태 전이의 주된 사유 코드
  - `status_reason_message`: 운영 확인용 상태 사유 메시지
  - `cancel_requested_at`: 취소 요청이 접수된 시각
  - `cancel_requested_by`: 취소 요청 주체
  - `cancel_reason_code`: 취소 요청 사유 코드
  - `processed_count`: 처리 건수
  - `success_count`: 성공 건수
  - `failure_count`: 실패 건수
  - `pending_count`: 보류 또는 재처리 대기 건수
  - `triggered_by`: 실행 요청 주체
  - `retry_of_run_id`: 재시도인 경우 원본 실행 식별자
  - `created_at`: 생성 시각
- 직접 참조:
  - `integration_job_id -> integration_job.integration_job_id`
  - `retry_of_run_id -> integration_run.integration_run_id`
- 초기 인덱스:
  - `ix_integration_run_job_id`
  - `ix_integration_run_status`
  - `ix_integration_run_started_at`
  - `ix_integration_run_cancel_requested_at`
  - `ix_integration_run_retry_of_run_id`

### 4.6 `raw_ingestion_event`

- 목적: 원시 수집 이벤트와 멱등 메타데이터 저장
- 기본키: `raw_ingestion_event_id`
- 후보 유니크키:
  - `(integration_run_id, source_record_key, payload_hash)` 선택 적용 가능
  - `(source_system, source_object_type, source_object_id, source_event_key)` 멱등 키
- 주요 컬럼:
  - `raw_ingestion_event_id`: 내부 기본키
  - `integration_run_id`: 수집 실행 식별자
  - `source_system`: 원천 시스템 코드
  - `source_object_type`: 원천 객체 유형
  - `source_object_id`: 원천 객체 식별자
  - `source_event_key`: 멱등 수집 키
  - `source_version`: 원천 버전
  - `source_sequence_no`: 원천 시퀀스 번호
  - `source_updated_at`: 원천 수정 시각
  - `source_record_key`: 원천 레코드 키
  - `payload_reference`: 원문 저장 위치
  - `payload_hash`: 원문 해시
  - `ingested_at`: 수집 시각
  - `normalization_status`: 표준화 상태
  - `created_at`: 생성 시각
- 직접 참조:
  - `integration_run_id -> integration_run.integration_run_id`
- 초기 인덱스:
  - `ux_raw_ingestion_event_idempotency`
  - `ix_raw_ingestion_event_run_id`
  - `ix_raw_ingestion_event_normalization_status`

### 4.7 `normalized_record_reference`

- 목적: 표준화 결과와 내부 대상 엔터티 연결 저장
- 기본키: `normalized_record_reference_id`
- 후보 유니크키:
  - `(raw_ingestion_event_id, target_entity_type, target_entity_id)`
- 주요 컬럼:
  - `normalized_record_reference_id`: 내부 기본키
  - `raw_ingestion_event_id`: 원시 이벤트 식별자
  - `target_entity_type`: 연결 대상 엔터티 유형
  - `target_entity_id`: 연결 대상 엔터티 식별자
  - `normalization_version`: 표준화 규칙 버전
  - `normalized_at`: 표준화 시각
  - `created_at`: 생성 시각
- 직접 참조:
  - `raw_ingestion_event_id -> raw_ingestion_event.raw_ingestion_event_id`
- 초기 인덱스:
  - `ix_normalized_record_reference_raw_id`
  - `ix_normalized_record_reference_target`

### 4.8 `sync_error`

- 목적: 수집/표준화/반영 오류 저장
- 기본키: `sync_error_id`
- 후보 유니크키:
  - 없음
- 주요 컬럼:
  - `sync_error_id`: 내부 기본키
  - `integration_run_id`: 실행 식별자
  - `error_code`: 오류 코드
  - `error_message`: 오류 메시지
  - `error_type`: 오류 유형
  - `retry_status`: 재시도 상태
  - `resolved_at`: 해결 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `integration_run_id -> integration_run.integration_run_id`
- 초기 인덱스:
  - `ix_sync_error_run_id`
  - `ix_sync_error_retry_status`
  - `is_default`: 기본 스킴 여부
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `process_model_definition_id -> process_model_definition.process_model_definition_id`

### 3.10 `view_scheme`

- 목적: 보드/간트/`WBS` 표현 규칙 저장
- 기본키: `view_scheme_id`
- 후보 유니크키:
  - `(process_model_definition_id, scheme_code, version)`
- 주요 컬럼:
  - `view_scheme_id`: 내부 기본키
  - `process_model_definition_id`: 프로세스 모델 식별자
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `default_board_type`: 기본 보드 유형
  - `supports_gantt`: 간트 지원 여부
  - `supports_wbs`: `WBS` 지원 여부
  - `supports_release_board`: 릴리스 보드 지원 여부
  - `default_grouping_rule`: 기본 그룹화 규칙
  - `default_sort_rule`: 기본 정렬 규칙
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `process_model_definition_id -> process_model_definition.process_model_definition_id`

### 3.11 `project_process_model`

- 목적: 프로젝트별 활성 프로세스 설정 저장
- 기본키: `project_process_model_id`
- 후보 유니크키:
  - `(project_id, process_model_definition_id, effective_from)`
- 주요 제약:
  - 프로젝트별 동일 시점 `is_primary=true` 는 1건만 허용
- 주요 컬럼:
  - `project_process_model_id`: 내부 기본키
  - `project_id`: 프로젝트 식별자
  - `process_model_definition_id`: 프로세스 모델 식별자
  - `workflow_scheme_id`: 상태 스킴 식별자
  - `planning_scheme_id`: 계획 스킴 식별자
  - `view_scheme_id`: 보기 스킴 식별자
  - `assignment_scope`: 기본/예외 범위 구분
  - `is_primary`: 기본 활성 여부
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `project_id -> project.project_id`
  - `process_model_definition_id -> process_model_definition.process_model_definition_id`
  - `workflow_scheme_id -> workflow_scheme.workflow_scheme_id`
  - `planning_scheme_id -> planning_scheme.planning_scheme_id`
  - `view_scheme_id -> view_scheme.view_scheme_id`
- 초기 인덱스:
  - `ix_project_process_model_project_id`
  - `ix_project_process_model_primary`
  - `ix_project_process_model_effective_range`

### 3.12 `iteration`

- 목적: 반복 계획 단위 저장
- 기본키: `iteration_id`
- 후보 유니크키:
  - `(project_id, name)`
- 주요 컬럼:
  - `iteration_id`: 내부 기본키
  - `project_id`: 프로젝트 식별자
  - `name`: 반복 이름
  - `goal`: 반복 목표
  - `status`: 현재 상태
  - `start_date`: 시작일
  - `end_date`: 종료일
  - `capacity`: 계획 수용량
  - `sequence_no`: 순번
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `project_id -> project.project_id`

### 3.13 `release`

- 목적: 릴리스 단위 저장
- 기본키: `release_id`
- 후보 유니크키:
  - `(project_id, release_code)`
- 주요 컬럼:
  - `release_id`: 내부 기본키
  - `project_id`: 프로젝트 식별자
  - `release_code`: 릴리스 코드
  - `release_name`: 릴리스 이름
  - `status`: 현재 상태
  - `planned_release_at`: 계획 배포 시각
  - `actual_release_at`: 실제 배포 시각
  - `description`: 설명
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `project_id -> project.project_id`

### 3.14 `milestone`

- 목적: 마일스톤 저장
- 기본키: `milestone_id`
- 후보 유니크키:
  - `(project_id, milestone_code)`
- 주요 컬럼:
  - `milestone_id`: 내부 기본키
  - `project_id`: 프로젝트 식별자
  - `milestone_code`: 마일스톤 코드
  - `milestone_name`: 마일스톤 이름
  - `milestone_type`: 마일스톤 유형
  - `target_at`: 목표 시각
  - `actual_at`: 실제 달성 시각
  - `status`: 현재 상태
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `project_id -> project.project_id`

### 3.15 `work_item_plan_link`

- 목적: 업무와 계획 단위 연결 저장
- 기본키: `work_item_plan_link_id`
- 후보 유니크키:
  - `(work_item_id, plan_type, plan_id, link_role)`
- 주요 컬럼:
  - `work_item_plan_link_id`: 내부 기본키
  - `work_item_id`: 업무 식별자
  - `plan_type`: 계획 단위 유형 코드
  - `plan_id`: 계획 단위 식별자
  - `link_role`: 연결 역할 코드
  - `sequence_no`: 표시 순서
  - `is_primary`: 대표 연결 여부
  - `linked_by_rule_ref`: 적용 규칙 참조값
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `work_item_id -> work_item.work_item_id`
- 다형 참조:
  - `plan_type + plan_id -> iteration | release | milestone | wbs_node`
- 초기 인덱스:
  - `ix_work_item_plan_link_work_item_id`
  - `ix_work_item_plan_link_plan_ref`
  - `ix_work_item_plan_link_primary`

### 3.16 `work_item_status_history`

- 목적: 업무 상태 이력 저장
- 기본키: `work_item_status_history_id`
- 주요 컬럼:
  - `work_item_status_history_id`: 내부 기본키
  - `work_item_id`: 업무 식별자
  - `from_common_status`: 변경 전 공통 상태
  - `from_detailed_status_code`: 변경 전 상세 상태
  - `to_common_status`: 변경 후 공통 상태
  - `to_detailed_status_code`: 변경 후 상세 상태
  - `workflow_transition_definition_id`: 적용 전이 규칙 식별자
  - `changed_by`: 변경 주체 식별자
  - `changed_at`: 변경 시각
  - `change_reason`: 변경 사유
  - `source_type`: 사용자/시스템/연계 구분
- 직접 참조:
  - `work_item_id -> work_item.work_item_id`
  - `workflow_transition_definition_id -> workflow_transition_definition.workflow_transition_definition_id`
- 초기 인덱스:
  - `ix_work_item_status_history_work_item_id_changed_at`

### 3.17 `organization_master`

- 목적: 조직 기준정보 저장
- 기본키: `organization_id`
- 후보 유니크키:
  - `organization_code`
- 주요 컬럼:
  - `organization_id`: 내부 기본키
  - `organization_code`: 조직 코드
  - `organization_name`: 조직 이름
  - `parent_organization_id`: 상위 조직 식별자
  - `organization_status`: 상태
  - `effective_from`: 유효 시작 시점
  - `effective_to`: 유효 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `parent_organization_id -> organization_master.organization_id`

### 3.18 `workforce_master`

- 목적: 인력 기준정보 저장
- 기본키: `workforce_id`
- 후보 유니크키:
  - `employee_number`
  - `email`
- 주요 컬럼:
  - `workforce_id`: 내부 기본키
  - `employee_number`: 사번
  - `display_name`: 표시 이름
  - `employment_status`: 재직 상태
  - `primary_organization_id`: 주 조직 식별자
  - `job_family`: 직무 분류
  - `email`: 이메일
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `primary_organization_id -> organization_master.organization_id`

### 3.19 `role_policy`

- 목적: 역할 정책 저장
- 기본키: `role_policy_id`
- 후보 유니크키:
  - `role_code`
- 주요 컬럼:
  - `role_policy_id`: 내부 기본키
  - `role_code`: 역할 코드
  - `role_name`: 역할 이름
  - `description`: 설명
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각

### 3.20 `permission_scope`

- 목적: 역할별 권한 범위 저장
- 기본키: `permission_scope_id`
- 후보 유니크키:
  - `(role_policy_id, resource_type, action_code, scope_code)`
- 주요 컬럼:
  - `permission_scope_id`: 내부 기본키
  - `role_policy_id`: 역할 정책 식별자
  - `resource_type`: 자원 유형
  - `action_code`: 행위 코드
  - `scope_code`: 범위 코드
  - `constraint_expression`: 추가 제약 표현
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `role_policy_id -> role_policy.role_policy_id`

### 3.21 `role_assignment`

- 목적: 역할 배정 저장
- 기본키: `role_assignment_id`
- 후보 유니크키:
  - `(subject_type, subject_id, role_type, assignee_workforce_id, effective_from)`
- 주요 컬럼:
  - `role_assignment_id`: 내부 기본키
  - `subject_type`: 배정 대상 유형
  - `subject_id`: 배정 대상 식별자
  - `role_type`: 역할 유형 코드
  - `assignee_workforce_id`: 담당자 식별자
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 직접 참조:
  - `assignee_workforce_id -> workforce_master.workforce_id`
- 논리 정렬:
  - `role_type` 은 `role_policy.role_code` 체계와 정렬
- 다형 참조:
  - `subject_type + subject_id -> organization_master | project | work_item`
- 초기 인덱스:
  - `ix_role_assignment_assignee_workforce_id`
  - `ix_role_assignment_subject_ref`

### 3.22 `audit_log`

- 목적: 운영 및 업무 감사 이벤트 저장
- 기본키: `audit_log_id`
- 주요 컬럼:
  - `audit_log_id`: 내부 기본키
  - `actor_type`: 행위자 유형
  - `actor_id`: 행위자 식별자
  - `target_entity_type`: 대상 엔터티 유형
  - `target_entity_id`: 대상 엔터티 식별자
  - `event_type`: 이벤트 유형
  - `event_result`: 처리 결과
  - `event_payload_ref`: 상세 payload 참조
  - `occurred_at`: 발생 시각
  - `created_at`: 적재 시각
- 다형 참조:
  - `actor_type + actor_id -> workforce_master | system_account | integration_process`
  - `target_entity_type + target_entity_id -> project | work_item | governance_entity`
- 초기 인덱스:
  - `ix_audit_log_actor_ref`
  - `ix_audit_log_target_ref`
  - `ix_audit_log_occurred_at`

## 4. 초기 제약 및 검증 규칙

- `project_process_model` 에서는 동일 시점 프로젝트당 `is_primary=true` 레코드가 하나만 존재해야 한다.
- `work_item_hierarchy` 는 초기 릴리스에서 단일 부모와 순환 금지 규칙을 적용한다.
- `work_item_status_history` 최신 레코드와 `work_item.current_*` 상태는 일치해야 한다.
- `work_item_plan_link.plan_type` 은 허용 코드값 집합과 일치해야 한다.
- `role_assignment.role_type` 은 허용 코드값 집합 및 `role_policy.role_code` 체계와 정렬되어야 한다.
- 다형 참조 무결성은 애플리케이션 서비스 또는 후속 검증 배치에서 보완한다.

## 5. 구현 우선순위 제안

### 5.1 1차 생성 대상

- `organization_master`
- `workforce_master`
- `project`
- `work_item_type`
- `work_item`
- `process_model_definition`
- `workflow_scheme`
- `workflow_status_definition`
- `workflow_transition_definition`
- `planning_scheme`
- `view_scheme`
- `project_process_model`

### 5.2 2차 생성 대상

- `iteration`
- `release`
- `milestone`
- `work_item_hierarchy`
- `work_item_plan_link`
- `work_item_status_history`
- `role_policy`
- `permission_scope`
- `role_assignment`
- `audit_log`

## 6. 다음 작업 후보

- 초기 릴리스 필수 테이블 기준 `DDL` 스타일 초안 작성
- 제약/인덱스 규칙을 `RDBMS` 후보 기준으로 구체화
- 다형 참조 무결성 검증 방식을 서비스 레벨 규칙으로 분리
