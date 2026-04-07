# 핵심 엔터티 정의 초안

- 문서 목적: 핵심 도메인 모델에서 도출한 엔터티를 별도 문서로 분리해 검토와 항목 조정을 쉽게 한다.
- 범위: 핵심 엔터티의 목적, 핵심 식별자, 주요 속성 초안
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, 기획자
- 상태: draft
- 최종 수정일: 2026-04-06
- 관련 문서: `docs/architecture/domain_model_draft.md`, `docs/architecture/system_architecture_draft.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 도메인 문서: [./domain_model_draft.md](./domain_model_draft.md)

## 1. 문서 사용 원칙

- 본 문서는 엔터티 단위 검토와 항목 수정에 집중할 수 있도록 개별 엔터티를 분리해 관리한다.
- 현재 단계에서는 물리 테이블 확정이 아니라 논리 엔터티와 주요 속성의 방향을 정리한다.
- 속성은 우선 식별자, 운영 핵심 필드, 상태/관계 연결, 감사 최소 필드 중심으로 제한한다.
- 프로세스 모델은 `Agile`, `V-Model` 을 포함하되 고정 목록으로 닫지 않고 확장 가능한 메타모델로 유지한다.
- 주요 속성은 `속성명: 설명` 형식으로 기록해 원문 편집 시 의미를 바로 확인할 수 있게 한다.

## 2. 빠른 탐색

### 2.1 업무 항목 및 프로젝트

- [project](#31-project)
- [project_scope](#32-project_scope)
- [work_item](#33-work_item)
- [work_item_type](#34-work_item_type)
- [work_item_hierarchy](#35-work_item_hierarchy)
- [work_item_status_history](#36-work_item_status_history)
- [work_item_external_reference](#37-work_item_external_reference)
- [project_work_item_policy](#38-project_work_item_policy)
- [project_process_model](#39-project_process_model)
- [iteration](#310-iteration)
- [release](#311-release)
- [milestone](#312-milestone)
- [wbs_node](#313-wbs_node)
- [work_item_plan_link](#314-work_item_plan_link)

### 2.2 조직 및 인력

- [organization_master](#41-organization_master)
- [organization_history](#42-organization_history)
- [workforce_master](#43-workforce_master)
- [workforce_assignment_history](#44-workforce_assignment_history)
- [role_assignment](#45-role_assignment)
- [absence_calendar_entry](#46-absence_calendar_entry)
- [identity_mapping](#47-identity_mapping)
- [organization_change_case](#48-organization_change_case)

### 2.3 연계 및 수집

- [integration_system](#51-integration_system)
- [integration_endpoint](#52-integration_endpoint)
- [integration_job](#53-integration_job)
- [integration_run](#54-integration_run)
- [raw_ingestion_event](#55-raw_ingestion_event)
- [normalized_record_reference](#56-normalized_record_reference)
- [sync_error](#57-sync_error)

### 2.4 품질 및 개발 문맥

- [review_summary](#61-review_summary)
- [build_summary](#62-build_summary)
- [artifact_metadata](#63-artifact_metadata)
- [quality_signal](#64-quality_signal)
- [test_execution_summary](#65-test_execution_summary)
- [defect_link](#66-defect_link)
- [release_readiness_snapshot](#67-release_readiness_snapshot)
- [quality_gate_decision](#68-quality_gate_decision)

### 2.5 운영 통제 및 감사

- [role_policy](#71-role_policy)
- [permission_scope](#72-permission_scope)
- [migration_request](#73-migration_request)
- [exception_case](#74-exception_case)
- [audit_log](#75-audit_log)
- [governance_decision](#76-governance_decision)

### 2.6 `AI` 보조

- [ai_review_draft](#81-ai_review_draft)
- [ai_test_draft](#82-ai_test_draft)
- [ai_ci_plan_draft](#83-ai_ci_plan_draft)
- [ai_input_context](#84-ai_input_context)
- [rule_set_reference](#85-rule_set_reference)
- [ai_execution_profile](#86-ai_execution_profile)

### 2.7 프로세스 모델 메타모델

- [process_model_definition](#91-process_model_definition)
- [workflow_scheme](#92-workflow_scheme)
- [workflow_status_definition](#93-workflow_status_definition)
- [workflow_transition_definition](#94-workflow_transition_definition)
- [planning_scheme](#95-planning_scheme)
- [view_scheme](#96-view_scheme)
- [process_model_component_link](#97-process_model_component_link)

## 3. 업무 항목 및 프로젝트 도메인 엔터티

### 3.1 `project`

- 목적: 운영 범위와 관리 대상을 정의하는 최상위 컨테이너
- 핵심 식별자: `project_id`
- 주요 속성:
  - `project_code`: 프로젝트를 사람이 식별하는 업무 코드
  - `project_name`: 프로젝트 표시 이름
  - `project_type`: 관리 대상 여부 등 프로젝트 분류 값
  - `project_status`: 현재 운영 상태
  - `owning_organization_id`: 주관 조직 참조
  - `project_owner_workforce_id`: 프로젝트 책임자 참조
  - `start_date`: 계획 시작일
  - `target_end_date`: 목표 종료일
  - `actual_end_date`: 실제 종료일
  - `description`: 프로젝트 설명
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 관리 대상/관리 외 프로젝트 공통 엔터티. 기본 프로세스 모델은 `project_process_model.is_primary=true` 레코드로 해석하고, `project` 본체에는 중복 보관하지 않는다.

### 3.2 `project_scope`

- 목적: 프로젝트의 관리 범위, 대상 시스템, 관리 수준 정의
- 핵심 식별자: `project_scope_id`
- 주요 속성:
  - `project_id`: 소속 프로젝트 참조
  - `scope_type`: 범위 유형
  - `management_level`: 관리 수준 정의
  - `target_system_code`: 대상 시스템 식별 코드
  - `effective_from`: 범위 적용 시작 시점
  - `effective_to`: 범위 적용 종료 시점
  - `scope_note`: 범위 관련 메모
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 전환 전/후 범위 차이 반영

### 3.3 `work_item`

- 목적: 공통 업무 항목의 기준 엔터티
- 핵심 식별자: `work_item_id`
- 주요 속성:
  - `project_id`: 소속 프로젝트 참조
  - `work_item_type_id`: 업무 항목 유형 참조
  - `work_item_key`: 화면과 연계에서 사용하는 업무 키
  - `title`: 업무 항목 제목
  - `description`: 업무 항목 설명
  - `priority`: 우선순위
  - `current_common_status`: 공통 라이프사이클 상태
  - `current_detailed_status_code`: 프로세스별 상세 상태 코드
  - `owning_organization_id`: 담당 조직 참조
  - `assignee_workforce_id`: 현재 담당자 참조
  - `reporter_workforce_id`: 등록자 또는 요청자 참조
  - `planned_start_at`: 계획 시작 시각
  - `planned_end_at`: 계획 종료 시각
  - `actual_start_at`: 실제 시작 시각
  - `actual_end_at`: 실제 종료 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `epic`, `story`, `task`, `sub-task`, `defect` 등을 포괄한다. `process_model_definition_id` 는 프로젝트 기본 설정과 중복될 수 있어 `project_process_model` 에서 관리하고, `progress_percent`, `board_rank` 는 읽기 모델 또는 별도 표현 모델에서 관리하는 방향을 우선 적용한다.

### 3.4 `work_item_type`

- 목적: 업무 항목 유형 정의
- 핵심 식별자: `work_item_type_id`
- 주요 속성:
  - `type_code`: 유형 코드
  - `type_name`: 유형 이름
  - `is_hierarchical`: 계층 참여 가능 여부
  - `default_hierarchy_level`: 기본 계층 레벨
  - `is_plannable`: 일정 계획 연결 가능 여부
  - `is_release_scoped`: 릴리스 범위 포함 여부
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 고정 enum 대신 메타데이터화 가능

### 3.5 `work_item_hierarchy`

- 목적: 업무 항목 간 상하위 관계
- 핵심 식별자: `work_item_hierarchy_id`
- 주요 속성:
  - `parent_work_item_id`: 상위 업무 항목 참조
  - `child_work_item_id`: 하위 업무 항목 참조
  - `relationship_type`: 관계 유형
  - `sequence_no`: 같은 부모 내 표시 순서
  - `created_at`: 생성 시각
- 비고: `parent_child`, `related` 등 관계 확장 고려

### 3.6 `work_item_status_history`

- 목적: 업무 항목 상태 이력
- 핵심 식별자: `work_item_status_history_id`
- 주요 속성:
  - `work_item_id`: 대상 업무 항목 참조
  - `from_common_status`: 변경 전 공통 상태
  - `from_detailed_status_code`: 변경 전 상세 상태 코드
  - `to_common_status`: 변경 후 공통 상태
  - `to_detailed_status_code`: 변경 후 상세 상태 코드
  - `workflow_transition_definition_id`: 적용된 상태 전이 규칙 참조
  - `changed_by`: 변경 주체
  - `changed_at`: 변경 시각
  - `change_reason`: 변경 사유
  - `source_type`: 사용자/시스템/연계 등 변경 출처
- 비고: 공통 상태와 상세 상태 모두 추적하며, 가능하면 전이 규칙 기준으로 이력을 남긴다.

### 3.7 `work_item_external_reference`

- 목적: 외부 시스템 업무 항목과의 매핑
- 핵심 식별자: `work_item_external_reference_id`
- 주요 속성:
  - `work_item_id`: 내부 업무 항목 참조
  - `integration_system_id`: 외부 시스템 참조
  - `external_record_type`: 외부 레코드 유형
  - `external_record_key`: 외부 시스템 키
  - `external_url`: 외부 화면 링크
  - `sync_status`: 연계 동기화 상태
  - `last_synced_at`: 마지막 동기화 시각
- 비고: `Jira issue key` 등 연결

### 3.8 `project_work_item_policy`

- 목적: 프로젝트별 업무 항목 운영 규칙
- 핵심 식별자: `project_work_item_policy_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `allowed_hierarchy_rule`: 허용 계층 구조 규칙
  - `required_type_rule`: 필수 유형 규칙
  - `required_field_rule`: 필수 필드 규칙
  - `status_policy_ref`: 상태 규칙 참조
  - `effective_from`: 규칙 적용 시작 시점
  - `effective_to`: 규칙 적용 종료 시점
