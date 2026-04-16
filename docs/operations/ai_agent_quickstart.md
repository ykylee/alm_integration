# AI Agent 빠른 참조 문서

- 문서 목적: 새 세션의 AI agent 가 최소 문서 집합과 빠른 탐색 키를 사용해 현재 저장소 작업을 시작할 수 있도록 진입 경로를 제공한다.
- 범위: 세션 시작 순서, 질문 유형별 읽기 경로, 빠른 파일 탐색 키, 작업 전후 체크리스트
- 대상 독자: AI agent, 개발자, 문서 작성자
- 상태: draft
- 최종 수정일: 2026-04-16
- 관련 문서: `docs/operations/session_handoff.md`, `docs/operations/workflow_task_execution.md`, `docs/operations/workflow_documentation_sync.md`, `docs/operations/workflow_validation.md`, `docs/operations/code_index_strategy.md`, `docs/README.md`, `AGENTS.md`

## 1. 문서 개요

이 문서는 새 세션의 AI agent 가 긴 문서를 처음부터 끝까지 다시 읽지 않고도 현재 기준선과 다음 탐색 경로를 빠르게 복원하게 하는 라우팅 문서다.

이 문서는 기준 사실을 새로 정의하지 않는다. 반드시 원문 기준 문서로 이동시키는 진입점 역할만 담당한다.

## 2. 가장 먼저 읽을 문서

새 세션 시작 시 기본 순서는 아래와 같다.

1. [session_handoff.md](./session_handoff.md)
2. [work_backlog.md](./work_backlog.md)
3. 최신 날짜 백로그
4. [standard_workflow_draft.md](./standard_workflow_draft.md)
5. 작업 성격에 맞는 상세 워크플로우 문서

이 순서를 쓰는 이유:

- `session_handoff.md` 는 현재 상태 요약 문서다.
- `work_backlog.md` 와 날짜별 백로그는 상세 근거와 최근 작업 이력을 제공한다.
- `standard_workflow_draft.md` 는 절차 허브 문서다.
- 상세 워크플로우 문서는 실제 작업 방식의 기준 문서다.

## 3. 작업 유형별 읽기 경로

### 3.1 문서 작업을 할 때

1. [session_handoff.md](./session_handoff.md)
2. [workflow_documentation_sync.md](./workflow_documentation_sync.md)
3. 관련 카테고리 `README.md`
4. 해당 기준 문서
5. 오늘 날짜 백로그

주로 확인할 것:

- 어떤 문서가 기준 사실 문서인지
- 허브와 인덱스 문서까지 같이 갱신해야 하는지
- 이번 변경이 세션 인계 문서에 반영돼야 하는지

### 3.2 코드 구현 또는 수정 작업을 할 때

1. [session_handoff.md](./session_handoff.md)
2. [workflow_task_execution.md](./workflow_task_execution.md)
3. [workflow_documentation_sync.md](./workflow_documentation_sync.md)
4. [workflow_validation.md](./workflow_validation.md)
5. 관련 요구사항/아키텍처 문서

주로 확인할 것:

- 영향 문서
- 검증 수준 선택 기준
- 결과 기록과 다음 세션 시작 포인트

### 3.3 검증 또는 실행 확인을 할 때

1. [workflow_validation.md](./workflow_validation.md)
2. [development_environment.md](./development_environment.md)
3. 현재 호스트 환경 문서
4. 관련 백로그와 실행 증적

주로 확인할 것:

- 로컬 검증과 격리 검증의 구분
- 현재 호스트 제약
- 증적 저장 위치

### 3.4 문서 구조나 탐색 체계를 정리할 때

1. [docs/README.md](../README.md)
2. [README.md](./README.md)
3. [standard_workflow_draft.md](./standard_workflow_draft.md)
4. [workflow_documentation_sync.md](./workflow_documentation_sync.md)
5. [code_index_strategy.md](./code_index_strategy.md)

