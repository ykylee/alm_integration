# 표준 작업 워크플로우 초안

- 문서 목적: 이 저장소의 표준 작업 흐름을 한눈에 파악하고, 세부 워크플로우 문서로 이동하기 위한 상위 허브를 제공한다.
- 범위: 표준 작업 흐름의 공통 원칙, 단계 요약, 세부 문서 분리 기준, 문서 간 진입 경로
- 대상 독자: 개발자, 운영자, 리뷰어, 문서 작성자
- 상태: draft
- 최종 수정일: 2026-04-16
- 관련 문서: `docs/operations/work_backlog.md`, `docs/operations/session_handoff.md`, `docs/operations/workflow_development_plan.md`, `docs/operations/development_environment.md`, `docs/operations/environments/README.md`, `AGENTS.md`

## 1. 문서 개요

현재 저장소는 코드 작업과 문서 작업이 함께 진행되고, 세션마다 다른 호스트 환경에서 작업할 수 있다. 따라서 작업 흐름은 단순 구현 순서가 아니라 `무엇을 할지 정리`, `어느 환경에서 했는지 기록`, `로컬/격리 검증`, `실행 증적 보관`, `세션 간 인계`까지 포함해야 한다.

이 문서는 현재까지 실제로 사용한 방식을 기준으로 표준 흐름의 뼈대를 정리한 상위 허브다. 세부 절차는 이 문서에 계속 누적하지 않고, 목적별 상세 문서로 분리하는 것을 전제로 한다.

## 2. 문서 역할 재정의

이 문서는 앞으로 아래 역할만 담당한다.

- 표준 작업 흐름의 공통 원칙 제시
- 전체 작업 단계를 짧게 요약
- 어떤 상황에서 어떤 세부 문서를 먼저 읽어야 하는지 안내
- 세부 문서가 분리될 때 기준 문서 역할 유지

이 문서가 직접 자세히 다루지 않을 내용은 아래와 같다.

- 세션 시작/종료 절차의 상세 체크리스트
- 작업 진행 절차의 상세 단계와 예외
- 문서 동기화 판단 기준
- 로컬/격리/실행 검증 상세 절차
- 코드 색인과 AI 전용 참조 구조

즉, 이 문서는 `절차를 모두 담는 본문` 이 아니라 `절차 문서들의 허브` 로 유지한다.

## 3. 기본 원칙

- 모든 작업은 시작 전에 목적과 범위를 짧게 브리핑한다.
- 작업은 날짜별 백로그 문서에 등록하고, `호스트명`, `호스트 IP` 를 함께 기록한다.
- 새 세션은 상세 백로그보다 먼저 `session_handoff.md` 를 통해 현재 기준 상태를 재구성한다.
- 환경 의존 정보는 일반 문서에 섞어 쓰지 않고 `docs/operations/environments/<hostname>-<ip>/` 아래에 분리 기록한다.
- 가능한 한 고정된 진입점 명령을 사용한다.
- 코드 수정 후에는 로컬 검증과 격리 검증을 구분해 수행한다.
- UI 또는 실행 결과가 중요하면 실제 프로세스를 띄우고 증적을 남긴다.
- 작업 결과가 기존 문서에 영향을 주면 관련 문서를 함께 갱신한다.

## 4. 표준 흐름 요약

표준 흐름은 아래 8단계로 본다.

1. 세션 인계 확인
2. 작업 시작 및 브리핑
3. 환경 확인
4. 구현 또는 문서 수정
5. 로컬 검증
6. 격리 검증
7. 실행 확인
8. 결과 기록 및 세션 종료 정리

각 단계의 상세 절차는 후속 세부 문서로 분리한다. 현재 문서에서는 각 단계가 왜 필요한지와 어떤 문서로 연결되는지만 요약한다.

### 4.1 세션 인계 확인

새 세션이나 새 환경에서 시작할 때는 아래 순서를 먼저 따른다.

1. `docs/operations/session_handoff.md` 를 먼저 읽는다.
2. `현재 작업 요약`, `진행 중 작업`, `차단 작업`, `잔여 작업 우선순위` 를 확인한다.
3. 필요한 경우 최신 날짜 백로그로 내려가 상세 근거를 확인한다.
4. 현재 호스트 환경 문서를 확인해 도구 설치 상태와 검증 이력을 확인한다.

