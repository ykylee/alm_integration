# 시스템 통합 DB 백엔드 API 및 배치 계약 초안

- 문서 목적: 시스템 통합 DB 백엔드의 초기 릴리스 API, 내부 배치 실행 인터페이스, 운영 계약을 구현 관점에서 정리한다.
- 범위: 수집 수신 API, 동기화 운영 API, 정합성 운영 API, 기준정보 API, 배치 실행 계약, 공통 응답 규칙
- 대상 독자: 백엔드 개발자, 아키텍트, 운영자, 프론트엔드 개발자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_backend_component_draft.md`, `docs/architecture/integration_data_ingestion_sequence_draft.md`, `docs/architecture/reference_integrity_dashboard_ui_and_access_draft.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 목적

본 문서는 초기 릴리스에서 어떤 API 와 배치 인터페이스를 먼저 제공해야 하는지 정리한다. 범위를 과도하게 넓히지 않고, 통합 DB 구축과 운영 통제에 필요한 최소 계약을 먼저 고정하는 것이 목표다.

## 2. 계약 설계 원칙

- 외부 수신 API 와 관리자 운영 API 를 분리한다.
- `push` 수신 API 는 빠른 `ack` 를 우선하고, 도메인 반영 완료를 동기 응답으로 약속하지 않는다.
- 운영 API 는 조회와 실행을 분리한다.
- 실행 API 는 멱등 키를 받아 중복 실행을 제어할 수 있어야 한다.
- 배치 실행도 API 와 같은 명령 모델을 사용해 추적 가능해야 한다.
- 모든 실행성 인터페이스는 감사 로그 대상이다.

## 3. API 카테고리

### 3.1 외부 수신 API

- `ingestion_api`

### 3.2 관리자 운영 API

- `sync_operation_api`
- `integrity_operation_api`
- `master_data_api`
- `integration_admin_api`
- `policy_admin_api`

### 3.3 내부 배치 인터페이스

- `sync_job_command`
- `integrity_check_command`
- `retry_queue_command`
- `read_model_refresh_command`

## 4. 공통 요청/응답 원칙

### 4.1 공통 헤더

- `X-Request-Id`: 요청 추적 식별자
- `X-Idempotency-Key`: 중복 실행 방지 키
- `X-Source-System`: 외부 원천 시스템 코드
- `Authorization`: 인증 토큰 또는 서명 기반 인증

### 4.2 공통 응답 필드

- `request_id`: 내부 추적 식별자
- `accepted`: 동기 완료가 아닌 접수 여부
- `run_id`: 생성된 실행 단위 식별자
- `status`: `accepted`, `queued`, `running`, `completed`, `failed`, `ignored`
- `message`: 운영자 또는 호출자 확인용 요약 메시지
- `status_reason_code`: 상태 전이 또는 처리 결과의 대표 사유 코드
- `cancel_requested_at`: 취소 요청이 접수된 시각

### 4.3 공통 오류 코드 예시

- `INVALID_SCHEMA`
- `UNAUTHORIZED_SOURCE`
- `DUPLICATE_REQUEST`
- `STALE_EVENT`
- `REFERENCE_PENDING`
- `POLICY_VIOLATION`
- `RESOURCE_NOT_FOUND`
- `CONFLICTING_OPERATION`
- `RUN_ALREADY_FINISHED`
- `RUN_NOT_CANCELLABLE`
- `RUN_NOT_RETRIABLE`
- `RUN_ALREADY_CANCEL_REQUESTED`

## 5. 외부 수신 API 초안

### 5.1 `POST /api/v1/ingestion/events`

- 목적: 외부 시스템이 `push` 방식으로 변경 이벤트를 전달한다.
- 호출 주체: 외부 원천 시스템
- 처리 원칙:
  - 인증, 기본 스키마 검증, 멱등 키 판정까지만 동기 수행
  - 도메인 반영은 내부 파이프라인으로 위임

요청 예시:

```json
{
  "source_system": "jira",
  "source_object_type": "issue",
  "source_object_id": "ALM-123",
  "source_event_key": "jira-event-8891",
  "source_version": 42,
  "source_updated_at": "2026-04-07T08:15:00Z",
  "payload": {
    "summary": "Sync process update",
    "status": "In Progress"
  }
}
```

응답 예시:

```json
{
  "request_id": "req_01",
  "accepted": true,
  "run_id": "ing_20260407_001",
  "status": "accepted",
  "message": "payload accepted for asynchronous processing"
}
```

### 5.2 `POST /api/v1/ingestion/bulk-events`

- 목적: 외부 시스템이 배치 단위 이벤트 묶음을 전달한다.
- 호출 주체: 외부 원천 시스템, 내부 게이트웨이
- 비고:
  - 항목별 결과가 필요하므로 부분 실패 리포트를 반환할 수 있어야 한다.
  - 큰 payload 는 원시 적재 후 내부 파티셔닝 대상이 된다.

