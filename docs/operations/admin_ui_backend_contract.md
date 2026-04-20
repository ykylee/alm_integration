# 운영 UI-백엔드 계약 매핑

- 문서 목적: 현재 운영 UI 작업면이 어떤 백엔드 API 와 연결되는지 화면 단위로 빠르게 확인할 수 있도록 계약 매핑을 정리한다.
- 범위: `src/ui_prototype/` 운영 UI 화면, `backend/src/http/routes/` 관리자 API, 화면별 읽기/쓰기/후속 검증 포인트
- 대상 독자: 프론트엔드 개발자, 백엔드 개발자, QA 담당자, 운영자, AI agent
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `backend_operation_ui_render_and_scenarios.md`, `../architecture/admin_ui_code_index.md`, `../architecture/backend_code_index.md`, `workflow_validation.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 운영 위키: [./README.md](./README.md)
- 운영 UI 시나리오: [./backend_operation_ui_render_and_scenarios.md](./backend_operation_ui_render_and_scenarios.md)

## 1. 문서 개요

이 문서는 “어떤 화면이 어떤 API 를 읽고 쓰는가”를 빠르게 확인하기 위한 계약 문서다. 설계 배경 설명보다 현재 브랜치의 실제 연결 표면과 후속 검증 포인트를 짧게 찾는 데 목적이 있다.

## 2. 화면별 계약 요약

| 화면 | 주요 목적 | 읽기 API | 쓰기 API | 비고 |
| --- | --- | --- | --- | --- |
| `admin.html` | 운영 관리자 콘솔 | `GET /health`, `GET /admin/sync-runs`, `GET /admin/master-data/organizations`, `GET /admin/master-data/workforce`, `GET /admin/projects`, `GET /admin/work-items` | 후속 확장 대상 | 현재는 조회/요약 중심 |
| `organization.html` | 조직 운영 상세 | `GET /admin/master-data/organizations`, `GET /admin/master-data/workforce`, `GET /admin/projects`, `GET /admin/work-items` | 없음 | 설명형 운영 상세 성격이 강함 |
| `data_organizations.html` | 조직 관리 작업면 | `GET /admin/master-data/organizations`, `GET /admin/master-data/organizations/{organization_code}/structure`, `GET /admin/master-data/organizations/{organization_code}/history`, `GET /admin/master-data/organizations/{organization_code}/members`, `GET /admin/master-data/organizations/{organization_code}/member-history`, `GET /admin/master-data/workforce` | `POST /admin/master-data/organizations`, `PATCH /admin/master-data/organizations/{organization_code}`, `DELETE /admin/master-data/organizations/{organization_code}`, `POST /admin/master-data/organizations/{organization_code}/members`, `PATCH /admin/master-data/workforce/{employee_number}`, `DELETE /admin/master-data/workforce/{employee_number}` | 조직/구성원 액션이 한 화면에 모여 있음 |
| `data_workforce.html` | 인력 관리 작업면 | `GET /admin/master-data/workforce`, `GET /admin/master-data/organizations` | `POST /admin/master-data/workforce`, `PATCH /admin/master-data/workforce/{employee_number}`, `DELETE /admin/master-data/workforce/{employee_number}` | 조직 범위 필터와 인력 상세 인스펙터 중심 |
| `data_settings.html` | 연결 설정/기준 URL 관리 | `GET /health` | 없음 | 연결 검증용 보조 화면 |

## 3. 화면별 상세 계약

### 3.1 `admin.html`

주요 사용자 질문:

- 지금 백엔드 연결이 살아 있는가
- 최근 동기화 실행은 어떤 상태인가
- 조직/인력/프로젝트/업무 항목 데이터가 얼마나 들어와 있는가

검증 포인트:

- `API Base URL` 변경 후 재조회가 정상 동작해야 한다.
- `sync-runs` 목록이 비어 있어도 오류가 아니라 빈 상태 메시지를 보여야 한다.
- DB 미연결 환경에서는 일부 읽기 API 가 `503` 을 반환할 수 있으므로, 화면 문구도 이 계약을 전제로 점검해야 한다.

### 3.2 `organization.html`

주요 사용자 질문:

- 선택 조직 기준으로 어떤 프로젝트/업무 항목이 연결되는가
- 조직/인력 기준정보가 현재 어떤 상태인가

검증 포인트:

- 조직 필터와 요약 카드가 함께 갱신돼야 한다.
- 프로젝트/업무 항목이 비어 있는 경우 빈 상태 문구가 유지돼야 한다.

### 3.3 `data_organizations.html`

주요 사용자 질문:

- 이 조직은 어디에 속하며 어떤 하위 구조를 가지는가
- 직속 구성원과 최근 이동 이력은 무엇인가
- 조직/구성원 배치를 바로 수정할 수 있는가

검증 포인트:

- 조직 선택 시 `structure`, `history`, `member-history` 조회가 함께 동기화돼야 한다.
- 구성원 이동 액션은 대상 조직 코드 입력과 선택 조직 컨텍스트를 모두 필요로 한다.
- 삭제/비활성화 액션 이후 목록과 인스펙터가 다시 갱신돼야 한다.

### 3.4 `data_workforce.html`

주요 사용자 질문:

- 특정 조직 범위 안에 누가 있는가
- 선택 인력을 어떻게 수정하거나 비활성화할 수 있는가

검증 포인트:

- 조직 범위 필터가 인력 목록과 인스펙터에 동시에 반영돼야 한다.
- 등록/수정/비활성화 후 목록 재조회와 상태 메시지 노출 기준이 필요하다.

## 4. 후속 자동화 우선순위

- `data_organizations.html` 의 조직 선택 -> 구조 조회 -> 구성원 이동 액션 흐름
- `data_workforce.html` 의 조직 필터 -> 인력 선택 -> 수정/비활성화 흐름
- `admin.html` 의 연결 확인 -> `sync-runs` 조회 -> 빈 상태/오류 상태 분기

## 5. 현재 남은 계약 공백

- 성공/실패 토스트 또는 상태 메시지 규격은 아직 문서로 고정되지 않았다.
- `sync-runs` 재시도/취소를 실제 UI 버튼과 연결한 표면은 아직 고정되지 않았다.
- 브라우저 자동화가 사용할 명시 셀렉터 계약은 별도 문서로 관리하는 편이 안전하다.

## 다음에 읽을 문서

- 운영 UI 시나리오: [./backend_operation_ui_render_and_scenarios.md](./backend_operation_ui_render_and_scenarios.md)
- 운영 UI 셀렉터 계약: [./ui_automation_selector_contract.md](./ui_automation_selector_contract.md)
- 운영 UI 코드 인덱스: [../architecture/admin_ui_code_index.md](../architecture/admin_ui_code_index.md)
