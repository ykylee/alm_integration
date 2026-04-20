# 운영 UI 렌더 점검 및 기능 동작 시나리오

- 문서 목적: 운영 UI 프로토타입의 실데이터 연결 상태를 점검하고, 현재 구현된 기능의 재현 가능한 동작 시나리오를 정리한다.
- 범위: `src/ui_prototype/admin.html`, `src/ui_prototype/organization.html`, `src/ui_prototype/data_organizations.html`, `src/ui_prototype/data_workforce.html`, Rust 관리자 API, 로컬 개발 환경 기준 확인 절차
- 대상 독자: 백엔드 개발자, 프론트엔드 개발자, 운영자, QA 담당자
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `docs/overview/ui_ux_prototype.md`, `docs/architecture/current_backend_implementation_status_summary.md`, `docs/architecture/admin_ui_code_index.md`, `README.md`, `docs/operations/backlog/2026-04-17.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 운영 위키: [./README.md](./README.md)
- UI/UX 프로토타입 설명: [../overview/ui_ux_prototype.md](../overview/ui_ux_prototype.md)

## 1. 점검 개요

2026-04-17 기준 운영 UI 프로토타입은 정적 설명 화면을 넘어 실제 관리자 API 를 조회하고, 일부 화면에서는 조직/인력 운영 액션까지 수행할 수 있는 상태다. 이번 점검의 목적은 다음 세 가지다.

- 로컬 환경에서 `admin`/`organization`/`data_organizations`/`data_workforce` 화면이 실제 API 를 읽는 경로가 살아 있는지 확인한다.
- 조직/인력 관리용 액션 폼이 어떤 관리자 API 와 맞물리는지 문서 기준선을 갱신한다.
- 현재까지 구현된 기능을 운영자가 어떤 순서로 확인하고 사용할 수 있는지 시나리오 형태로 정리한다.

## 2. 이번 점검에서 실제 확인한 항목

### 2.1 서버 가동 확인

- Rust 백엔드: `http://127.0.0.1:8080/api/v1/health` 에서 `{"status":"ok"}` 응답 확인
- 정적 UI 서버: `admin.html`, `organization.html`, `data_organizations.html`, `data_workforce.html` 모두 `200 OK` 응답 기준으로 점검
- 로컬 `PostgreSQL`: `alm-postgres` 컨테이너가 `healthy` 상태임을 확인

### 2.2 운영 API 응답 확인

점검 중 실제로 아래 데이터를 생성하거나 조회했다.

- 활성 조직 1건 확인
  - `default_org / Default Organization`
- 활성 인력 1건 생성 및 조회 확인
  - `E9001 / 박연계 / default_org`
- 수동 동기화 실행 1건 생성 및 조회 확인
  - `jira / incremental / queued / reason = ui render scenario`

### 2.3 렌더링 점검 결과

브라우저 기반 자동 렌더링 확인은 부분 확인 상태다.

- 확인 완료:
  - 운영 UI HTML 파일이 정적 서버에서 정상 서빙된다.
  - 화면이 호출하는 관리자 API 경로는 로컬 백엔드에서 정상 응답한다.
  - `app.js` 문법 검증은 `node --check src/ui_prototype/app.js` 로 통과했다.
  - `data_organizations.html` 과 `data_workforce.html` 이 공통 데이터 관리 내비게이션과 API Base URL 기반 연결 구조를 공유함을 코드 기준으로 확인했다.
- 환경 제약으로 미완료:
  - Playwright 브라우저 바이너리가 설치되지 않아 자동 스크린샷 캡처를 수행하지 못했다.
  - Safari 자동 DOM 추출은 AppleEvent timeout 으로 완료하지 못했다.

따라서 이번 문서의 렌더 점검은 “실데이터 연결 경로와 페이지 서빙은 확인, 브라우저 픽셀 단위 캡처는 미확인”으로 본다.

## 3. 로컬 재현 절차

### 3.1 서버 기동

```bash
docker compose up -d postgres
export $(grep -v '^#' .env.example | xargs)
cargo run --manifest-path backend/Cargo.toml
```

별도 터미널에서 정적 파일 서버를 올린다.

```bash
python3 -m http.server 8000
```

### 3.2 브라우저 진입