목적:

- 과거 백로그 전체를 다시 읽지 않고도 현재 기준선을 빠르게 복원
- 작업 상태와 환경 상태를 분리해서 파악

상세 절차:

- [workflow_session_handoff.md](./workflow_session_handoff.md)

### 4.2 작업 시작 및 브리핑

1. `docs/operations/work_backlog.md` 와 최근 날짜 백로그를 확인한다.
2. `in_progress`, `blocked` 상태 작업이 있는지 확인하고 현재 작업과의 관계를 정리한다.
3. 이번 작업의 목적, 범위, 예상 산출물, 영향 가능 문서를 짧게 브리핑한다.
4. 오늘 날짜 백로그 문서가 없으면 새로 만들고, 작업 항목을 `planned` 또는 `in_progress` 로 등록한다.

필수 기록 항목:

- 작업명
- 상태
- 우선순위
- 요청일
- 완료일
- 담당
- 호스트명
- 호스트 IP
- 영향 문서
- 작업 내용
- 진행 현황
- 완료 기준
- 작업 결과
- 다음 세션 시작 포인트
- 남은 리스크
- 후속 작업

상세 절차:

- [workflow_task_execution.md](./workflow_task_execution.md)

### 4.3 환경 확인

1. 현재 호스트 식별 정보를 확인한다.
2. 필요한 런타임과 도구가 있는지 확인한다.
3. 환경에 따라 달라지는 설치 상태, 버전, 제약 사항은 호스트별 환경 문서에 기록한다.
4. 공통 절차는 `development_environment.md` 를 기준으로 따르고, 세션 특이사항만 호스트 문서에 추가한다.

대표 확인 항목:

- `hostname`
- `hostname -I`
- `python3 --version`
- `cargo --version`
- `pytest --version`
- `podman --version` 또는 `docker --version`

관련 문서:

- 환경 공통 절차: [development_environment.md](./development_environment.md)
- 호스트별 환경 기록: [environments/README.md](./environments/README.md)

### 4.4 구현 또는 문서 수정

1. 관련 코드와 문서를 먼저 확인한다.
2. 영향 범위가 있는 문서를 식별한다.
3. 코드 작업은 가능하면 테스트 기준을 먼저 확인하고 수정한다.
4. 문서 작업은 위키 인덱스와 상호 링크까지 함께 정리한다.

구현 시 기본 방향:

- 로컬에서만 통하는 일회성 절차보다 저장소 공용 진입점을 우선한다.
- 환경 호환 문제가 있으면 저장소 기본값으로 흡수할 수 있는지 먼저 검토한다.
- 테스트 실패가 환경 문제가 아니라 기대값 문제면, 실제 동작 기준으로 테스트를 바로잡는다.

상세 절차:

- [workflow_documentation_sync.md](./workflow_documentation_sync.md)

### 4.5 로컬 검증

수정 직후 가장 먼저 빠른 로컬 검증을 수행한다.

대표 경로:

- Python: `python3 -m pytest`
- Rust: `cargo test --manifest-path backend/Cargo.toml`
- 정적 검증: `git diff --check`

목적:

- 구현이나 문서 수정이 즉시 깨지지 않는지 빠르게 확인
- 격리 테스트 전에 실패 지점을 줄이기

### 4.6 격리 검증

로컬 검증 후에는 가능한 경우 컨테이너 기반 경로를 실행한다.

대표 경로:

- `CONTAINER_RUNTIME=podman make infra-up`
- `CONTAINER_RUNTIME=podman make container-test-python`
- `CONTAINER_RUNTIME=podman make container-test-rust`
- `CONTAINER_RUNTIME=podman make container-test`

현재 기준 유의사항:

- `podman` 사용 시 fully qualified image 사용
- SELinux 환경에서는 bind mount 에 `:Z` 필요
- Rust 컨테이너 버전은 저장소 의존성과 맞춰 유지

### 4.7 실행 확인

기능이 API, UI, 배치, 동기화처럼 실행 결과가 중요한 작업이면 실제로 기동해 확인한다.

대표 흐름:

1. 필요한 DB 또는 컨테이너 기동
2. 백엔드 또는 서버 기동
3. `curl` 또는 HTTP 체크로 1차 확인
4. 브라우저 또는 Playwright 로 렌더링/상태 확인
5. 필요 시 스크린샷, 로그, 응답 결과를 산출물로 저장