- 비고: 계층 허용 범위, 필수 속성 등

### 3.9 `project_process_model`

- 목적: 프로젝트와 프로세스 모델의 연결
- 핵심 식별자: `project_process_model_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `process_model_definition_id`: 적용 프로세스 모델 참조
  - `workflow_scheme_id`: 상태 스킴 참조
  - `planning_scheme_id`: 계획 스킴 참조
  - `view_scheme_id`: 보기 스킴 참조
  - `is_primary`: 기본 활성 모델 여부
  - `assignment_scope`: 프로젝트 기본값인지 예외 설정인지 구분하는 범위 값
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 프로젝트별 활성 프로세스 모델 지정. 동일 시점에는 기본 활성 모델이 1개만 존재하도록 관리하고, `work_item` 은 기본적으로 이 설정을 상속한다.

### 3.10 `iteration`

- 목적: `sprint` 등 반복 계획 단위
- 핵심 식별자: `iteration_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `name`: 반복 단위 이름
  - `goal`: 반복 목표
  - `status`: 반복 진행 상태
  - `start_date`: 시작일
  - `end_date`: 종료일
  - `capacity`: 계획 수용량
  - `sequence_no`: 반복 순번
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 프로세스 모델에 따라 선택적 사용

### 3.11 `release`

- 목적: 배포 또는 제공 범위 단위
- 핵심 식별자: `release_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `release_code`: 릴리스 코드
  - `release_name`: 릴리스 이름
  - `status`: 릴리스 상태
  - `planned_release_at`: 계획 배포 시각
  - `actual_release_at`: 실제 배포 시각
  - `description`: 릴리스 설명
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 여러 `work_item` 묶음 허용

### 3.12 `milestone`

- 목적: 주요 일정 기준점
- 핵심 식별자: `milestone_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `milestone_code`: 마일스톤 코드
  - `milestone_name`: 마일스톤 이름
  - `milestone_type`: 마일스톤 유형
  - `target_at`: 목표 시각
  - `actual_at`: 실제 달성 시각
  - `status`: 마일스톤 상태
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 간트/WBS 집계에 활용