주로 확인할 것:

- 새 문서를 만든 뒤 허브와 카테고리 인덱스까지 연결했는지
- stale 설명이 남아 있지 않은지
- AI용 빠른 참조 경로도 갱신 대상인지

## 4. 핵심 기준 문서 맵

- 현재 상태 요약: [session_handoff.md](./session_handoff.md)
- 날짜별 작업 이력: [work_backlog.md](./work_backlog.md)
- 작업 절차 허브: [standard_workflow_draft.md](./standard_workflow_draft.md)
- 작업 실행 절차: [workflow_task_execution.md](./workflow_task_execution.md)
- 문서 동기화 절차: [workflow_documentation_sync.md](./workflow_documentation_sync.md)
- 세션 인계 절차: [workflow_session_handoff.md](./workflow_session_handoff.md)
- 검증 절차: [workflow_validation.md](./workflow_validation.md)
- 색인 구조 기준: [code_index_strategy.md](./code_index_strategy.md)
- 문서 위키 홈: [../README.md](../README.md)

## 5. 빠른 파일 탐색 키

현재 저장소에서 자주 찾을 가능성이 높은 경로와 검색 키는 아래와 같다.

### 5.1 운영 문서

- `docs/operations/session_handoff.md`
- `docs/operations/backlog/`
- `docs/operations/workflow_`
- `docs/operations/code_index_strategy.md`

추천 검색 키:

- `session_handoff`
- `TASK-`
- `workflow_`
- `최종 수정일`

### 5.2 요구사항 및 아키텍처

- `docs/requirements/system_crs.md`
- `docs/requirements/system_srs.md`
- `docs/architecture/system_architecture_draft.md`
- `docs/architecture/current_backend_implementation_status_summary.md`

추천 검색 키:

- `CRS`
- `SRS`
- `architecture`
- `implementation_status`

### 5.3 구현 및 검증

- `backend/`
- `src/`
- `tests/`
- `docker/`
- `Makefile`
- `docker-compose.yml`

추천 검색 키:

- `cargo test`
- `pytest`
- `podman`
- `admin.html`
- `organization.html`

## 6. AI agent 작업 전 체크리스트

- 현재 세션의 주 작업 축이 무엇인지 `session_handoff.md` 에서 확인했는가
- 최신 날짜 백로그에서 마지막 완료 작업과 후속 작업을 확인했는가
- 이번 작업의 영향 문서를 식별했는가
- 작업 성격에 맞는 상세 워크플로우 문서를 열었는가
- 환경 제약이 있으면 호스트 문서를 확인했는가

## 7. AI agent 작업 후 체크리스트

- 오늘 날짜 백로그를 갱신했는가
- 허브 문서와 인덱스 링크를 갱신했는가
- 세션 인계 문서에 반영해야 할 상태 변화가 있는가
- 문서 경로나 역할이 바뀌었다면 quickstart 도 갱신했는가
- 다음 세션 시작 포인트와 남은 리스크를 남겼는가

## 8. 현재 기준선에서 다음 후속 문서

현재 quickstart 다음 우선순위 문서는 아래와 같다.

1. `workflow_automation_plan.md`
2. 도메인 또는 기능 단위 색인 문서 초안
3. `document_sync_policy.md` 의 독립 유지 여부 정리

병합 관련 기준 문서:

- [branch_merge_document_policy.md](./branch_merge_document_policy.md)

## 다음에 읽을 문서

- [세션 인계 문서](./session_handoff.md)
- [작업 진행 워크플로우](./workflow_task_execution.md)
- [문서화 및 동기화 워크플로우](./workflow_documentation_sync.md)
- [검증 워크플로우](./workflow_validation.md)
- [코드 색인 전략](./code_index_strategy.md)
- [브랜치 병합 문서 정책](./branch_merge_document_policy.md)
- [운영 위키](./README.md)
