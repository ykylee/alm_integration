# 문서 동기화 정책

- 문서 목적: 코드, 설계, 운영 규칙, 문서 구조 변경이 발생했을 때 어떤 기준 문서와 허브 문서를 함께 갱신해야 하는지 빠르게 판단할 수 있는 정책 기준을 제공한다.
- 범위: 변경 유형 분류, 기준 문서 우선순위, 동기화 순서, 허브/인덱스 갱신 규칙, 예외 처리
- 대상 독자: 개발자, 문서 작성자, 운영자, AI agent, 리뷰어
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `docs/operations/workflow_documentation_sync.md`, `docs/operations/standard_workflow_draft.md`, `docs/README.md`, `AGENTS.md`

## 1. 문서 개요

이 문서는 상세 절차 본문인 [문서화 및 동기화 워크플로우](./workflow_documentation_sync.md) 를 대체하지 않는다. 그 문서가 전체 판단 흐름을 설명한다면, 이 문서는 실제 작업 중 “이번 변경으로 무엇을 같이 봐야 하는가”를 빠르게 고르는 정책표 역할을 한다.

## 2. 기본 원칙

- 기준 사실은 가능한 한 한 문서에서 먼저 확정한다.
- 허브 문서와 인덱스 문서는 기준 사실을 복제하지 않고 링크와 요약만 제공한다.
- 상세 문서를 만들거나 이동했다면 `docs/README.md` 와 카테고리 `README.md` 까지 갱신해야 동기화가 끝난다.
- 세션 상태 문서와 작업 이력 문서는 사실 문서를 대체하지 않는다.

## 3. 변경 유형별 기준 문서

### 3.1 프로젝트 방향 또는 범위 변경

- 먼저 볼 문서:
  - `docs/overview/*`
  - `docs/requirements/system_crs.md`
  - `docs/requirements/system_srs.md`

### 3.2 기능 요구 또는 정책 변경

- 먼저 볼 문서:
  - `docs/requirements/system_crs.md`
  - `docs/requirements/system_srs.md`
  - 관련 요구사항 상세 문서

### 3.3 시스템 구조 또는 구현 책임 변경

- 먼저 볼 문서:
  - `docs/architecture/*`
  - 관련 요구사항 문서
- 함께 확인할 문서:
  - `docs/operations/code_index_strategy.md`
  - `docs/operations/ai_agent_quickstart.md`

### 3.4 운영 절차 변경

- 먼저 볼 문서:
  - `docs/operations/standard_workflow_draft.md`
  - 관련 상세 워크플로우 문서
- 함께 확인할 문서:
  - `AGENTS.md`
  - `docs/README.md`
  - `docs/operations/README.md`

### 3.5 환경 또는 검증 기준 변경

- 먼저 볼 문서:
  - `docs/operations/development_environment.md`
  - `docs/operations/workflow_validation.md`
  - 호스트별 환경 문서

### 3.6 문서 구조 또는 탐색 경로 변경

- 먼저 볼 문서:
  - `docs/README.md`
  - 해당 카테고리 `README.md`
  - 관련 허브 문서
- 함께 확인할 문서:
  - `docs/operations/ai_agent_quickstart.md`
  - `docs/operations/session_handoff.md`

## 4. 동기화 순서

1. 기준 사실 문서를 먼저 갱신한다.
2. 관련 상세 문서를 갱신한다.
3. 허브 문서와 카테고리 인덱스를 갱신한다.
4. 세션 인계와 날짜별 백로그에 결과를 반영한다.
5. AI 빠른 참조나 색인 문서가 영향을 받으면 마지막에 갱신한다.

## 5. 허브 문서 최소 갱신 규칙

다음 상황에서는 허브 또는 인덱스 문서 갱신이 필수다.

- 새 문서를 만들었을 때
- 문서를 이동하거나 이름을 바꿨을 때
- 카테고리 책임이 달라졌을 때
- 세션 시작 순서나 운영 기준 문서가 바뀌었을 때

필수 확인 대상:

- `docs/README.md`
- 해당 카테고리 `README.md`
- `docs/operations/standard_workflow_draft.md`
- `docs/operations/ai_agent_quickstart.md`

## 6. 세션 상태 문서 반영 규칙

- 현재 세션의 주 작업 축이 바뀌면 `session_handoff.md` 를 갱신한다.
- 장기 후속 문서 우선순위가 바뀌면 `session_handoff.md` 의 `잔여 작업 우선순위` 를 갱신한다.
- 실제 작업 근거와 수행 내역은 날짜별 백로그에 남긴다.

## 7. 아직 작성되지 않은 세부 문서 처리 규칙

1. 허브 문서가 해당 문서를 참조하고 있는지 확인한다.
2. 참조 중이라면 최소 메타데이터와 책임 범위를 가진 초안 문서를 먼저 만든다.
3. 허브/인덱스에서 새 문서로 연결한다.
4. 세부 절차는 이후 세션에서 단계적으로 보강한다.

## 8. 다음에 읽을 문서

- [문서화 및 동기화 워크플로우](./workflow_documentation_sync.md)
- [표준 작업 워크플로우 초안](./standard_workflow_draft.md)
- [문서 위키 홈](../README.md)