### 3.13 `wbs_node`

- 목적: 단계형 계획/WBS 구조 노드
- 핵심 식별자: `wbs_node_id`
- 주요 속성:
  - `project_id`: 대상 프로젝트 참조
  - `parent_wbs_node_id`: 상위 `WBS` 노드 참조
  - `wbs_code`: `WBS` 코드
  - `wbs_name`: `WBS` 이름
  - `level_no`: 계층 레벨
  - `sequence_no`: 같은 레벨 내 순서
  - `planned_start_at`: 계획 시작 시각
  - `planned_end_at`: 계획 종료 시각
  - `progress_percent`: 집계 진행률
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `V-Model` 계열 구조 대응

### 3.14 `work_item_plan_link`

- 목적: 업무 항목과 계획 단위 간 연결
- 핵심 식별자: `work_item_plan_link_id`
- 주요 속성:
  - `work_item_id`: 대상 업무 항목 참조
  - `plan_type`: 연결 대상 계획 단위 유형
  - `plan_id`: 연결 대상 계획 단위 식별자
  - `link_role`: 소속/대상/게이트 등 연결 역할
  - `sequence_no`: 동일 계획 단위 내 표시 순서
  - `is_primary`: 동일 유형 다중 연결 시 대표 연결 여부
  - `linked_by_rule_ref`: 어떤 계획 규칙 또는 정책으로 연결됐는지에 대한 참조
  - `effective_from`: 연결 적용 시작 시점
  - `effective_to`: 연결 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `iteration`, `release`, `milestone`, `wbs_node` 연결 공통화. 저장 모델은 연결 사실과 최소 제약만 보관하고, 보드/간트/릴리스 뷰는 읽기 모델로 재구성한다.

## 4. 조직 및 인력 도메인 엔터티

### 4.1 `organization_master`

- 목적: 현재 시점 조직 기준정보
- 핵심 식별자: `organization_id`
- 주요 속성:
  - `organization_code`: 조직 코드
  - `organization_name`: 조직 이름
  - `parent_organization_id`: 상위 조직 참조
  - `organization_status`: 조직 상태
  - `effective_from`: 유효 시작 시점
  - `effective_to`: 유효 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 활성 조직 기준

### 4.2 `organization_history`

- 목적: 조직 개편 이력 관리
- 핵심 식별자: `organization_history_id`
- 주요 속성:
  - `organization_id`: 대상 조직 참조
  - `change_type`: 변경 유형
  - `previous_parent_organization_id`: 이전 상위 조직 참조
  - `current_parent_organization_id`: 현재 상위 조직 참조
  - `effective_from`: 변경 적용 시작 시점
  - `effective_to`: 변경 적용 종료 시점
  - `change_reason`: 변경 사유
  - `created_at`: 생성 시각
- 비고: 명칭/구조/유효기간 보관

### 4.3 `workforce_master`

- 목적: 인력 기준정보
- 핵심 식별자: `workforce_id`
- 주요 속성:
  - `employee_number`: 사번 등 원천 인력 번호
  - `display_name`: 표시 이름
  - `employment_status`: 재직 상태
  - `primary_organization_id`: 주 조직 참조
  - `job_family`: 직군 또는 직무 분류
  - `email`: 대표 이메일
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 내부 표준 인력 식별자

### 4.4 `workforce_assignment_history`

- 목적: 인력의 조직 소속 이력
- 핵심 식별자: `assignment_history_id`
- 주요 속성:
  - `workforce_id`: 대상 인력 참조
  - `organization_id`: 소속 조직 참조
  - `assignment_type`: 배치 유형
  - `effective_from`: 배치 시작 시점
  - `effective_to`: 배치 종료 시점
  - `is_primary`: 주 소속 여부
  - `created_at`: 생성 시각
- 비고: 과거/현재 소속 추적

### 4.5 `role_assignment`

- 목적: 조직장, 승인권자, 대행 승인자 등 역할 관계
- 핵심 식별자: `role_assignment_id`
- 주요 속성:
  - `subject_type`: 역할이 연결되는 대상 유형
  - `subject_id`: 역할이 연결되는 대상 식별자
  - `role_type`: 역할 유형
  - `assignee_workforce_id`: 역할 담당자 참조
  - `effective_from`: 역할 시작 시점
  - `effective_to`: 역할 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 별도 엔터티보다 관계 모델 권장

### 4.6 `absence_calendar_entry`

- 목적: 개인 부재 일정
- 핵심 식별자: `absence_entry_id`
- 주요 속성:
  - `workforce_id`: 대상 인력 참조
  - `absence_type`: 부재 유형
  - `start_at`: 부재 시작 시각
  - `end_at`: 부재 종료 시각
  - `approval_status`: 승인 상태
  - `source_type`: 입력 출처
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 조직/프로젝트 캘린더는 읽기 모델로 재구성

### 4.7 `identity_mapping`

- 목적: 외부 원천과 내부 식별자 매핑
- 핵심 식별자: `identity_mapping_id`
- 주요 속성:
  - `source_system_code`: 원천 시스템 코드
  - `source_identity_key`: 원천 식별자 값
  - `internal_entity_type`: 내부 엔터티 유형
  - `internal_entity_id`: 내부 엔터티 식별자
  - `mapping_status`: 매핑 상태
  - `verified_at`: 검증 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 인사 `DB`, `Crowd`, 내부 마스터 연결

### 4.8 `organization_change_case`

- 목적: 조직 변경 감지 및 검토 케이스
- 핵심 식별자: `organization_change_case_id`
- 주요 속성:
  - `change_source`: 변경 감지 출처
  - `detected_at`: 감지 시각
  - `change_scope_summary`: 영향 범위 요약
  - `review_status`: 검토 상태
  - `requested_by`: 검토 요청자
  - `reviewed_by`: 검토 담당자
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 영향 분석과 마이그레이션 준비 연결

## 5. 연계 및 수집 도메인 엔터티

### 5.1 `integration_system`

