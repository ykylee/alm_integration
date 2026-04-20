# 운영 UI 코드 인덱스

- 문서 목적: `src/ui_prototype/` 의 현재 운영 UI 구조를 화면별 책임, 공통 스크립트, 백엔드 연동 관점에서 빠르게 탐색할 수 있도록 정리한다.
- 범위: `src/ui_prototype/` 하위 정적 HTML, `app.js`, 운영 데이터 관리 관련 화면
- 대상 독자: 프론트엔드 개발자, 백엔드 개발자, 기획자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `../overview/ui_ux_prototype.md`, `current_backend_implementation_status_summary.md`, `../operations/backend_operation_ui_render_and_scenarios.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- UI/UX 프로토타입 개요: [../overview/ui_ux_prototype.md](../overview/ui_ux_prototype.md)

## 1. 현재 운영 UI의 성격

현재 `src/ui_prototype/` 는 단순 디자인 샘플이 아니라, 일부 화면에서 실제 Rust 관리자 API 를 읽고 쓰는 운영 프로토타입이다. 이 브랜치 기준으로는 `admin.html`, `organization.html`, `data_organizations.html`, `data_workforce.html`, `data_settings.html` 이 문서 동기화 우선 대상이다.

## 2. 화면 인덱스

| 경로 | 역할 | 현재 코드 기준 상태 |
| --- | --- | --- |
| `src/ui_prototype/index.html` | 프로토타입 전체 진입점 | 화면 허브 |
| `src/ui_prototype/admin.html` | 관리자 콘솔 | `sync-runs`, 조직/인력/프로젝트/업무 항목 요약 조회 |
| `src/ui_prototype/organization.html` | 조직 운영 상세 | 조직 단위 운영 맥락과 도메인 조회 |
| `src/ui_prototype/data_organizations.html` | 조직 관리 작업면 | 조직 트리, 상세 인스펙터, 구성원 액션 폼, 이력 조회 |
| `src/ui_prototype/data_workforce.html` | 인력 관리 작업면 | 조직 필터 기반 인력 목록과 인력 상세/운영 액션 |
| `src/ui_prototype/data_settings.html` | 데이터 관리 연결 설정 | 관리자 API Base URL 관리와 연결 점검 |
| `src/ui_prototype/data_alm.html` | 내부 `ALM` 데이터 관리 | 프로젝트/업무 항목 조회 중심 |
| `src/ui_prototype/data_external_*.html` | 외부 시스템별 관리면 | 현재는 `sync-runs` 중심 요약 |

## 3. 공통 스크립트 구조

대부분의 동작은 `src/ui_prototype/app.js` 한 파일에 집중돼 있다. 현재 문서화 관점의 핵심 구간은 다음과 같다.

| 구간 | 역할 |
| --- | --- |
| 공통 설정/도우미 | API Base URL 처리, `fetchJson`, 상태 메시지 렌더 |
| 관리자 콘솔 렌더 | `admin/sync-runs`, `master-data`, `projects`, `work-items` 조회와 요약 카드 |
| 조직 운영 렌더 | 조직 선택 상태, 조직 기준 프로젝트/업무 항목 표시 |
| 조직 관리 워크벤치 | 조직 등록/수정/삭제, 구성원 등록/이동/비활성화, 트리/이력 렌더 |
| 인력 관리 워크벤치 | 조직 범위 필터, 인력 상세 인스펙터, 인력 수정 액션 |
| 데이터 설정 화면 | API 엔드포인트 연결 확인과 저장 |

## 4. 현재 코드 기준 핵심 UX 구조

### 4.1 관리자 콘솔

- 상단 `API Base URL` 입력과 연결 상태 표시
- `sync-runs` 실시간 목록
- 조직/인력/프로젝트/업무 항목 요약
- 운영 API 커버리지와 상태를 한 화면에서 파악하는 용도

### 4.2 조직 운영 상세

- 조직 관점의 운영 상태, 프로젝트, 업무 항목을 모아서 보는 화면
- 운영자 설명용 상세 페이지 성격이 강하다

### 4.3 조직 관리 작업면

이 브랜치의 `data_organizations.html` 은 단순 목록형 화면이 아니라 `Directory + Inspector + Action Rail` 구조를 따른다.

- 좌측: 조직 디렉터리와 선택 흐름
- 중앙: 선택 조직 상세, 구조 정보, 구성원, 이력
- 우측 또는 하단 액션 구역: 조직 등록/수정/삭제, 구성원 등록/이동/비활성화

특히 `organization-member-target-organization-code-input` 같은 명시적 입력 요소가 존재하므로, 자동화 시나리오나 운영 절차 문서도 이 구조를 기준으로 설명해야 한다.

### 4.4 인력 관리 작업면

- 조직 범위를 좁혀 인력을 탐색
- 선택 인력의 조직 소속과 운영 판단 정보를 인스펙터에서 확인
- 인력 등록/수정/비활성화 중심 액션을 같은 화면에서 수행

## 5. 백엔드 연동 기준

현재 운영 UI 는 기본적으로 `http://127.0.0.1:8080/api/v1` 를 기준 URL 로 사용한다. 실제 연동 문서와 맞춰야 할 주요 API 는 다음과 같다.

- `GET /api/v1/health`
- `GET/POST /api/v1/admin/sync-runs`
- `POST /api/v1/admin/sync-runs/{run_id}/retry`
- `POST /api/v1/admin/sync-runs/{run_id}/cancel`
- `GET/POST/PATCH/DELETE /api/v1/admin/master-data/organizations...`
- `GET /api/v1/admin/master-data/organizations/{organization_code}/history`
- `GET /api/v1/admin/master-data/organizations/{organization_code}/structure`
- `GET /api/v1/admin/master-data/organizations/{organization_code}/member-history`
- `GET/POST/PATCH/DELETE /api/v1/admin/master-data/workforce...`
- `GET /api/v1/admin/projects`
- `GET /api/v1/admin/work-items`

## 6. 문서 동기화 시 체크 포인트

- 새 화면이 추가되면 `docs/overview/ui_ux_prototype.md` 와 이 문서를 함께 갱신한다.
- 실제 API 쓰기 동작이 추가되면 `../operations/backend_operation_ui_render_and_scenarios.md` 에 재현 절차를 반영한다.
- 화면 구조를 단순화하거나 레이아웃을 재설계하면 자동화 셀렉터와 백로그 작업 상태도 함께 확인한다.

## 7. 현재 남아 있는 문서 공백

- `app.js` 를 모듈 단위로 분해한 상세 설계 문서는 아직 없다.
- 화면별 DOM 셀렉터 계약과 자동화 테스트 포인트를 정리한 문서는 아직 없다.
- 운영 UI 와 실제 사용자 역할 권한 매핑은 개념 수준 문서에 머물러 있다.

## 다음에 읽을 문서

- UI/UX 프로토타입 개요: [../overview/ui_ux_prototype.md](../overview/ui_ux_prototype.md)
- 운영 UI 시나리오: [../operations/backend_operation_ui_render_and_scenarios.md](../operations/backend_operation_ui_render_and_scenarios.md)
- 백엔드 코드 인덱스: [./backend_code_index.md](./backend_code_index.md)