## 6. 동기화 운영 API 초안

### 6.1 `POST /api/v1/admin/sync-runs`

- 목적: 특정 원천 시스템 또는 범위에 대한 동기화를 수동 실행한다.
- 호출 주체: 시스템 관리자

요청 예시:

```json
{
  "source_system": "jira",
  "mode": "incremental",
  "scope": {
    "project_keys": ["ALM", "OPS"]
  },
  "reason": "manual consistency check"
}
```

응답 예시:

```json
{
  "request_id": "req_02",
  "accepted": true,
  "run_id": "sync_20260407_002",
  "status": "queued"
}
```

응답 규칙:

- `202 Accepted`: 새 실행이 `queued` 상태로 등록됨
- `200 OK`: 같은 멱등 키의 기존 실행을 재사용해 현재 실행 정보를 반환함
- `409 Conflict`: 같은 범위의 충돌 실행 정책 위반

### 6.2 `GET /api/v1/admin/sync-runs`

- 목적: 동기화 실행 목록을 조회한다.
- 호출 주체: 시스템 관리자
- 주요 필터:
  - `source_system`
  - `status`
  - `started_from`
  - `started_to`
  - `cancel_requested_from`
  - `cancel_requested_to`

응답 항목 예시:

- `run_id`
- `source_system`
- `run_status`
- `status_reason_code`
- `queued_at`
- `started_at`
- `ended_at`
- `cancel_requested_at`
- `processed_count`
- `success_count`
- `failure_count`
- `pending_count`

### 6.3 `GET /api/v1/admin/sync-runs/{run_id}`

- 목적: 특정 동기화 실행 상세를 조회한다.
- 포함 정보:
  - 실행 범위
  - 원시 적재 건수
  - 표준화 성공/실패 건수
  - 도메인 반영 성공/보류 건수
  - 오류 요약
  - 취소 요청 시각, 요청 주체, 취소 사유 코드

응답 예시:

```json
{
  "run_id": "sync_20260407_002",
  "source_system": "jira",
  "run_status": "partially_completed",
  "status_reason_code": "cancel_requested_during_domain_write",
  "queued_at": "2026-04-07T10:00:00Z",
  "started_at": "2026-04-07T10:00:02Z",
  "ended_at": "2026-04-07T10:04:11Z",
  "cancel_requested_at": "2026-04-07T10:03:54Z",
  "cancel_requested_by": "admin.user",
  "cancel_reason_code": "operator_manual_stop",
  "processed_count": 120,
  "success_count": 100,
  "failure_count": 5,
  "pending_count": 15
}
```

### 6.4 `POST /api/v1/admin/sync-runs/{run_id}/retry`

- 목적: 실패 또는 보류된 동기화 실행을 재실행한다.
- 호출 주체: 시스템 관리자
- 비고:
  - 원래 `run_id` 와의 연결이 남아야 한다.
  - 같은 멱등 키면 중복 재실행을 막는다.

요청 예시:

```json
{
  "reason": "retry after transient source timeout"
}
```

응답 예시:

```json
{
  "request_id": "req_04",
  "accepted": true,
  "run_id": "sync_20260407_003",
  "status": "queued",
  "status_reason_code": "retry_enqueued",
  "message": "retry run created"
}
```

응답 규칙:

- `202 Accepted`: 재시도용 새 실행이 생성됨
- `200 OK`: 같은 멱등 키의 기존 재시도 실행 정보를 반환함
- `409 Conflict`: 현재 상태가 재시도 불가인 경우
- `404 Not Found`: 대상 실행이 존재하지 않음

### 6.5 `POST /api/v1/admin/sync-runs/{run_id}/cancel`

- 목적: 진행 중이거나 대기 중인 동기화 실행에 취소 요청을 등록한다.
- 호출 주체: 시스템 관리자
- 처리 원칙:
  - 강제 종료가 아니라 협조적 취소를 요청한다.
  - 이미 커밋된 원시 적재와 도메인 반영은 자동 전역 롤백하지 않는다.
  - 실행이 이미 종료 상태면 상태 변경 없이 현재 상태를 반환할 수 있다.

요청 예시:

```json
{
  "reason": "operator requested stop due to upstream issue",
  "cancel_reason_code": "operator_manual_stop"
}
```

응답 예시:

```json
{
  "request_id": "req_03",
  "accepted": true,
  "run_id": "sync_20260407_002",
  "status": "running",
  "status_reason_code": "cancel_requested",
  "cancel_requested_at": "2026-04-07T10:03:54Z",
  "message": "cancellation request registered"
}
```

종료 상태 예외 응답 예시:

```json
{
  "request_id": "req_03",
  "accepted": false,
  "run_id": "sync_20260407_002",
  "status": "completed",
  "status_reason_code": "run_already_finished",
  "message": "run already finished; cancellation not applied"
}
```

응답 규칙:

- `202 Accepted`: 취소 요청이 정상 등록됨
- `200 OK`: 이미 취소 요청이 등록된 실행의 현재 상태를 반환함
- `409 Conflict`: 실행 상태상 취소 불가
- `404 Not Found`: 대상 실행이 존재하지 않음

오류 코드 기준:

- `RUN_ALREADY_FINISHED`: 이미 `completed`, `failed`, `cancelled`, `partially_completed` 상태인 실행
- `RUN_ALREADY_CANCEL_REQUESTED`: 이미 취소 요청이 접수된 실행
- `RUN_NOT_CANCELLABLE`: 정책상 취소 불가한 실행

## 7. 참조 정합성 운영 API 초안

### 7.1 `GET /api/v1/admin/reference-integrity/issues`

- 목적: 정합성 오류 목록을 조회한다.
- 호출 주체: 시스템 관리자, 제한 범위 운영 담당자
- 주요 필터:
  - `issue_type`
  - `severity`
  - `issue_status`
  - `source_entity_type`
  - `project_id`
  - `organization_id`

### 7.2 `GET /api/v1/admin/reference-integrity/issues/{issue_id}`

- 목적: 특정 오류 상세와 조치 이력을 조회한다.

### 7.3 `POST /api/v1/admin/reference-integrity/issues/{issue_id}/acknowledge`

- 목적: 오류를 확인 상태로 전환한다.
- 호출 주체: 시스템 관리자, 허용된 운영 역할

### 7.4 `POST /api/v1/admin/reference-integrity/issues/{issue_id}/retry`

- 목적: 관련 대상을 재처리 큐에 적재한다.
- 호출 주체: 시스템 관리자, 허용된 운영 역할

### 7.5 `POST /api/v1/admin/reference-integrity/issues/{issue_id}/resolve`

- 목적: 수동 검토 후 해결 처리한다.
- 호출 주체: 시스템 관리자, 운영 관리자

### 7.6 `POST /api/v1/admin/reference-integrity/issues/{issue_id}/ignore`

- 목적: 예외 승인 또는 무시 처리를 남긴다.
- 호출 주체: 시스템 관리자
- 비고:
  - 무시 사유와 만료 시각이 선택 또는 필수로 들어가야 한다.

## 8. 기준정보 API 초안

### 8.1 `GET /api/v1/admin/projects`

- 목적: 프로젝트 기준정보 목록 조회

### 8.2 `GET /api/v1/admin/work-items`

- 목적: 업무 항목 목록 조회
- 주요 필터:
  - `project_id`
  - `work_item_type`
  - `current_common_status`
  - `updated_from`

### 8.3 `GET /api/v1/admin/organizations`

- 목적: 조직 기준정보 목록 조회

### 8.4 `GET /api/v1/admin/workforces`

- 목적: 인력 기준정보 목록 조회

### 8.5 `POST /api/v1/admin/master-data/corrections`

- 목적: 제한된 수동 보정을 요청한다.
- 호출 주체: 시스템 관리자
- 비고:
  - 직접 수정이 아니라 “보정 요청” 모델로 남기고 감사 로그를 강제한다.

## 9. 외부 시스템 연결/인증정보 관리 API 초안

### 9.1 `GET /api/v1/admin/integration-systems`

- 목적: 외부 시스템 연결 정의와 상태를 조회한다.

### 9.2 `POST /api/v1/admin/integration-systems`

- 목적: 외부 시스템 정의를 신규 등록한다.
- 호출 주체: 시스템 관리자

### 9.3 `POST /api/v1/admin/integration-endpoints`

- 목적: 외부 시스템 접속 대상 URL, 엔드포인트, 인증 유형을 등록한다.
- 호출 주체: 시스템 관리자

요청 예시:

```json
{
  "integration_system_id": "sys_jira",
  "endpoint_name": "jira-primary",
  "endpoint_type": "api",
  "base_url": "https://jira.example.com",
  "resource_path": "/rest/api/2",
  "authentication_type": "basic"
}
```

### 9.4 `POST /api/v1/admin/integration-credentials`

- 목적: 외부 시스템 연결 자격증명을 등록한다.
- 호출 주체: 시스템 관리자
- 처리 원칙:
  - 비밀번호, `token`, `client secret` 는 수신 즉시 암호화 저장
  - 응답에는 민감정보 원문을 포함하지 않음

요청 예시:

```json
{
  "integration_endpoint_id": "ep_jira_primary",
  "credential_type": "basic",
  "principal_id": "admin_user",
  "secret": "plain-secret-input"
}
```