- 목적: 연계 대상 시스템 정의
- 핵심 식별자: `integration_system_id`
- 주요 속성:
  - `system_code`: 시스템 코드
  - `system_name`: 시스템 이름
  - `system_type`: 시스템 유형
  - `authentication_type`: 인증 방식
  - `connection_status`: 연결 상태
  - `owner_team`: 운영 책임 조직 또는 팀
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 유형, 인증, 상태 관리

### 5.2 `integration_endpoint`

- 목적: 시스템별 접속 대상/자원 정의
- 핵심 식별자: `integration_endpoint_id`
- 주요 속성:
  - `integration_system_id`: 대상 시스템 참조
  - `endpoint_type`: 엔드포인트 유형
  - `endpoint_name`: 엔드포인트 이름
  - `resource_path`: 접근 자원 경로
  - `request_method`: 호출 방식
  - `is_active`: 활성 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: API, DB, 파일 등

### 5.3 `integration_job`

- 목적: 동기화 작업 정의
- 핵심 식별자: `integration_job_id`
- 주요 속성:
  - `integration_system_id`: 대상 시스템 참조
  - `job_code`: 작업 코드
  - `job_name`: 작업 이름
  - `trigger_type`: 실행 트리거 유형
  - `schedule_expression`: 스케줄 표현식
  - `job_status`: 작업 상태
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 배치/수동/이벤트형

### 5.4 `integration_run`

- 목적: 작업 실행 이력
- 핵심 식별자: `integration_run_id`
- 주요 속성:
  - `integration_job_id`: 실행된 작업 참조
  - `started_at`: 시작 시각
  - `ended_at`: 종료 시각
  - `run_status`: 실행 상태
  - `processed_count`: 처리 건수
  - `success_count`: 성공 건수
  - `failure_count`: 실패 건수
  - `triggered_by`: 실행 요청 주체
  - `created_at`: 생성 시각
- 비고: 시작/종료/결과/건수

### 5.5 `raw_ingestion_event`

- 목적: 원시 수집 결과 보관
- 핵심 식별자: `raw_ingestion_event_id`
- 주요 속성:
  - `integration_run_id`: 수집 실행 참조
  - `source_record_key`: 원천 레코드 키
  - `payload_reference`: 원문 저장 위치 참조
  - `payload_hash`: 원문 무결성 확인 값
  - `ingested_at`: 수집 시각
  - `normalization_status`: 표준화 상태
  - `created_at`: 생성 시각
- 비고: 원천 참조와 payload reference 보관

### 5.6 `normalized_record_reference`

- 목적: 표준화 결과와 원천 연결
- 핵심 식별자: `normalized_record_reference_id`
- 주요 속성:
  - `raw_ingestion_event_id`: 원시 수집 이벤트 참조
  - `target_entity_type`: 연결 대상 엔터티 유형
  - `target_entity_id`: 연결 대상 엔터티 식별자
  - `normalization_version`: 표준화 규칙 버전
  - `normalized_at`: 표준화 시각
  - `created_at`: 생성 시각
- 비고: 스냅샷 추적용

### 5.7 `sync_error`

- 목적: 실패/예외 이력
- 핵심 식별자: `sync_error_id`
- 주요 속성:
  - `integration_run_id`: 실패가 발생한 실행 참조
  - `error_code`: 오류 코드
  - `error_message`: 오류 메시지
  - `error_type`: 오류 유형
  - `retry_status`: 재시도 상태
  - `resolved_at`: 해결 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 재시도 정책과 연결

## 6. 품질 및 개발 문맥 도메인 엔터티

### 6.1 `review_summary`

- 목적: 코드리뷰 상태 및 요약
- 핵심 식별자: `review_summary_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `review_source_type`: 리뷰 출처 유형
  - `review_key`: 외부 리뷰 식별 키
  - `review_status`: 리뷰 상태
  - `reviewer_count`: 참여 리뷰어 수
  - `approval_count`: 승인 수
  - `last_reviewed_at`: 마지막 리뷰 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `work_item` 또는 변경 집합 연결

### 6.2 `build_summary`

- 목적: 빌드 실행과 결과 요약
- 핵심 식별자: `build_summary_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `pipeline_key`: 외부 파이프라인 식별 키
  - `build_number`: 빌드 번호
  - `build_status`: 빌드 상태
  - `started_at`: 시작 시각
  - `ended_at`: 종료 시각
  - `triggered_by`: 실행 주체
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 외부 `CI` 결과 연결

### 6.3 `artifact_metadata`

- 목적: 산출물 메타데이터
- 핵심 식별자: `artifact_id`
- 주요 속성:
  - `build_summary_id`: 원본 빌드 참조
  - `artifact_name`: 산출물 이름
  - `artifact_version`: 산출물 버전
  - `target_environment`: 배포 대상 환경
  - `signature_status`: 서명 또는 검증 상태
  - `published_at`: 게시 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 버전, 대상 환경, 서명 등

### 6.4 `quality_signal`

- 목적: 정적분석/품질 신호 공통 모델
- 핵심 식별자: `quality_signal_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `signal_source`: 신호 출처
  - `signal_type`: 신호 유형
  - `severity`: 심각도
  - `signal_status`: 처리 상태
  - `detected_at`: 감지 시각
  - `resolved_at`: 해소 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 심각도, 상태, 출처 포함

### 6.5 `test_execution_summary`

- 목적: 테스트 실행 결과 요약
- 핵심 식별자: `test_execution_summary_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `test_scope`: 테스트 범위
  - `execution_source`: 실행 출처
  - `total_count`: 전체 실행 수
  - `passed_count`: 성공 수
  - `failed_count`: 실패 수
  - `executed_at`: 실행 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 단위/통합/수동 테스트 포괄

### 6.6 `defect_link`

