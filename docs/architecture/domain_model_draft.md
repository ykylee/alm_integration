# 핵심 도메인 모델 초안

- 문서 목적: 통합 관리 플랫폼의 핵심 도메인 경계와 각 도메인이 책임지는 주요 데이터 개념을 정리한다.
- 범위: 과제/프로젝트, 조직/인력, 연계, 품질, 운영 통제, `AI` 보조 도메인의 책임 초안과 도메인 간 관계 원칙
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, 운영자
- 상태: draft
- 최종 수정일: 2026-04-06
- 관련 문서: `docs/architecture/system_architecture_draft.md`, `docs/architecture/domain_entity_definition_draft.md`, `docs/requirements/system_srs.md`, `docs/requirements/task_registration_process.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 개요 문서: [./system_architecture_draft.md](./system_architecture_draft.md)
- 엔터티 상세 초안: [./domain_entity_definition_draft.md](./domain_entity_definition_draft.md)

## 1. 목적

본 문서는 통합 관리 플랫폼을 구성하는 핵심 도메인을 분리해 각 도메인의 책임과 주요 데이터 단위를 정의한다. 이번 단계에서는 물리 테이블보다 도메인 경계와 식별자 책임을 먼저 고정한다. 특히 업무 식별자는 특정 프로세스 모델에 종속된 `task` 가 아니라 공통 `work_item` 을 기준으로 잡고, 현재 검토 중인 `Agile`, `V-Model` 뿐 아니라 향후 새롭게 추가될 수 있는 다른 운영 모델도 같은 모델 위에서 수용하는 방향을 전제로 한다.

엔터티별 목적, 식별자, 주요 속성은 [domain_entity_definition_draft.md](./domain_entity_definition_draft.md) 에서 별도 관리한다. 본 문서는 도메인 경계, 책임, 관계 원칙 중심으로 유지한다.

## 2. 업무 항목 및 프로젝트 도메인

이 도메인은 시스템의 최상위 업무 문맥을 담당한다.

- 공통 `work_item` 등록과 상태 전이
- 관리 외 프로젝트 등록과 관리 대상 전환
- `work_item` 과 프로젝트의 관계 관리
- `epic`, `story`, `task`, `sub-task` 간 계층 관계 관리
- `sprint`, `release`, 마일스톤, `WBS` 등 계획 단위와 `work_item` 연결
- 프로세스 모델별 운영 방식을 식별하고 확장 가능한 메타데이터로 관리
- `work_item` 기준의 요구사항, 변경, 빌드, 테스트, 품질 연결 기준 유지

핵심 데이터 책임:

- `project`
- `project_scope`
- `work_item`
- `work_item_hierarchy`
- `work_item_type`
- `project_work_item_policy`
- `project_process_model`
- `process_model_definition`
- `iteration`
- `release`
- `milestone`
- `wbs_node`
- `work_item_plan_link`
- `work_item_status_history`
- `work_item_external_reference`

## 3. 조직 및 인력 도메인

이 도메인은 원천 데이터와 내부 운영 마스터 사이의 차이를 흡수한다.

- 조직 구조, 조직장, 승인권자, 인력 소속 관리
- 인사 `DB` 원천 식별자와 내부 식별자 매핑
- 부재 일정과 조직 가용성 관리
- 조직 변경 영향 분석과 참조 갱신 준비

핵심 데이터 책임:

- `organization_master`
- `workforce_master`
- `organization_history`
- `workforce_assignment_history`
- `role_assignment`
- `absence_calendar_entry`
- `identity_mapping`
- `organization_change_case`

## 4. 연계 및 수집 도메인

이 도메인은 외부 시스템과의 통신, 수집 이력, 실패 재처리를 담당한다.

- 시스템별 연결 설정
- 동기화 작업 실행과 결과 기록
- 원천 데이터 수집과 원시 메타데이터 보존
- 실패, 재시도, 수동 재처리 관리

핵심 데이터 책임:

- `integration_system`
- `integration_endpoint`
- `integration_job`
- `integration_run`
- `raw_ingestion_event`
- `normalized_record_reference`
- `sync_error`

## 5. 품질 및 개발 문맥 도메인

이 도메인은 개발과 검증 문맥을 `work_item` 기준으로 재구성한다.

- 코드리뷰 메타데이터와 품질 신호 통합
- 빌드/산출물/배포 준비도 관리
- 정적분석, 테스트 결과, 결함 상태 연결
- 품질 게이트와 릴리스 준비도 판단 지원

핵심 데이터 책임:

- `review_summary`
- `build_summary`
- `artifact_metadata`
- `quality_signal`
- `test_execution_summary`
- `defect_link`
- `release_readiness_snapshot`
- `quality_gate_decision`

## 6. 운영 통제 및 감사 도메인

이 도메인은 관리 행위와 예외 처리를 통제한다.

- 권한, 역할, 메뉴 범위 관리
- 조직 변경 반영 승인과 실행 이력 관리
- 예외 승인, 정책 예외, 재평가 요청 추적
- 감사 로그와 운영 이력 보존

핵심 데이터 책임:

- `role_policy`
- `permission_scope`
- `migration_request`
- `exception_case`
- `audit_log`
- `governance_decision`

## 7. `AI` 보조 도메인

이 도메인은 초안 생성과 사람 검토 워크플로우를 담당한다.

- `AI` 리뷰 초안 생성과 상태 관리
- `AI` 단위테스트 생성/실행 보조 결과 관리
- `AI` `CI` 플랜 초안과 확정 전 검토 상태 관리
- 입력 근거, 규칙셋, 생성 시각, 결과 상태 기록

핵심 데이터 책임:

- `ai_review_draft`
- `ai_test_draft`
- `ai_ci_plan_draft`
- `ai_input_context`
- `rule_set_reference`
- `ai_execution_profile`

## 8. 프로세스 모델 메타모델 원칙

프로세스 모델은 고정 코드값이 아니라 정의 엔터티와 하위 스킴으로 구성하는 편이 적절하다.

- `process_model_definition` 은 현재 `Agile`, `V-Model` 같은 대표 예시뿐 아니라 향후 신규 모델을 수용해야 한다.
- `workflow_scheme` 은 공통 상태와 상세 상태 매핑, 전이 규칙의 기준이 된다.
- `workflow_status_definition` 은 상세 상태와 공통 상태의 매핑 기준이 된다.
- `workflow_transition_definition` 은 상태 간 이동 가능 조건과 허용 규칙의 기준이 된다.
- `planning_scheme` 은 `iteration`, `release`, `milestone`, `WBS` 같은 계획 단위 사용 규칙을 정의한다.
- `view_scheme` 은 보드, 간트, `WBS`, 릴리스 보드의 기본 표현 규칙을 정의한다.
- `process_model_component_link` 는 정의 모델과 각 스킴의 조합 관계를 관리한다.

세부 엔터티와 주요 속성은 [domain_entity_definition_draft.md](./domain_entity_definition_draft.md) 에서 별도 관리한다.

## 9. 핵심 관계 초안

- `project` 는 하나 이상의 `project_process_model` 을 가질 수 있으나, 기본 활성 모델은 1개로 제한하는 편이 적절하다.
- `project` 는 다수의 `work_item` 을 가진다.
- `work_item` 은 다수의 `work_item_hierarchy` 관계를 통해 계층을 구성한다.
- `work_item` 은 `work_item_plan_link` 를 통해 `iteration`, `release`, `milestone`, `wbs_node` 와 연결된다.
- `work_item` 의 계획 연결 방식은 기본적으로 `project_process_model` 과 `planning_scheme` 을 상속하며, 필요한 경우에만 예외 연결 규칙을 허용하는 편이 적절하다.
- `process_model_definition` 은 `workflow_scheme`, `planning_scheme`, `view_scheme` 조합으로 구체화된다.
- `work_item.current_detailed_status_code` 와 `work_item_status_history` 는 `workflow_status_definition`, `workflow_transition_definition` 과 일관되게 연결되어야 한다.
- `quality_signal`, `review_summary`, `build_summary`, `test_execution_summary` 는 `work_item` 또는 외부 변경/빌드 식별자에 연결된다.
- `exception_case`, `governance_decision`, `audit_log` 는 업무 엔터티와 운영 행위 양쪽에 참조될 수 있어야 한다.
- `ai_review_draft`, `ai_test_draft`, `ai_ci_plan_draft` 는 공통적으로 `ai_input_context`, `ai_execution_profile`, `rule_set_reference` 와 연결된다.

## 10. 도메인 간 관계 원칙

- 업무 항목/프로젝트 도메인은 업무 기준 식별자와 계층 관계를 소유한다.
- 조직/인력 도메인은 승인 라우팅과 가용성 계산에 필요한 기준 마스터를 소유한다.
- 연계 및 수집 도메인은 원천 데이터와 동기화 이력을 소유하되 업무 상태를 직접 확정하지 않는다.
- 운영 통제 도메인은 예외와 승인 상태를 일관되게 관리한다.
- `AI` 보조 도메인은 결과 초안과 근거를 관리하지만 최종 업무 판단은 소유하지 않는다.

추가 원칙:

- `epic`, `story`, `task`, `sub-task` 는 별도 엔터티보다 `work_item_type` 과 계층 관계로 표현하는 편이 우선이다.
- 현재의 `Agile`, `V-Model` 과 향후 신규 모델의 차이는 핵심 업무 엔터티가 아니라 `process_model`, 상태 모델, 계획 단위, 시각화 방식에서 흡수해야 한다.
- `process_model` 은 고정 코드값보다 별도 정의 엔터티 또는 이에 준하는 메타모델로 관리하는 편이 적절하다.
- 계획 단위 연결 사실은 저장 모델에 남기고, 보드/간트/릴리스/WBS 표현은 읽기 모델에서 재구성하는 편이 적절하다.
- `스크럼 보드`, `간트 차트`, `WBS` 차트, 릴리스 보드는 같은 `work_item` 데이터를 서로 다른 읽기 모델로 재구성한 결과로 본다.

## 11. 후속 상세화 후보

- 엔터티 속성/관계 검토 결과 반영
- 식별자/매핑 규칙 문서
- 도메인 간 이벤트/상태 전이 초안
- 프로세스 모델 정의서
- 상태 전이 스킴 정의서