예시:

- 백엔드 헬스 체크: `curl http://127.0.0.1:8080/api/v1/health`
- 정적 UI 확인: `python3 -m http.server 8000`
- 브라우저 증적: `output/playwright/...`

상세 절차:

- [workflow_validation.md](./workflow_validation.md)

### 4.8 결과 기록 및 세션 종료 정리

1. 오늘자 백로그의 `진행 현황`, `작업 결과` 를 갱신한다.
2. 환경 변경이나 도구 설치가 있었다면 호스트별 환경 문서를 갱신한다.
3. 세션 인계에 영향이 큰 작업이면 `session_handoff.md` 를 갱신한다.
4. 공통 절차가 바뀌었다면 운영 문서와 README 를 함께 갱신한다.
5. 남은 리스크, 미검증 항목, 후속 작업을 명시한다.

목적:

- 다음 세션이 이전 세션의 맥락을 다시 복원하느라 시간을 쓰지 않게 하기

## 5. 세부 문서 분리 원칙

세부 문서는 아래 기준으로 분리한다.

- 하나의 문서는 하나의 책임만 가진다.
- 상태 문서와 절차 문서를 분리한다.
- 허브 문서는 짧고 안정적으로 유지한다.
- 예시와 체크리스트는 상세 문서에 둔다.
- 문서가 분리되면 이 허브 문서에서는 요약과 링크만 유지한다.

우선 분리 대상 문서:

- `workflow_task_execution.md`
- `workflow_documentation_sync.md`
- `workflow_session_handoff.md`
- `workflow_validation.md`

현재 작성 완료 문서:

- [작업 진행 워크플로우](./workflow_task_execution.md)
- [문서화 및 동기화 워크플로우](./workflow_documentation_sync.md)
- [세션 인계 워크플로우](./workflow_session_handoff.md)
- [검증 워크플로우](./workflow_validation.md)

향후 추가 대상 문서:

- `document_sync_policy.md`
- [code_index_strategy.md](./code_index_strategy.md)
- [ai_agent_quickstart.md](./ai_agent_quickstart.md)
- [branch_merge_document_policy.md](./branch_merge_document_policy.md)
- `workflow_automation_plan.md`

## 6. 현재 초안의 한계

- 허브 문서에도 단계 요약은 남아 있으므로, 세부 절차와 중복되지 않도록 요약 수준을 유지해야 한다.
- `document_sync_policy.md` 와 `workflow_automation_plan.md` 는 아직 별도 문서로 정리되지 않았다.
- PR 생성, 배포, 운영 장애 대응 흐름은 별도 표준 절차 문서로 더 세분화할 필요가 있다.
- 팀 역할별 승인 흐름과 리뷰 기준은 후속 문서로 보강해야 한다.

## 7. 예외 처리 원칙

- 네트워크가 불안정하면 설치 실패를 환경 문제로 기록하고, 다운로드 의존 작업과 저장소 수정 작업을 분리한다.
- 로컬은 되지만 컨테이너에서 안 되면 저장소 기본값 보정 가능성을 먼저 검토한다.
- 특정 호스트에서만 발생한 문제는 공통 문서보다 호스트별 환경 문서에 남긴다.
- 미확인 사실은 확정처럼 쓰지 않고 `미검증`, `가정`, `추정` 으로 표시한다.

## 8. 다음에 정리할 문서

- 작업 진행 워크플로우 상세판
- 문서화 및 동기화 워크플로우 상세판
- 세션 시작/종료 워크플로우 상세판
- 로컬/격리/실행 검증 워크플로우 상세판
- 코드 색인 전략
- AI agent 빠른 참조 문서
- 브랜치 병합 문서 정책
- 자동화 계획

## 다음에 읽을 문서

- 워크플로우 개발 계획: [workflow_development_plan.md](./workflow_development_plan.md)
- 세션 인계 문서: [session_handoff.md](./session_handoff.md)
- 작업 백로그 인덱스: [work_backlog.md](./work_backlog.md)
- 개발 환경 및 테스트 환경 가이드: [development_environment.md](./development_environment.md)
- 환경 기록 위키: [environments/README.md](./environments/README.md)