- 관리자 콘솔: `http://127.0.0.1:8000/src/ui_prototype/admin.html?apiBase=http://127.0.0.1:8080/api/v1`
- 조직 운영: `http://127.0.0.1:8000/src/ui_prototype/organization.html?apiBase=http://127.0.0.1:8080/api/v1`
- 조직 관리: `http://127.0.0.1:8000/src/ui_prototype/data_organizations.html?apiBase=http://127.0.0.1:8080/api/v1`
- 인력 관리: `http://127.0.0.1:8000/src/ui_prototype/data_workforce.html?apiBase=http://127.0.0.1:8080/api/v1`

### 3.3 최소 데이터 준비

조직 기본 데이터만 있는 상태라면 아래 예시로 인력과 동기화 실행을 추가할 수 있다.

```bash
curl -X POST http://127.0.0.1:8080/api/v1/admin/master-data/workforce \
  -H 'content-type: application/json' \
  -d '{
    "employee_number":"E9001",
    "display_name":"박연계",
    "employment_status":"active",
    "primary_organization_code":"default_org",
    "job_family":"platform_engineering",
    "email":"e9001@example.com"
  }'

curl -X POST http://127.0.0.1:8080/api/v1/admin/sync-runs \
  -H 'content-type: application/json' \
  -d '{
    "source_system":"jira",
    "mode":"incremental",
    "scope":{"project_keys":["OPS"]},
    "reason":"ui render scenario"
  }'
```

## 4. 기능 동작 시나리오

### 4.1 시나리오 A: 운영 UI 기본 연결 확인

목적:

- 운영자가 화면 상단의 `API Base URL` 을 기준으로 실제 백엔드 연결 여부를 확인한다.

사전 조건:

- Rust 백엔드와 정적 서버가 모두 실행 중이어야 한다.

단계:

1. `admin.html` 또는 `organization.html` 을 연다.
2. 상단 `API Base URL` 이 `http://127.0.0.1:8080/api/v1` 로 맞는지 확인한다.
3. `새로고침` 버튼을 누른다.
4. 상태 칩이 성공이면 연결 완료, 실패면 연결 실패 메시지를 확인한다.

기대 결과:

- 성공 시 상단 상태 칩이 연결 정상 상태로 바뀌고, 라이브 메트릭 블록이 실제 숫자로 채워진다.
- 실패 시 상단 상태 칩이 연결 실패 상태로 바뀌고, 표/요약 영역은 빈 상태 메시지를 보여준다.

### 4.2 시나리오 B: 관리자 콘솔에서 `sync-runs` 확인

목적:

- 수동 동기화 실행 요청이 운영 UI 에 반영되는지 확인한다.

사전 조건:

- `POST /api/v1/admin/sync-runs` 로 최소 1건의 실행을 생성한다.

단계:

1. `admin.html` 을 연다.
2. `동기화 실행 현황` 패널을 확인한다.
3. 필요 시 `새로고침` 을 눌러 최신 상태를 반영한다.

기대 결과:

- 최근 실행 목록에 `source_system`, 처리 요약, 상태가 행 단위로 표시된다.
- 이번 점검 기준 예시는 `jira / incremental / queued` 1건이다.

### 4.3 시나리오 C: 조직/인력 마스터 운영 확인

목적:

- 조직 마스터와 인력 마스터가 운영 화면에서 함께 보이는지 확인한다.

사전 조건:

- 활성 조직 1건 이상 존재해야 한다.
- 활성 인력 1건 이상 존재해야 한다.

단계:

1. `organization.html` 을 연다.
2. 필요하면 `조직 코드 필터` 에 `default_org` 를 입력한다.
3. `조직 마스터` 와 `인력 기준정보` 패널을 확인한다.

기대 결과:

- 조직 마스터 표에 `default_org / Default Organization / active` 가 보인다.
- 인력 기준정보 패널에 `박연계 (E9001)` 와 소속 조직 정보가 요약 카드로 표시된다.

### 4.4 시나리오 D: 조직 기준 프로젝트/업무 항목 조회

목적:

- 조직 기준 도메인 조회 API 가 UI 에 연결되는지 확인한다.

사전 조건:

- `project`, `work_item` 데이터가 아직 적재되지 않았을 수 있다.

단계:

1. `organization.html` 에서 `조직 소관 프로젝트`, `조직 소관 업무 항목` 패널을 확인한다.
2. 데이터가 없으면 빈 상태 메시지를 확인한다.
3. 이후 `pull` 또는 `push` 로 프로젝트/업무 항목이 적재되면 같은 패널에서 실제 목록을 확인한다.

기대 결과:

- 현재 데이터가 없으면 “조건에 맞는 프로젝트가 없습니다”, “조건에 맞는 업무 항목이 없습니다” 메시지가 표시된다.
- 이후 데이터 적재 후에는 `project_code`, `project_name`, `work_item_key`, 담당자, 상태가 행 단위로 채워진다.