응답 예시:

```json
{
  "request_id": "req_cred_01",
  "accepted": true,
  "status": "completed",
  "message": "credential stored in encrypted form"
}
```

### 9.5 `PATCH /api/v1/admin/integration-credentials/{integration_credential_id}`

- 목적: 자격증명을 교체하거나 비활성화한다.
- 호출 주체: 시스템 관리자
- 비고:
  - 변경 이력과 변경 주체를 감사 로그로 남긴다.
  - 기존 secret 원문은 다시 조회할 수 없고 신규 입력으로만 교체한다.

### 9.6 `POST /api/v1/admin/integration-endpoints/{integration_endpoint_id}/validate`

- 목적: 현재 연결 정보와 자격증명으로 접속 검증을 수행한다.
- 호출 주체: 시스템 관리자
- 비고:
  - 검증 결과와 마지막 검증 시각을 기록한다.

## 10. 정책/프로세스 관리 API 초안

### 9.1 `GET /api/v1/admin/process-models`

- 목적: 프로세스 모델 정의 조회

### 9.2 `POST /api/v1/admin/process-models`

- 목적: 신규 프로세스 모델 정의 추가

### 9.3 `POST /api/v1/admin/workflow-schemes`

- 목적: 워크플로우 스킴 정의 추가 또는 변경

### 9.4 `POST /api/v1/admin/planning-schemes`

- 목적: 계획 스킴 정의 추가 또는 변경

### 9.5 `POST /api/v1/admin/role-policies`

- 목적: 역할 정책 정의 추가 또는 변경

## 11. 내부 배치 인터페이스 초안

### 10.1 `sync_job_command`

- 목적: 스케줄러가 동기화 잡을 생성한다.
- 필수 필드:
  - `job_code`
  - `source_system`
  - `mode`
  - `scope`
  - `requested_by`
  - `idempotency_key`

선택 필드:

- `retry_of_run_id`
- `priority`

### 10.2 `integrity_check_command`

- 목적: 참조 정합성 점검 배치를 실행한다.
- 필수 필드:
  - `check_scope`
  - `severity_threshold`
  - `requested_by`
  - `idempotency_key`

선택 필드:

- `priority`

### 10.3 `retry_queue_command`

- 목적: 보류 또는 실패 대상을 재처리 큐에 적재하거나 즉시 실행한다.
- 필수 필드:
  - `target_type`
  - `target_id`
  - `retry_reason`
  - `requested_by`
  - `execute_immediately`

선택 필드:

- `priority`
- `retry_of_run_id`

### 10.4 `read_model_refresh_command`

- 목적: 운영 대시보드와 집계 조회 모델을 갱신한다.
- 비고:
  - 초기 릴리스에서는 선택 구현이지만 계약은 미리 정의해두는 편이 좋다.

## 12. 권한/감사 기준

- 외부 수신 API 는 시스템 간 인증만 허용한다.
- 운영 API 는 역할 기반 권한과 데이터 범위 정책을 같이 적용한다.
- 실행 API 는 모두 `requested_by`, `reason`, `idempotency_key` 를 남긴다.
- 취소 API 는 `cancel_requested_at`, `cancel_requested_by`, `cancel_reason_code` 를 실행 이력에 남긴다.
- 재시도와 취소 API 는 종료 상태 예외를 오류가 아니라 운영 응답으로 명시적으로 반환할 수 있어야 한다.
- 수동 보정, 재처리, 무시 처리, 정책 변경은 감사 로그 필수 대상이다.
- 연결 정보 및 자격증명 등록/변경 API 는 시스템 관리자에게만 허용한다.
- 자격증명 관련 API 응답과 로그에는 `secret`, `token`, `password` 원문을 포함하지 않는다.

## 13. 권장 구현 순서

1. `POST /api/v1/ingestion/events`
2. `POST /api/v1/admin/sync-runs`
3. `GET /api/v1/admin/sync-runs`, `GET /api/v1/admin/sync-runs/{run_id}`
4. `POST /api/v1/admin/sync-runs/{run_id}/cancel`
5. `GET /api/v1/admin/reference-integrity/issues`
6. `POST /api/v1/admin/reference-integrity/issues/{issue_id}/retry`
7. `POST /api/v1/admin/integration-endpoints`
8. `POST /api/v1/admin/integration-credentials`
9. 기준정보 조회 API

즉, 초기에는 수집 진입점과 운영 모니터링/재처리 최소선부터 구현하는 편이 적절하다.

## 14. 후속 상세화 후보

- 인증 방식별 보안 계약
- API 별 필드 스키마와 검증 규칙
- 배치 인터페이스와 실제 큐/스케줄러 매핑
- 읽기 모델 조회 API 상세화
- 페이지네이션, 정렬, 대량 조회 제한 규칙
