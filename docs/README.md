# 문서 위키 홈

- 문서 목적: 저장소 문서를 wiki 형태로 탐색할 수 있는 홈과 문서 체계를 제공한다.
- 범위: `docs/` 아래 모든 문서
- 대상 독자: 기획자, 개발자, 운영자, 프로젝트 참여자
- 상태: draft
- 최종 수정일: 2026-04-16
- 관련 문서: `AGENTS.md`

## 문서 읽기 흐름

문서를 처음 읽는 경우 다음 순서를 기본 경로로 사용한다.

1. 프로젝트 배경과 방향 확인: `overview`
2. 컨셉 수준 요구 확인: `CRS`
3. 상세 요구 확인: `SRS`
4. 아키텍처 초안과 설계 계획 확인: `architecture`
5. 운영 규칙과 작업 이력 확인: `operations`

## 빠른 이동

- 프로젝트 개요: [overview/README.md](./overview/README.md)
- 요구사항 위키: [requirements/README.md](./requirements/README.md)
- 아키텍처 위키: [architecture/README.md](./architecture/README.md)
- 연계 위키: [integrations/README.md](./integrations/README.md)
- 운영 위키: [operations/README.md](./operations/README.md)
- 작업 백로그 인덱스: [operations/work_backlog.md](./operations/work_backlog.md)
- 세션 인계 문서: [operations/session_handoff.md](./operations/session_handoff.md)
- 워크플로우 개발 계획: [operations/workflow_development_plan.md](./operations/workflow_development_plan.md)
- 표준 작업 워크플로우 초안: [operations/standard_workflow_draft.md](./operations/standard_workflow_draft.md)
- 작업 진행 워크플로우: [operations/workflow_task_execution.md](./operations/workflow_task_execution.md)
- 문서화 및 동기화 워크플로우: [operations/workflow_documentation_sync.md](./operations/workflow_documentation_sync.md)
- 세션 인계 워크플로우: [operations/workflow_session_handoff.md](./operations/workflow_session_handoff.md)
- 검증 워크플로우: [operations/workflow_validation.md](./operations/workflow_validation.md)
- 코드 색인 전략: [operations/code_index_strategy.md](./operations/code_index_strategy.md)
- AI agent 빠른 참조 문서: [operations/ai_agent_quickstart.md](./operations/ai_agent_quickstart.md)
- 브랜치 병합 문서 정책: [operations/branch_merge_document_policy.md](./operations/branch_merge_document_policy.md)
- 개발 환경 및 테스트 환경 가이드: [operations/development_environment.md](./operations/development_environment.md)
- 환경 기록 위키: [operations/environments/README.md](./operations/environments/README.md)
- 운영 UI 렌더 점검 및 기능 시나리오: [operations/backend_operation_ui_render_and_scenarios.md](./operations/backend_operation_ui_render_and_scenarios.md)

## 카테고리 안내

- `overview/`: 프로젝트 목적, 배경, 목표, 범위, 이해관계자, 핵심 용어
- `requirements/`: `CRS`, `SRS`, 요구사항 초안, 상세 요구사항, 유스케이스, 우선순위
- `architecture/`: 시스템 구조, 컴포넌트, 데이터 흐름, 연계 설계
- `operations/`: 운영 정책, 배포 절차, 권한, 장애 대응, 작업 관리, 백로그
- `integrations/`: 외부 시스템별 연계 정책, API, 데이터 매핑
- `decisions/`: 설계 의사결정과 ADR

## 현재 핵심 문서

- 프로젝트 개요: [overview/project_overview.md](./overview/project_overview.md)
- 프로젝트 기안서: [overview/project_proposal.md](./overview/project_proposal.md)
- 프로젝트 요약본: [overview/project_summary.md](./overview/project_summary.md)
- 프로젝트 실행 계획서: [overview/project_execution_plan.md](./overview/project_execution_plan.md)
- 프로젝트 WBS 초안: [overview/project_wbs.md](./overview/project_wbs.md)
- 프로젝트 일정 초안: [overview/project_timeline_draft.md](./overview/project_timeline_draft.md)
- 프로젝트 예산 초안: [overview/project_budget_draft.md](./overview/project_budget_draft.md)
- 프로젝트 발표 목차: [overview/project_presentation_outline.md](./overview/project_presentation_outline.md)
- 역할 기반 UX 구조 방향: [overview/role_based_ux_direction.md](./overview/role_based_ux_direction.md)
- 통합 데이터 관리 UI 구조 방향: [overview/integrated_data_management_ui_direction.md](./overview/integrated_data_management_ui_direction.md)
- 통합 중앙 관리 시스템 CRS: [requirements/system_crs.md](./requirements/system_crs.md)
- 통합 중앙 관리 시스템 SRS: [requirements/system_srs.md](./requirements/system_srs.md)
- 통합 중앙 관리 시스템 아키텍처 초안: [architecture/system_architecture_draft.md](./architecture/system_architecture_draft.md)
- 통합 백엔드 구현 현황 스냅샷: [architecture/current_backend_implementation_status_summary.md](./architecture/current_backend_implementation_status_summary.md)
- SDLC 시스템 카테고리 정의: [integrations/sdlc_system_categories.md](./integrations/sdlc_system_categories.md)

## 운영 원칙

- 문서는 한국어로 작성하되 기술 식별자는 원문을 유지한다.
- 파일명은 `snake_case`를 사용한다.
- `docs/README.md` 를 wiki 홈으로 유지하고, 카테고리별 인덱스 문서를 통해 하위 문서로 이동할 수 있게 구성한다.
- 핵심 문서는 관련 문서와 다음 문서 링크를 유지해 탐색 경로가 끊기지 않게 한다.
- 확정되지 않은 내용은 `가정` 또는 `미정`으로 표시한다.
- 새로운 카테고리를 추가하면 `AGENTS.md`와 이 문서를 함께 갱신한다.
- 세션 시작 시에는 작업 전에 백로그 인덱스와 최근 날짜 백로그를 먼저 확인해 진행 중 작업과 후속 작업을 파악한다.
- 작업 시작 전 브리핑과 백로그 등록을 수행한다.
- 작업 중간 현황과 완료 결과는 날짜별 백로그 문서 `docs/operations/backlog/YYYY-MM-DD.md` 에 반영한다.
- `docs/operations/work_backlog.md` 는 날짜별 백로그 문서 인덱스와 운영 기준 문서로 사용한다.
- 과제 컨셉이 먼저 정리되는 작업은 `CRS`를 우선 작성하고, 이후 세분화한 내용을 `SRS`에 반영한다.
- 요구사항 문서가 재정리될 때는 초안 문서를 유지하기보다 `CRS`와 `SRS` 기준으로 역할을 분리해 중복을 줄인다.
