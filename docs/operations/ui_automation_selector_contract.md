# 운영 UI 자동화 셀렉터 계약

- 문서 목적: 운영 UI 자동화 검증에서 깨지기 쉬운 핵심 DOM 기준점을 정리해, 레이아웃 변경 시 어떤 셀렉터를 우선 보존하거나 재검증해야 하는지 명시한다.
- 범위: `src/ui_prototype/` 운영 UI 화면 중 자동화 가치가 높은 요소, 현재 확인 가능한 명시 셀렉터, 재검증 필요 항목
- 대상 독자: 프론트엔드 개발자, QA 담당자, AI agent, 리뷰어
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `admin_ui_backend_contract.md`, `backend_operation_ui_render_and_scenarios.md`, `workflow_validation.md`, `branch_merge_document_policy.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 운영 위키: [./README.md](./README.md)
- 검증 워크플로우: [./workflow_validation.md](./workflow_validation.md)

## 1. 문서 개요

운영 UI 는 현재 정적 프로토타입과 실제 관리자 API 연결이 섞여 있어, 화면 구조가 바뀌면 자동화가 쉽게 깨질 수 있다. 이 문서는 그중 우선 보존하거나 재검증해야 하는 기준점을 짧게 고정한다.

중요 원칙:

- 명시 `id` 나 안정적인 텍스트 앵커가 있는 요소를 우선 기준점으로 삼는다.
- 레이아웃 클래스만으로 자동화를 설계하지 않는다.
- 화면 구조를 바꾸면 백로그 상태와 자동화 재검증 여부를 함께 갱신한다.

## 2. 핵심 화면별 기준점

### 2.1 `admin.html`

우선 유지 대상:

- `API Base URL` 입력 영역
- `admin-sync-runs-body`

자동화 확인 질문:

- 연결 기준 URL 을 바꾼 뒤 데이터 재조회가 되는가
- `sync-runs` 테이블이 성공/빈 상태/오류 상태를 구분해 보여주는가

### 2.2 `organization.html`

우선 유지 대상:

- 상단 `API Base URL` 입력 영역
- 조직/프로젝트/업무 항목 패널 제목 텍스트

자동화 확인 질문:

- 조직 기준 조회 결과가 화면 블록에 반영되는가
- 빈 상태 메시지가 깨지지 않는가

### 2.3 `data_organizations.html`

현재 코드 기준으로 중요도가 가장 높다.

우선 유지 대상:

- `organization-division-tabs`
- 조직 트리 스테이지
- `organization-member-target-organization-code-input`
- `data-org-division-tab`

자동화 확인 질문:

- 사업부 탭 전환 시 현재 탐색 대상이 바뀌는가
- 조직 선택 시 현재 조직이 속한 사업부 탭이 동기화되는가
- 구성원 이동 액션이 목표 조직 코드 입력과 함께 준비되는가

주의사항:

- 사업부 탭과 조직 선택 동기화는 레이아웃보다 상태 전이 검증이 중요하다.
- 상단 사업부 탭은 이제 컨테이너 `organization-division-tabs`, 공통 `data-testid="organization-division-tab"`, 개별 `data-org-division-tab="<organization_code>"` 기준으로 고정할 수 있다.

### 2.4 `data_workforce.html`

우선 유지 대상:

- 조직 범위 필터 입력 영역
- 인력 목록 영역
- 인력 상세 인스펙터 제목/핵심 필드

자동화 확인 질문:

- 조직 범위 변경 시 목록과 인스펙터가 함께 갱신되는가
- 선택 인력 전환 시 상세 정보가 일관되게 바뀌는가

## 3. 현재 명시적으로 확인한 셀렉터

| 유형 | 식별자 | 상태 |
| --- | --- | --- |
| 입력 | `organization-member-target-organization-code-input` | 코드에서 확인 완료 |
| 테이블 본문 | `admin-sync-runs-body` | 코드에서 확인 완료 |
| 탭 컨테이너 | `organization-division-tabs` | 코드에 추가 완료 |
| 탭 공통 식별자 | `data-testid="organization-division-tab"` | 코드에 추가 완료 |
| 탭 개별 식별자 | `data-org-division-tab="<organization_code>"` | 코드에 추가 완료 |

## 4. 재검증이 필요한 항목

- 사업부 탭 전환과 현재 조직 선택 동기화 상태
- 조직 화면 단순화 또는 패널 재배치 이후에도 자동화 기준점이 유지되는지 여부

## 5. 병합 및 리뷰 시 주의사항

- 리뷰 코멘트가 특정 날짜 백로그나 작업 ID 를 인용하더라도, 실제 저장소에 그 파일과 상태가 존재하는지 먼저 확인한다.
- 자동화 미검증 상태의 UI 구조 변경을 문서에서 완료처럼 표현하지 않는다.
- 병합 후에는 이 문서와 `session_handoff.md`, 최신 날짜 백로그를 함께 맞춘다.

## 6. 다음 단계 제안

1. Playwright 기준 `사업부 탭 전환 -> 조직 선택 -> 구성원 이동 폼` 흐름을 최소 시나리오로 고정한다.
2. 사업부 탭 전환 후 현재 조직 선택 동기화 상태를 자동화로 재검증한다.
3. 성공/실패 상태 메시지도 자동화 계약에 포함할지 판단한다.

## 다음에 읽을 문서

- 운영 UI-백엔드 계약 매핑: [./admin_ui_backend_contract.md](./admin_ui_backend_contract.md)
- 운영 UI 시나리오: [./backend_operation_ui_render_and_scenarios.md](./backend_operation_ui_render_and_scenarios.md)
- 브랜치 병합 문서 정책: [./branch_merge_document_policy.md](./branch_merge_document_policy.md)