### 4.5 시나리오 E: 관리자 콘솔에서 전체 운영 표면 요약

목적:

- 운영자가 한 화면에서 `sync-runs`, 조직/인력 마스터, 프로젝트/업무 항목 API 커버리지를 빠르게 훑는다.

단계:

1. `admin.html` 을 연다.
2. 상단 라이브 메트릭 블록과 `API 커버리지` 패널을 확인한다.
3. 조직/인력/프로젝트/업무 항목 패널을 아래로 순서대로 확인한다.

기대 결과:

- 상단 메트릭에 실행 건수, 활성 조직 수, 활성 인력 수가 표시된다.
- `API 커버리지` 패널에 현재 운영 화면이 읽고 있는 API 범위가 요약된다.

### 4.6 시나리오 F: 조직 관리 작업면에서 구조와 이력 확인

목적:

- 운영자가 `data_organizations.html` 에서 선택 조직 기준 구조 정보와 이력을 한 작업면에서 확인한다.

사전 조건:

- 조직 1건 이상이 존재해야 한다.

단계:

1. `data_organizations.html` 을 연다.
2. 좌측 디렉터리 또는 목록에서 대상 조직을 선택한다.
3. 중앙 인스펙터에서 상위 경로, 직속 하위 조직, 직속 구성원을 확인한다.
4. 변경 이력 또는 구성원 이동 이력 영역을 확인한다.

기대 결과:

- 선택 조직 기준 구조 정보가 `structure` 조회 결과와 일치해야 한다.
- 조직 변경 이력과 구성원 이동 이력이 각각 별도 패널에 표시되어야 한다.
- 데이터가 없으면 빈 상태 메시지가 보여야 한다.

### 4.7 시나리오 G: 조직 관리 작업면에서 구성원 이동 액션 준비 확인

목적:

- 조직 관리 작업면이 단순 조회가 아니라 구성원 배치 변경 액션을 수행할 준비가 되어 있는지 확인한다.

사전 조건:

- 이동 대상 조직 코드와 대상 구성원이 존재해야 한다.

단계:

1. `data_organizations.html` 을 연다.
2. 조직 구성원 액션 영역에서 대상 구성원을 선택한다.
3. `target organization code` 입력에 이동 대상 조직 코드를 넣는다.
4. 이동 또는 비활성화 액션 버튼 흐름을 확인한다.

기대 결과:

- 액션 폼이 선택 조직 컨텍스트와 함께 표시된다.
- 입력 요소는 명시적 셀렉터 기반으로 노출되며, 자동화 검증 시 기준점으로 삼을 수 있다.
- 실제 API 호출 전후 상태 메시지 설계가 필요한 영역이 드러나야 한다.

### 4.8 시나리오 H: 인력 관리 작업면에서 조직 범위 기반 탐색

목적:

- 인력 관리 작업면에서 조직 범위 필터와 선택 인력 상세가 함께 동작하는지 확인한다.

사전 조건:

- 인력 1건 이상이 존재해야 한다.

단계:

1. `data_workforce.html` 을 연다.
2. 조직 코드 또는 검색 조건을 입력한다.
3. 목록에서 인력을 선택한다.
4. 인스펙터에서 소속 조직, 기본 인적 정보, 후속 액션 영역을 확인한다.

기대 결과:

- 조직 범위가 바뀌면 목록과 선택 인력 상세가 함께 갱신된다.
- 인력 상세에는 운영자가 이동/비활성화 판단에 필요한 맥락 정보가 표시된다.

## 5. 현재 점검 기준 데이터 상태

이번 점검 종료 시점의 로컬 데이터 상태는 다음과 같다.

- 조직:
  - `default_org / Default Organization`
- 인력:
  - `E9001 / 박연계 / default_org`
- 동기화 실행:
  - `jira / incremental / queued / ui render scenario`
- 프로젝트:
  - 없음
- 업무 항목:
  - 없음

## 6. 다음 확인 과제

- 브라우저 바이너리가 있는 환경에서 Playwright 또는 실제 브라우저 스크린샷으로 `data_organizations`/`data_workforce` 까지 픽셀 렌더링 재확인
- `project/work_item` 실데이터 적재 후 `organization`/`admin` 화면 목록 렌더링 재검증
- 조직/인력 액션 성공·실패 시 사용자 피드백과 자동화 셀렉터를 문서와 함께 고정
