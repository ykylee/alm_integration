# 워크플로우 agent 토폴로지

- 문서 목적: 표준 워크플로우를 실행할 때 사용할 agent 역할, 입력/출력 문서, skill/MCP 연결 구조, 권한 경계를 정의한다.
- 범위: 추천 agent 목록, 역할 분담, 상태 문서 수정 원칙, 공통 agent 와 프로젝트 특화 agent 분리 기준
- 대상 독자: AI agent 설계자, 개발자, 운영자, 문서 작성자
- 상태: draft
- 최종 수정일: 2026-04-18
- 관련 문서: `workflow_standard_composition_proposal.md`, `ai_agent_quickstart.md`, `workflow_automation_plan.md`, `branch_merge_document_policy.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 운영 위키: [./README.md](./README.md)
- 표준 워크플로우 구성안: [./workflow_standard_composition_proposal.md](./workflow_standard_composition_proposal.md)

## 1. 설계 목표

agent 는 사람을 대체하는 단일 자동화가 아니라, 워크플로우의 각 단계에서 판단과 실행을 분담하는 역할 실행자로 정의한다.

핵심 목표:

- 세션 시작 기준선을 빠르게 복원한다.
- 상태 문서와 기준 문서를 혼동하지 않는다.
- 반복 점검과 초안 생성은 tool 로 내리고, 판단 순서는 agent 와 skill 로 유지한다.
- 여러 프로젝트에 공통 적용할 수 있는 agent 와 저장소 특화 agent 를 구분한다.

## 2. 권장 agent 세트

### 2.1 1차 필수 agent

| agent | 핵심 역할 | 주요 입력 | 주요 출력 |
| --- | --- | --- | --- |
| `session-orchestrator` | 세션 시작 기준선 복원 | `session_handoff.md`, `work_backlog.md`, 최신 백로그, 프로젝트 프로파일 | 현재 작업 요약, 다음 행동 제안 |
| `backlog-steward` | 작업 등록과 상태 기록 보조 | 오늘 날짜 백로그, 작업 브리핑, 영향 문서 후보 | 백로그 신규 항목 초안, 상태 갱신 초안 |
| `doc-sync-guardian` | 문서 동기화와 stale 설명 탐지 | 변경 파일, 허브 문서, 관련 기준 문서 | 영향 문서 후보, stale 문서 경고, 허브 갱신 포인트 |

### 2.2 2차 확장 agent

| agent | 핵심 역할 | 주요 입력 | 주요 출력 |
| --- | --- | --- | --- |
| `validation-coordinator` | 변경 유형별 검증 수준 결정 | 변경 요약, 테스트 명령, 환경 문서 | 검증 계획, 미실행 사유, 결과 요약 |
| `merge-reconciler` | 병합 후 상태/인덱스 정리 | 병합 결과, handoff, 백로그, 허브 문서 | 병합 후 재확정 문안 |
| `workflow-governor` | 공통 규칙과 프로젝트 특화 규칙 경계 관리 | 표준 문서, 프로젝트 프로파일, skill/MCP 목록 | 공통화 대상, 특화 규칙, 체계 변경 제안 |

## 3. agent 별 상세 역할

### 3.1 `session-orchestrator`

책임:

- 세션 시작 시 읽기 순서를 강제한다.
- 진행 중/차단/잔여 우선순위를 짧게 복원한다.
- 필요한 다음 문서로 라우팅한다.

잘 맞는 이유:

- 새 세션이 항상 같은 순서로 시작되도록 만들 수 있다.
- quickstart 와 handoff 문서를 실제 동작 흐름으로 연결한다.

사용할 skill:

- `session-start`

사용할 MCP 후보:

- 최신 백로그 탐색
- 프로젝트 프로파일 탐색

### 3.2 `backlog-steward`

책임:

- 오늘 날짜 백로그 생성 여부를 확인한다.
- 신규 작업 항목과 필수 필드를 채우도록 돕는다.
- 영향 문서와 다음 세션 시작 포인트를 정리한다.

주의사항:

- 검증 근거 없이 `done` 으로 바꾸지 않는다.
- 사용자가 말한 상태보다 실제 저장소와 실행 결과를 우선 확인한다.

사용할 skill:

- `backlog-update`

사용할 MCP 후보:

- 백로그 템플릿 생성
- 영향 문서 후보 출력

### 3.3 `doc-sync-guardian`

책임:

- 코드/문서 변경 이후 같이 갱신해야 할 문서를 찾는다.
- 허브, 인덱스, quickstart, handoff 누락을 감지한다.
- 오래된 설명 문서를 stale 후보로 표시한다.

사용할 skill:

- `doc-sync`

사용할 MCP 후보:

- 링크 검사
- 메타데이터 검사
- 영향 문서 후보 출력

### 3.4 `validation-coordinator`

책임:

- 이번 변경이 로컬 검증만 필요한지, 격리 검증과 실행 확인까지 필요한지 판단한다.
- 검증하지 못한 항목을 명시하게 한다.
- 검증 결과를 백로그와 handoff 용 요약으로 바꾼다.

사용할 skill:

- `validation-plan`

사용할 MCP 후보:

- 테스트 명령 추천
- 실행 증적 수집 위치 추천

### 3.5 `merge-reconciler`

책임:

- 병합 후 기준 사실 문서와 상태 문서를 다시 정리한다.
- 허브와 색인 경로가 최종 상태를 가리키는지 확인한다.

사용할 skill:

- `merge-doc-reconcile`

사용할 MCP 후보:

- 링크 검사
- changed files 기반 stale index 후보 추출

### 3.6 `workflow-governor`

책임:

- 어떤 항목을 문서, skill, MCP, agent 로 두어야 하는지 판단한다.
- 프로젝트별 예외가 공통 규칙을 훼손하는지 검토한다.
- 공통 코어와 프로젝트 프로파일 경계를 유지한다.

## 4. 상태 문서 수정 권한 원칙

가장 중요한 원칙은 상태 문서를 다루는 agent 의 권한을 제한하는 것이다.

| 문서 유형 | 수정 성격 | 원칙 |
| --- | --- | --- |
| 공통 표준 문서 | 제안/리뷰 중심 | 비교적 적극적 제안 가능 |
| 프로젝트 프로파일 | 저장소 기준 사실 반영 | 실제 저장소 구조 확인 후 수정 |
| `session_handoff.md` | 현재 상태 요약 | 보수적 수정 |
| 날짜별 백로그 | 작업 사실 기록 | 근거 확인 후 수정 |

금지해야 할 행동:

- 존재하지 않는 백로그 파일을 사실처럼 쓰기
- 외부 리뷰 코멘트를 저장소 사실보다 우선하기
- 검증하지 않은 작업을 완료 처리하기
- 병합 전 상태 문서를 단순 기계적 병합하기

## 5. skill / MCP / agent 연결 원칙

좋은 연결 구조는 아래와 같다.

- 문서: 정책과 원칙
- skill: 절차와 판단 순서
- MCP: 검사와 초안 생성
- agent: 역할 단위 조합과 실행 책임

예:

- `backlog-steward`
  - 문서: `workflow_task_execution.md`
  - skill: `backlog-update`
  - MCP: 백로그 템플릿 생성, 영향 문서 후보 출력

## 6. 공통 agent 와 프로젝트 특화 agent 구분

### 6.1 공통 agent 로 적합한 경우

- 세션 시작
- 백로그 등록
- 문서 동기화
- 병합 후 재정리
- 검증 수준 선택

### 6.2 프로젝트 특화 agent 가 필요한 경우

- 특정 도메인 운영 UI 검증
- 특정 파이프라인 배포 검증
- 특정 데이터 정합성 운영 절차

기본 원칙:

- 가능한 한 공통 agent 는 얇게 유지한다.
- 프로젝트 특화 동작은 프로젝트 프로파일 또는 별도 skill 로 밀어 넣는다.

## 7. 도입 순서 제안

1. `session-orchestrator`
2. `backlog-steward`
3. `doc-sync-guardian`
4. `validation-coordinator`
5. `merge-reconciler`
6. `workflow-governor`

처음부터 모두 만들기보다, 세션 시작과 문서/백로그 관리처럼 효과가 큰 영역부터 도입하는 편이 안정적이다.

## 8. 현재 한계

- agent 정의는 문서 수준이며, 아직 실제 skill/MCP 계약으로 내려가지는 않았다.
- 프로젝트 프로파일 템플릿이 없으면 공통 agent 의 재사용성이 떨어질 수 있다.
- 검증 agent 는 실제 테스트/브라우저 환경 제약을 프로젝트별로 흡수해야 한다.

## 다음에 읽을 문서

- 표준 워크플로우 구성안: [./workflow_standard_composition_proposal.md](./workflow_standard_composition_proposal.md)
- AI agent 빠른 참조 문서: [./ai_agent_quickstart.md](./ai_agent_quickstart.md)
- 워크플로우 자동화 계획: [./workflow_automation_plan.md](./workflow_automation_plan.md)
- 브랜치 병합 문서 정책: [./branch_merge_document_policy.md](./branch_merge_document_policy.md)