- 목적: 결함과 업무 항목/빌드/테스트 간 연결
- 핵심 식별자: `defect_link_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `external_defect_key`: 외부 결함 식별 키
  - `build_summary_id`: 관련 빌드 참조
  - `test_execution_summary_id`: 관련 테스트 실행 참조
  - `defect_status`: 결함 상태
  - `severity`: 결함 심각도
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 결함 자체는 외부 참조 또는 내부 정의 가능

### 6.7 `release_readiness_snapshot`

- 목적: 릴리스 준비도 계산 결과 스냅샷
- 핵심 식별자: `release_readiness_snapshot_id`
- 주요 속성:
  - `release_id`: 대상 릴리스 참조
  - `readiness_score`: 계산된 준비도 점수
  - `decision_hint`: 판단 보조 결과
  - `blocking_issue_count`: 차단 이슈 수
  - `snapshot_at`: 스냅샷 생성 시각
  - `created_at`: 생성 시각
- 비고: 계산 뷰 또는 스냅샷 성격 권장

### 6.8 `quality_gate_decision`

- 목적: 품질 게이트 판정 이력
- 핵심 식별자: `quality_gate_decision_id`
- 주요 속성:
  - `release_id`: 대상 릴리스 참조
  - `decision_status`: 판정 결과
  - `decision_reason`: 판정 사유
  - `decided_by`: 판정 주체
  - `decided_at`: 판정 시각
  - `created_at`: 생성 시각
- 비고: 계산 결과만 두지 않고 이력 보관 권장

## 7. 운영 통제 및 감사 도메인 엔터티

### 7.1 `role_policy`

- 목적: 역할 기반 정책 정의
- 핵심 식별자: `role_policy_id`
- 주요 속성:
  - `role_code`: 역할 코드
  - `role_name`: 역할 이름
  - `policy_status`: 정책 상태
  - `description`: 정책 설명
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `RBAC` 확장점

### 7.2 `permission_scope`

- 목적: 데이터 범위/액션 범위 정책
- 핵심 식별자: `permission_scope_id`
- 주요 속성:
  - `role_policy_id`: 적용 역할 정책 참조
  - `resource_type`: 대상 자원 유형
  - `action_code`: 허용 액션 코드
  - `scope_rule`: 데이터 범위 규칙
  - `condition_expression`: 조건식
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 단순 메뉴 권한 이상 포함

### 7.3 `migration_request`

- 목적: 조직 변경/참조 갱신 실행 요청
- 핵심 식별자: `migration_request_id`
- 주요 속성:
  - `organization_change_case_id`: 원본 변경 케이스 참조
  - `request_type`: 요청 유형
  - `request_status`: 요청 상태
  - `requested_by`: 요청자
  - `approved_by`: 승인자
  - `executed_at`: 실행 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 승인/반려/실행 상태 분리

### 7.4 `exception_case`

- 목적: 공통 예외 모델
- 핵심 식별자: `exception_case_id`
- 주요 속성:
  - `exception_type`: 예외 유형
  - `related_entity_type`: 관련 엔터티 유형
  - `related_entity_id`: 관련 엔터티 식별자
  - `exception_status`: 예외 상태
  - `raised_by`: 예외 등록 주체
  - `raised_at`: 예외 발생 시각
  - `resolved_at`: 해결 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 품질/권한/연계/AI 예외 공통화

### 7.5 `audit_log`

- 목적: 감사 로그
- 핵심 식별자: `audit_log_id`
- 주요 속성:
  - `event_category`: 업무/운영 등 이벤트 분류
  - `event_type`: 이벤트 유형
  - `actor_type`: 행위자 유형
  - `actor_id`: 행위자 식별자
  - `target_entity_type`: 대상 엔터티 유형
  - `target_entity_id`: 대상 엔터티 식별자
  - `event_at`: 이벤트 발생 시각
  - `event_payload_reference`: 세부 로그 참조
  - `created_at`: 생성 시각
- 비고: 업무/운영 이벤트 논리 구분 필요

### 7.6 `governance_decision`

- 목적: 승인/반려/예외 승인 결정
- 핵심 식별자: `governance_decision_id`
- 주요 속성:
  - `decision_type`: 결정 유형
  - `related_entity_type`: 대상 엔터티 유형
  - `related_entity_id`: 대상 엔터티 식별자
  - `decision_status`: 결정 상태
  - `decided_by`: 결정 주체
  - `decided_at`: 결정 시각
  - `decision_note`: 결정 메모
  - `created_at`: 생성 시각
- 비고: 운영 판단 이력 공통 모델

## 8. `AI` 보조 도메인 엔터티

### 8.1 `ai_review_draft`

- 목적: `AI` 코드리뷰 초안
- 핵심 식별자: `ai_review_draft_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `draft_status`: 초안 상태
  - `summary_text`: 초안 요약
  - `review_result_reference`: 상세 결과 참조
  - `generated_at`: 생성 시각
  - `reviewed_by`: 검토자
  - `reviewed_at`: 검토 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 업무 상태와 분리된 초안 엔터티

### 8.2 `ai_test_draft`

- 목적: `AI` 단위테스트 초안/실행 보조
- 핵심 식별자: `ai_test_draft_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `draft_status`: 초안 상태
  - `generated_test_reference`: 생성된 테스트 참조
  - `execution_status`: 실행 상태
  - `execution_result_reference`: 실행 결과 참조
  - `generated_at`: 생성 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 생성/실행 결과 포함

### 8.3 `ai_ci_plan_draft`

- 목적: `AI` `CI` 플랜 초안
- 핵심 식별자: `ai_ci_plan_draft_id`
- 주요 속성:
  - `work_item_id`: 관련 업무 항목 참조
  - `draft_status`: 초안 상태
  - `template_code`: 사용한 템플릿 코드
  - `plan_reference`: 생성 계획 참조
  - `generated_at`: 생성 시각
  - `reviewed_by`: 검토자
  - `reviewed_at`: 검토 시각
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 템플릿/출력 형식 연결

### 8.4 `ai_input_context`

- 목적: `AI` 입력 문맥 참조
- 핵심 식별자: `ai_input_context_id`
- 주요 속성:
  - `context_type`: 입력 문맥 유형
  - `context_reference`: 실제 입력 데이터 참조
  - `content_hash`: 입력 내용 해시
  - `captured_at`: 캡처 시각
  - `created_at`: 생성 시각
- 비고: 전문 저장보다 참조/해시 우선

### 8.5 `rule_set_reference`

- 목적: 코딩룰/정책 규칙셋 참조
- 핵심 식별자: `rule_set_reference_id`
- 주요 속성:
  - `rule_set_type`: 규칙셋 유형
  - `rule_set_version`: 규칙셋 버전
  - `source_reference`: 원본 위치 참조
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 버전 추적 포함

### 8.6 `ai_execution_profile`

- 목적: 모델, 프롬프트, 실행 정책 정보
- 핵심 식별자: `ai_execution_profile_id`
- 주요 속성:
  - `model_name`: 사용 모델 이름
  - `prompt_version`: 프롬프트 버전
  - `policy_version`: 실행 정책 버전
  - `temperature_profile`: 생성 파라미터 프로파일
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 감사 추적 필수 후보

## 9. 프로세스 모델 메타모델 엔터티

### 9.1 `process_model_definition`

- 목적: 프로세스 모델 정의
- 핵심 식별자: `process_model_definition_id`
- 주요 속성:
  - `model_code`: 프로세스 모델 코드
  - `model_name`: 프로세스 모델 이름
  - `model_category`: 프로세스 모델 분류
  - `description`: 프로세스 모델 설명
  - `is_builtin`: 기본 제공 모델 여부
  - `is_active`: 활성 여부
  - `version`: 정의 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 현재는 `Agile`, `V-Model` 이 대표 예시이며 향후 신규 모델 추가 가능

### 9.2 `workflow_scheme`

- 목적: 상태 체계/전이 규칙 정의
- 핵심 식별자: `workflow_scheme_id`
- 주요 속성:
  - `process_model_definition_id`: 소속 프로세스 모델 참조
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `description`: 스킴 설명
  - `initial_status_code`: 초기 상태 코드
  - `is_default`: 기본 스킴 여부
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 공통 상태 집합, 상세 상태 정의, 전이 규칙의 상위 컨테이너

### 9.3 `workflow_status_definition`

- 목적: 스킴 내 상세 상태와 공통 상태 매핑 정의
- 핵심 식별자: `workflow_status_definition_id`
- 주요 속성:
  - `workflow_scheme_id`: 소속 상태 스킴 참조
  - `status_code`: 상세 상태 코드
  - `status_name`: 상세 상태 이름
  - `mapped_common_status`: 대응 공통 상태
  - `status_category`: 진행/대기/완료/취소 등 상태 분류
  - `is_initial`: 초기 상태 여부
  - `is_terminal`: 종료 상태 여부
  - `sequence_no`: 표시 순서
  - `visibility_rule`: 역할 또는 뷰별 표시 규칙
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: `work_item.current_detailed_status_code` 와 직접 연결되는 기준 정의

### 9.4 `workflow_transition_definition`

- 목적: 상태 간 이동 가능 조건과 전이 규칙 정의
- 핵심 식별자: `workflow_transition_definition_id`
- 주요 속성:
  - `workflow_scheme_id`: 소속 상태 스킴 참조
  - `transition_code`: 전이 코드
  - `transition_name`: 전이 이름
  - `from_status_code`: 시작 상세 상태 코드
  - `to_status_code`: 도착 상세 상태 코드
  - `allowed_role_rule`: 전이 허용 역할 규칙
  - `condition_expression`: 전이 가능 조건식
  - `trigger_type`: 사용자/시스템/연계 등 트리거 유형
  - `is_default`: 기본 전이 여부
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 상태 전이 가능 여부와 이력 해석 기준을 제공한다

### 9.5 `planning_scheme`

- 목적: `iteration`, `release`, `milestone`, `WBS` 사용 규칙
- 핵심 식별자: `planning_scheme_id`
- 주요 속성:
  - `process_model_definition_id`: 소속 프로세스 모델 참조
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `uses_iteration`: 반복 단위 사용 여부
  - `uses_release`: 릴리스 사용 여부
  - `uses_milestone`: 마일스톤 사용 여부
  - `uses_wbs`: `WBS` 사용 여부
  - `allows_parallel_tracks`: 병렬 트랙 허용 여부
  - `default_plan_unit_type`: 기본 계획 단위 유형
  - `allows_multi_iteration_link`: 하나의 `work_item` 이 여러 반복 단위에 연결될 수 있는지 여부
  - `allows_multi_release_link`: 하나의 `work_item` 이 여러 릴리스에 연결될 수 있는지 여부
  - `allows_multi_milestone_link`: 하나의 `work_item` 이 여러 마일스톤에 연결될 수 있는지 여부
  - `allows_multi_wbs_link`: 하나의 `work_item` 이 여러 `WBS` 노드에 연결될 수 있는지 여부
  - `default_link_role_policy`: 기본 연결 역할 정책
  - `is_default`: 기본 스킴 여부
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 계획 단위 허용 여부, 다중 연결 허용 범위, 기본 연결 정책 정의

### 9.6 `view_scheme`

- 목적: 보드, 간트, `WBS`, 릴리스 뷰 표현 규칙
- 핵심 식별자: `view_scheme_id`
- 주요 속성:
  - `process_model_definition_id`: 소속 프로세스 모델 참조
  - `scheme_code`: 스킴 코드
  - `scheme_name`: 스킴 이름
  - `default_board_type`: 기본 보드 유형
  - `supports_gantt`: 간트 지원 여부
  - `supports_wbs`: `WBS` 지원 여부
  - `supports_release_board`: 릴리스 보드 지원 여부
  - `default_grouping_rule`: 기본 그룹핑 규칙
  - `default_sort_rule`: 기본 정렬 규칙
  - `version`: 스킴 버전
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 읽기 모델 기본 규칙 정의

### 9.7 `process_model_component_link`

- 목적: 프로세스 모델과 스킴 간 연결
- 핵심 식별자: `process_model_component_link_id`
- 주요 속성:
  - `process_model_definition_id`: 프로세스 모델 참조
  - `component_type`: 연결 컴포넌트 유형
  - `component_id`: 연결 컴포넌트 식별자
  - `is_default`: 기본 연결 여부
  - `effective_from`: 적용 시작 시점
  - `effective_to`: 적용 종료 시점
  - `created_at`: 생성 시각
  - `updated_at`: 최종 수정 시각
- 비고: 어떤 스킴을 조합하는지 정의

## 10. 다음 검토 포인트

- `work_item` 과 `project` 관계를 `1:N` 으로 고정할지, 일부 `N:N` 허용 케이스를 열어둘지
- `work_item_type` 기본 집합을 전사 표준으로 둘지, 프로젝트별 확장을 허용할지
- `project_scope`, `project_work_item_policy` 를 초기 릴리스 범위에 포함할지
- `workflow_status_definition`, `workflow_transition_definition` 을 초기 릴리스 범위에 포함할지
- `planning_scheme` 의 다중 연결 규칙을 프로젝트별 오버라이드까지 허용할지
- 읽기 모델 성격이 강한 엔터티를 별도 스냅샷 모델로 더 분리할지

## 11. 초기 릴리스 우선순위 초안

초기 릴리스는 공통 `work_item` 운영, 상태 관리, 계획 연결, 기본 조회와 권한/감사 최소선을 우선 구현하는 기준으로 본다. 따라서 아래 구분은 “초기 구현 필수”와 “후속 확장 또는 선택 적용” 기준으로 정리한다.

### 11.1 초기 릴리스 필수 엔터티

- `project`
  프로젝트 단위 운영과 소유 경계의 기준이므로 필수
- `work_item`
  모든 업무 관리의 기준 엔터티이므로 필수
- `work_item_type`
  `epic`, `story`, `task`, `sub-task` 같은 기본 유형 구분에 필요하므로 필수
- `work_item_hierarchy`
  상하위 구조 탐색과 집계에 필요하므로 필수
- `work_item_status_history`
  현재 상태만으로는 감사와 흐름 추적이 부족하므로 필수
- `project_process_model`
  프로젝트별 프로세스 모델과 스킴 상속의 기준이므로 필수
- `workflow_scheme`
  상태 체계의 상위 컨테이너이므로 필수
- `workflow_status_definition`
  상세 상태와 공통 상태 매핑의 기준이므로 필수
- `workflow_transition_definition`
  상태 전이 허용 규칙과 이력 해석 기준에 필요하므로 필수
- `planning_scheme`
  반복 단위, 릴리스, 마일스톤, `WBS` 허용 범위를 정하는 기준이므로 필수
- `work_item_plan_link`
  계획 단위와 `work_item` 연결 사실을 저장하는 최소 모델이므로 필수
- `iteration`
  `Agile` 계열 프로젝트의 기본 운영 단위이므로 초기 범위 포함 권장
- `release`
  배포 범위 관리와 품질/승인 집계의 기준이므로 필수
- `milestone`
  일정 기준점과 `V-Model` 계열 추적에 필요하므로 필수
- `role_policy`
  역할 기반 접근 제어의 최소 기준이므로 필수
- `permission_scope`
  데이터 범위와 액션 범위를 구체화하는 최소 정책 모델이므로 필수
- `audit_log`
  업무/운영 행위 감사의 최소선이므로 필수
- `organization_master`
  소유 조직, 승인 라우팅, 권한 범위 판단에 필요하므로 필수
- `workforce_master`
  담당자, 등록자, 승인자 식별에 필요하므로 필수
- `role_assignment`
  조직장, 승인권자, 책임자 연결에 필요하므로 필수

### 11.2 초기 릴리스 선택 포함 엔터티

- `wbs_node`
  `V-Model` 또는 단계형 계획 표현이 강하게 필요하면 초기 포함, 아니면 후속 확장 가능
- `integration_system`
  외부 연계 범위가 초기 릴리스에 포함되면 필수, 수기 입력 중심 시작이면 후속 가능
- `integration_job`
  자동 동기화까지 초기 포함 시 필요
- `integration_run`
  연계 실행 이력 추적이 필요할 때 초기 포함 권장
- `normalized_record_reference`
  초기부터 표준화 추적을 강하게 요구하면 포함
- `exception_case`
  예외 승인 프로세스를 초기부터 통합 관리하려면 포함
- `governance_decision`
  운영 승인/반려 이력을 공통 모델로 바로 묶을 경우 포함

### 11.3 후속 확장 엔터티

- `project_scope`
  관리 외 프로젝트 전환과 세부 범위 통제가 본격화될 때 확장
- `project_work_item_policy`
  프로젝트별 규칙 커스터마이징이 필요해질 때 확장
- `work_item_external_reference`
  외부 시스템 매핑 자동화가 본격화될 때 확장
- `organization_history`
  조직 개편 이력 관리가 고도화될 때 확장
- `workforce_assignment_history`
  소속 이력 기반 분석이 필요해질 때 확장
- `absence_calendar_entry`
  개인/조직 가용성 계산이 고도화될 때 확장
- `identity_mapping`
  복수 원천 시스템 식별자 정합성 관리가 필요해질 때 확장
- `organization_change_case`
  조직 변경 감지와 마이그레이션 검토 프로세스를 자동화할 때 확장
- `integration_endpoint`
  연계 설정을 세분화할 때 확장
- `raw_ingestion_event`
  원시 수집 메타데이터를 체계적으로 남길 때 확장
- `sync_error`
  연계 실패와 재시도 관리를 본격화할 때 확장
- `review_summary`
  코드리뷰 메타데이터 통합을 초기 범위에 넣지 않을 경우 후속
- `build_summary`
  빌드/배포 연계가 본격화될 때 확장
- `artifact_metadata`
  산출물 메타데이터 관리가 필요해질 때 확장
- `quality_signal`
  정적분석과 품질 신호를 통합할 때 확장
- `test_execution_summary`
  테스트 실행 요약을 내재화할 때 확장
- `defect_link`
  결함 연결 관리를 내재화할 때 확장
- `release_readiness_snapshot`
  릴리스 준비도 집계 모델을 도입할 때 확장
- `quality_gate_decision`
  품질 게이트 승인 이력을 체계화할 때 확장
- `migration_request`
  조직 변경 실행 요청을 운영 모델로 고도화할 때 확장
- `ai_review_draft`
  `AI` 리뷰 보조를 도입할 때 확장
- `ai_test_draft`
  `AI` 테스트 보조를 도입할 때 확장
- `ai_ci_plan_draft`
  `AI` `CI` 플랜 보조를 도입할 때 확장
- `ai_input_context`
  `AI` 입력 문맥 감사가 필요해질 때 확장
- `rule_set_reference`
  규칙셋 버전 추적이 필요해질 때 확장
- `ai_execution_profile`
  모델/프롬프트 감사가 필요해질 때 확장

### 11.4 구현 순서 제안

1. `project`, `work_item`, `work_item_type`, `work_item_hierarchy`
2. `project_process_model`, `workflow_scheme`, `workflow_status_definition`, `workflow_transition_definition`
3. `planning_scheme`, `iteration`, `release`, `milestone`, `work_item_plan_link`
4. `organization_master`, `workforce_master`, `role_assignment`, `role_policy`, `permission_scope`, `audit_log`
5. 이후 연계, 품질, 운영 고도화, `AI` 보조 엔터티 순으로 확장

## 12. 초기 릴리스 필수 엔터티 관계 초안

본 절은 초기 릴리스 필수 엔터티만 대상으로 관계 방향과 기본 `cardinality` 를 정리한 초안이다. 물리 `ERD` 를 확정하기 전의 논리 관계 기준으로 사용한다.

### 12.1 업무 기준 관계

- `project 1:N work_item`
  하나의 프로젝트는 여러 업무 항목을 가질 수 있고, 업무 항목은 기본적으로 하나의 프로젝트에 소속된다.
- `work_item_type 1:N work_item`
  하나의 업무 항목 유형은 여러 업무 항목에 적용될 수 있다.
- `work_item 1:N work_item_hierarchy` as parent
  상위 업무 항목은 여러 하위 관계를 가질 수 있다.
- `work_item 1:N work_item_hierarchy` as child
  하위 업무 항목도 하나의 상위 관계에 참여하며, 초기 모델에서는 단일 부모를 기본 원칙으로 보는 편이 적절하다.
- `work_item 1:N work_item_status_history`
  하나의 업무 항목은 여러 상태 변경 이력을 가진다.
- `work_item 1:N work_item_plan_link`
  하나의 업무 항목은 여러 계획 단위와 연결될 수 있다. 다만 실제 허용 범위는 `planning_scheme` 이 제어한다.

### 12.2 프로세스 모델과 상태 관계

- `project 1:N project_process_model`
  프로젝트는 시점에 따라 여러 프로세스 모델 설정을 가질 수 있으나 동일 시점 기본 활성 모델은 하나만 둔다.
- `process_model_definition 1:N project_process_model`
  하나의 프로세스 모델 정의가 여러 프로젝트 설정에 재사용될 수 있다.
- `workflow_scheme 1:N project_process_model`
  프로젝트 프로세스 설정은 하나의 상태 스킴을 참조한다.
- `planning_scheme 1:N project_process_model`
  프로젝트 프로세스 설정은 하나의 계획 스킴을 참조한다.
- `workflow_scheme 1:N workflow_status_definition`
  하나의 상태 스킴은 여러 상세 상태 정의를 가진다.
- `workflow_scheme 1:N workflow_transition_definition`
  하나의 상태 스킴은 여러 상태 전이 규칙을 가진다.
- `workflow_transition_definition 1:N work_item_status_history`
  하나의 전이 규칙은 여러 상태 변경 이력에서 재사용될 수 있다.

### 12.3 계획 단위 관계

- `planning_scheme 1:N iteration`
  반복 단위를 사용하는 프로세스 모델에서는 같은 계획 스킴 아래 여러 반복 단위가 생성될 수 있다.
- `planning_scheme 1:N release`
  릴리스 단위를 사용하는 프로세스 모델에서는 같은 계획 스킴 아래 여러 릴리스가 생성될 수 있다.
- `planning_scheme 1:N milestone`
  마일스톤 단위를 사용하는 프로세스 모델에서는 같은 계획 스킴 아래 여러 마일스톤이 생성될 수 있다.
- `project 1:N iteration`
  하나의 프로젝트는 여러 반복 단위를 가질 수 있다.
- `project 1:N release`
  하나의 프로젝트는 여러 릴리스를 가질 수 있다.
- `project 1:N milestone`
  하나의 프로젝트는 여러 마일스톤을 가질 수 있다.
- `iteration 1:N work_item_plan_link`
  반복 단위는 여러 업무 항목 연결을 가질 수 있다.
- `release 1:N work_item_plan_link`
  릴리스는 여러 업무 항목 연결을 가질 수 있다.
- `milestone 1:N work_item_plan_link`
  마일스톤은 여러 업무 항목 연결을 가질 수 있다.

### 12.4 조직과 권한 관계

- `organization_master 1:N project`
  하나의 조직이 여러 프로젝트를 주관할 수 있다.
- `organization_master 1:N work_item`
  하나의 조직이 여러 업무 항목의 소유 조직이 될 수 있다.
- `organization_master 1:N workforce_master`
  하나의 조직에 여러 인력이 소속될 수 있다.
- `workforce_master 1:N project` as owner
  하나의 인력이 여러 프로젝트의 책임자가 될 수 있다.
- `workforce_master 1:N work_item` as assignee/reporter
  하나의 인력이 여러 업무 항목의 담당자 또는 등록자가 될 수 있다.
- `workforce_master 1:N role_assignment`
  하나의 인력이 여러 역할 배정 기록을 가질 수 있다.
- `role_policy 1:N permission_scope`
  하나의 역할 정책은 여러 권한 범위 규칙을 가진다.
- `role_assignment N:1 role_policy`
  역할 배정은 하나의 역할 정책에 매핑되는 구조로 상세화하는 편이 적절하다.

### 12.5 감사 관계

- `project 1:N audit_log`
  프로젝트 수준 행위는 감사 로그로 남길 수 있어야 한다.
- `work_item 1:N audit_log`
  업무 항목 수준 행위는 감사 로그로 남길 수 있어야 한다.
- `workforce_master 1:N audit_log` as actor
  인력은 여러 감사 이벤트의 행위자가 될 수 있다.

### 12.6 구현 시 유의사항

- `work_item_hierarchy` 는 초기 모델에서 순환 참조를 허용하지 않는 제약이 필요하다.
- `work_item_status_history` 는 현재 상태와 모순되지 않도록 최신 이력과 `work_item` 본체 상태를 일관되게 유지해야 한다.
- `project_process_model` 은 동일 시점에 `is_primary=true` 인 레코드가 하나만 존재하도록 제약이 필요하다.
- `work_item_plan_link` 의 다중 연결 허용 여부는 `planning_scheme` 규칙과 함께 검증되어야 한다.
