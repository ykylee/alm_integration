# 시스템 통합 DB 백엔드 초기 구현 순서 및 개발 착수 체크리스트

- 문서 목적: 현재까지 작성된 백엔드 설계 문서를 기준으로 초기 구현 순서, 선행조건, 개발 착수 체크리스트를 정리한다.
- 범위: 문서 리뷰 요약, 초기 구현 단계, 단계별 산출물, 개발 착수 체크리스트, 남은 보완 과제
- 대상 독자: 백엔드 개발자, 아키텍트, `DBA`, 운영자, 기술 리드
- 상태: draft
- 최종 수정일: 2026-04-08
- 관련 문서: `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_backend_component_draft.md`, `docs/architecture/integration_data_ingestion_sequence_draft.md`, `docs/architecture/integration_backend_api_and_batch_contract_draft.md`, `docs/architecture/integration_db_migration_and_seed_strategy_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 문서 리뷰 요약

기술 스택 가정:

- 프론트엔드 1차 검토안: `React`
- 백엔드 1차 검토안: `Rust`
- 현재 저장소의 Python 스캐폴딩은 구조 검토용 임시 골격으로 본다.

### 1.1 현재 구현 착수에 충분히 정리된 항목

- 핵심 도메인 경계와 엔터티 구조
- 초기 릴리스 `ERD`, 물리 모델, `DDL`
- 백엔드 런타임 컴포넌트 분해
- `pull`/`push` 공통 수집 파이프라인과 충돌 방지 규칙
- 초기 운영 API 와 배치 명령 계약
- `DB` 마이그레이션, 시드 데이터, 환경 승격 원칙
- 참조 정합성 운영 보완 구조

위 항목은 초기 백엔드 개발을 시작하는 데 필요한 최소 설계선으로 볼 수 있다.

### 1.2 아직 남아 있는 보완 포인트

- 외부 수신 `ingestion_api` 의 인증/서명 정책 상세화
- 읽기 모델/집계 모델 생성 전략 구체화
- 외부 시스템 자격증명 암호화 키 관리와 회전 정책 상세화
- 운영 배포 체크리스트와 기능 플래그 전환 절차 상세화

다만 위 항목들은 초기 구현 착수를 막는 수준은 아니고, 1차 구현과 병행 보완 가능한 범위다.

### 1.3 구현 관점 최종 판단

현재 문서 상태는 “개발 시작 가능” 수준이다. 따라서 다음 단계는 추가 개념 문서를 늘리는 것보다, 구현 순서를 고정하고 각 단계의 완료 조건을 체크리스트로 관리하는 편이 적절하다.

## 2. 초기 구현 순서

### 2.1 Phase BE-1 저장 기초선 구축

목표:

- 마이그레이션 실행 체계 마련
- 핵심 테이블과 필수 시드 반영
- 개발 환경 부트스트랩 가능 상태 확보

구현 범위:

- `organization_master`
- `workforce_master`
- `project`
- `work_item_type`
- `work_item`
- `work_item_hierarchy`
- `process_model_definition`
- `workflow_scheme`
- `workflow_status_definition`
- `planning_scheme`
- `view_scheme`
- `project_process_model`

완료 기준:

- 빈 데이터베이스에서 스키마와 필수 시드까지 자동 반영 가능
- 프로젝트/업무 항목 기준 조회가 실패 없이 동작 가능

### 2.2 Phase BE-2 원시 적재와 실행 이력 구축

목표:

- 외부 시스템 수집 데이터를 손실 없이 적재
- 동기화 실행 단위 추적 가능 상태 확보

구현 범위:

- 원시 적재 저장 구조
- 동기화 실행 이력 구조
- `sync_job_scheduler`
- `sync_run_orchestrator`
- `raw_ingestion_writer`
- `ingestion_error_recorder`

완료 기준:

- `pull` 과 `push` 모두 원시 적재까지 성공
- 멱등 수집 키 기준 중복 적재 판정 가능
- 실행 이력과 실패 원인 추적 가능

### 2.3 Phase BE-3 표준화와 도메인 반영 구축

목표:

- 원시 데이터에서 내부 기준 엔터티까지 단일 경로로 반영
- 충돌 방지와 최신성 판정 적용

구현 범위:

- `normalization_pipeline`
- `identity_mapping_service`
- `reference_resolution_service`
- `project_write_service`
- `work_item_write_service`
- 조직/인력 기준 반영 최소선

완료 기준:

- 표준화 결과가 `project`, `work_item` 반영까지 이어져야 한다.
- `source_version`, `source_event_key`, `pending_reference` 기준이 실제 코드에 반영돼야 한다.

### 2.4 Phase BE-4 운영 통제 최소선 구축

목표:

- 정합성 오류와 재처리 운영이 가능해야 한다.

구현 범위:

- `reference_integrity_service`
- `integrity_issue_service`
- `retry_queue_service`
- `audit_event_service`

완료 기준:

- 참조 누락/버전 충돌/중복 반영 시도가 운영 이슈로 남아야 한다.
- 보류 대상 재처리가 가능해야 한다.

### 2.5 Phase BE-5 운영 API 최소선 구축

목표:

- 관리자와 운영자가 동기화와 정합성 상태를 실제로 조회/제어 가능해야 한다.

구현 범위:

- `POST /api/v1/ingestion/events`
- `POST /api/v1/admin/sync-runs`
- `GET /api/v1/admin/sync-runs`
- `GET /api/v1/admin/sync-runs/{run_id}`
- `POST /api/v1/admin/sync-runs/{run_id}/cancel`
- `GET /api/v1/admin/reference-integrity/issues`
- `POST /api/v1/admin/reference-integrity/issues/{issue_id}/retry`
- 기준정보 조회 API 최소선

완료 기준:

- 수동 동기화 실행, 취소 요청 등록, 실행 이력 조회, 오류 목록 조회, 재처리 요청이 가능해야 한다.

## 3. 권장 개발 단위

구현 작업 방식 원칙:

- 백엔드 구현은 `TDD` 를 기본 원칙으로 진행한다.
- 새 기능 또는 변경 작업은 단위 테스트를 먼저 작성하고, 실패를 확인한 뒤 구현을 시작한다.
- 구현 완료 후 테스트 통과와 최소 리팩터링까지 한 작업 단위로 본다.

### 3.1 첫 스프린트 권장 범위

- 마이그레이션 실행 기반
- 핵심 테이블 생성
- 필수 시드 반영
- 원시 적재 테이블
- `ingestion_api` 단건 수신
- `sync_run` 생성/조회 최소선
- 애플리케이션 부트스트랩과 헬스체크

### 3.2 두 번째 스프린트 권장 범위

- 표준화 파이프라인
- 식별자 매핑
- `project`/`work_item` 반영
- `pull` 실행 경로

### 3.3 세 번째 스프린트 권장 범위

- 정합성 오류 적재
- 재처리 큐
- 운영 API 최소선

## 4. 개발 착수 체크리스트

### 4.1 공통 준비

- [ ] 프론트엔드 `React`, 백엔드 `Rust` 채택 여부를 구현 착수 전에 최종 확정했다.
- [ ] 선택할 `RDBMS` 와 마이그레이션 도구를 확정했다.
- [ ] 개발 환경에서 빈 데이터베이스 생성과 초기화 절차를 합의했다.
- [ ] 마이그레이션 파일 디렉터리 구조(`schema/seed/backfill`)를 준비했다.
- [ ] 필수 시드 목록과 소유자를 정했다.

### 4.1A 스택 재매핑 준비

- [x] 백엔드 웹 프레임워크 1차 기준을 `axum` 으로 좁혔다.
- [x] `Rust` 기준 DB 접근 계층 1차 기준을 `sqlx` 로 좁혔다.
- [x] `Rust` 기준 마이그레이션 도구 1차 기준을 `sqlx migrate` 로 좁혔다.
- [ ] 현재 임시 Python 스캐폴딩에서 유지할 계약과 폐기할 구현 세부를 구분했다.
- [x] Rust 프로젝트 위치를 `backend/` 하위로 정했다.
- [ ] `sqlx` offline mode 사용 여부를 정했다.

### 4.2 저장 모델 준비

- [x] `sqlx migrate` 초기 구조와 첫 마이그레이션 파일을 생성했다.
- [ ] `initial_release_ddl_draft.md` 기준으로 1차 테이블 구현 범위를 확정했다.
- [ ] `work_item_type`, `process_model_definition`, `workflow_scheme` 필수 시드를 확정했다.
- [ ] `schema_migration_history` 추적 방식을 정했다.
- [ ] 운영 환경 파괴적 변경 금지 원칙을 팀에 공유했다.

### 4.3 수집/정합성 준비

- [ ] `source_event_key` 생성 규칙을 원천 시스템별로 합의했다.
- [ ] `source_version`, `source_sequence_no`, `source_updated_at` 중 어떤 최신성 필드를 우선 사용할지 정했다.
- [ ] `pull` 과 `push` 모두 공통 원시 적재 경로를 사용하도록 설계했다.
- [ ] `pending_reference` 와 재처리 큐 처리 기준을 정했다.
- [ ] `integration_endpoint` 기준 동시 실행 상한과 직렬화 키 기준을 정했다.
- [ ] `push`/`pull`/재처리/정합성 점검 배치의 우선순위와 슬롯 상한을 정했다.

### 4.4 API/운영 준비

- [x] `ingestion_api` 인증 방식 초안을 정했다.
- [x] 외부 시스템 연결 정보 등록/수정 UI 또는 관리자 API 경로를 정했다.
- [x] 자격증명(`password`, `token`, `client secret`) 암호화 저장 방식을 정했다.
- [ ] 화면/API/log 에서 민감정보 마스킹 규칙을 정했다.
- [ ] 실행 취소 요청 시 `cancelled` 와 `partially_completed` 판정 기준을 팀 내에서 합의했다.
- [ ] 취소 시 롤백하지 않을 대상(`raw_ingestion_event`, `audit_log`, 실행 이력)을 팀 내에서 합의했다.
- [ ] `integration_run.cancel_requested_at`, `cancel_requested_by`, `cancel_reason_code` 저장 여부와 의미를 구현 기준으로 확정했다.
- [ ] `sync-runs` 실행 API 와 조회 API 의 권한 범위를 정했다.
- [ ] `sync-runs/{run_id}/cancel` API 의 호출 권한과 종료 상태 예외 처리 규칙을 정했다.
- [ ] `sync-runs/{run_id}/retry` 와 `sync-runs/{run_id}/cancel` 의 `HTTP` 응답 코드와 오류 코드(`RUN_NOT_RETRIABLE`, `RUN_ALREADY_FINISHED` 등)를 정했다.
- [ ] 정합성 오류 조회/재처리 API 의 역할별 접근 범위를 정했다.
- [ ] 모든 실행 API 에 `idempotency_key` 와 감사 필드를 남기도록 합의했다.

### 4.5 검증 준비

- [ ] 구현 단위마다 먼저 작성할 실패 테스트 목록을 정했다.
- [x] 상태 전이, 재시도, 취소 감사 필드 기록 같은 서비스 핵심 로직에 단위 테스트를 우선 추가했다.
- [x] 빈 데이터베이스 부트스트랩 테스트 시나리오를 정의했다.
- [x] `push` 수신 후 원시 적재까지의 통합 테스트 시나리오를 정의했다.
- [x] `pull` 실행 후 원시 적재까지의 통합 테스트 시나리오를 정의했다.
- [x] 중복 이벤트, 늦게 도착한 이벤트, 참조 누락 이벤트 테스트 케이스를 정의했다.

## 6. 최근 진행 메모

- 2026-04-07 `backend/` 하위 `axum + sqlx` Rust 골격을 생성했고, `health`, `sync-runs` stub API, 서비스 단위 테스트를 추가했다.
- 2026-04-07 `backend/migrations/` 아래에 `integration_job`, `integration_run`, `raw_ingestion_event` 1차 마이그레이션을 추가했다.
- 2026-04-07 `ALM_BACKEND_DATABASE_URL`, `ALM_BACKEND_DATABASE_MAX_CONNECTIONS`, `ALM_BACKEND_AUTO_APPLY_MIGRATIONS` 설정을 정의하고, 앱 시작 시 `PgPool` 생성 및 선택적 migration 실행 경로를 연결했다.
- 2026-04-07 `ALM_BACKEND_TEST_DATABASE_ADMIN_URL` 기반 임시 테스트 데이터베이스 생성 헬퍼와 빈 `PostgreSQL` 부트스트랩/migration 통합 테스트를 추가했다.
- 2026-04-07 `integration_run` 저장 필드 확장 마이그레이션을 추가하고, `sync-runs` API가 `db_pool` 존재 시 `sqlx` 기반 `SyncRunRepository`를 사용하도록 연결했다.
- 2026-04-07 `POST /api/v1/ingestion/events` 라우트와 `RawIngestionRepository`를 추가하고, 원시 적재/멱등 처리/잘못된 timestamp 검증 테스트를 반영했다.
- 2026-04-07 `PullSyncOrchestrator` 를 추가해 수동 `pull` 실행이 하나의 `sync-run` 아래에서 `raw_ingestion_event` 로 적재되고, 성공/실패 건수에 따라 `completed` 또는 `partially_completed` 로 닫히는 경로를 테스트로 고정했다.
- 2026-04-08 `organization_master` 확장과 `workforce_master` 최소 마이그레이션을 추가했고, `master-data` 관리자 API 로 조직/인력 기준정보의 목록 조회와 최소 등록/수정 경로를 열었다.
- 2026-04-08 정적 운영 UI 프로토타입의 `organization`/`admin` 화면에 조직 마스터, 인력 기준정보, 연계 설정, `sync-runs` 운영 흐름을 반영했다.
- 2026-04-08 `organization`, `workforce` source object 를 표준화/매핑 대상에 포함했고, `pull` 오케스트레이터가 조직 마스터와 인력 마스터를 `organization -> workforce -> project -> work_item` 순서로 반영하도록 연결했다.
- 2026-04-08 `ingestion/events` 수신 후에도 `push` 후처리 오케스트레이션이 실행되어 조직/인력 이벤트가 즉시 표준화, 마스터 반영, `push_completed` 상태 종료까지 이어지도록 연결했다.
- 2026-04-08 `project.project_owner_workforce_id`, `work_item.owning_organization_id/assignee_workforce_id/reporter_workforce_id` 참조 컬럼을 추가했고, payload 의 조직 코드와 사번을 기준으로 실제 조직/인력 마스터 참조를 반영하도록 도메인 쓰기 서비스를 확장했다.
- 2026-04-08 운영자가 `project/work_item` 와 연결된 조직/인력 책임 정보를 바로 조회할 수 있도록 `GET /api/v1/admin/projects`, `GET /api/v1/admin/work-items` 최소 조회 API 를 추가했다.
- 2026-04-08 운영 UI 연결 전 점검 결과를 [current_backend_implementation_status_summary.md](./current_backend_implementation_status_summary.md) 에 정리했고, 운영 조회 API 는 DB 미연결 시 `200` 빈 목록 대신 `503 SERVICE_UNAVAILABLE` 를 반환하도록 보정했다.
- 2026-04-08 `src/ui_prototype/admin.html`, `src/ui_prototype/organization.html` 이 실제 운영 API 를 조회하도록 연결했고, 화면 상단에서 API 기준 URL 과 조직 코드 필터를 조정할 수 있게 했다.
- 2026-04-07 외부 시스템별 API 호출과 수신 payload 변환을 분리하기 위해 `pull`/`push` 어댑터 인터페이스와 `AdapterRegistry` 를 추가하고, `ingestion` 라우트와 `PullSyncOrchestrator` 가 이를 통해 시스템별 구현을 찾도록 연결했다.
- 2026-04-07 `Jira`, `Bitbucket`, `Bamboo`, `Confluence` concrete adapter 를 추가하고, 공통 `reqwest` 전송 계층 위에서 시스템별 URL 조합과 응답/payload 파싱을 구현했다.
- 2026-04-07 concrete adapter 단위 테스트와 레지스트리 기반 통합 테스트를 보강했고, 전체 `cargo test --manifest-path backend/Cargo.toml` 통과를 확인했다.
- 2026-04-07 기본 registry 생성을 `AdapterEndpointConfig` 기반 builder 로 재구성해, 환경변수 직접 참조와 후속 `integration_endpoint` 로더 경로가 같은 조립 함수를 재사용할 수 있게 정리했다.
- 2026-04-07 `integration_system`, `integration_endpoint`, `integration_credential` 최소 마이그레이션과 `DbAdapterConfigLoader` 를 추가해 앱 시작 시 DB 설정 기반 registry 를 우선 사용하고, DB 설정이 없을 때만 환경변수 기반 registry 로 fallback 하도록 연결했다.
- 2026-04-07 `secret_ciphertext` 복호화 서비스와 `DbAdapterConfigLoader` 연계를 추가해 DB 자격증명을 adapter 및 `push` 서명 검증 secret 으로 주입할 수 있게 정리했다.
- 2026-04-07 `RawIngestionRepository` 에 늦게 도착한 이벤트의 `stale`, 참조 미충족 payload 의 `pending_reference` 상태 판정 로직과 테스트를 추가했다.
- 2026-04-07 `POST /api/v1/ingestion/events` 에 `HMAC-SHA256` 기반 서명 검증과 필수 헤더 검사를 추가했다.
- 2026-04-07 `normalized_record_reference` 마이그레이션과 `NormalizationPipeline` 을 추가해 `pending` 원시 적재를 최소 표준 참조로 변환하고 `normalized` 상태 전이까지 구현했다.
- 2026-04-07 `PullSyncOrchestrator` 가 같은 실행 범위의 `pending` 이벤트를 즉시 표준화하도록 연결했고, 오케스트레이터 통합 테스트에서 표준 참조 생성까지 검증했다.
- 2026-04-07 `identity_mapping` 마이그레이션과 `IdentityMappingService` 를 추가해 표준화 결과와 함께 외부 식별자 `source_object_type:source_object_id` 를 내부 기준키에 매핑하도록 구현했다.
- 2026-04-07 `ReferenceResolutionService` 를 추가해 `pending_reference` 이벤트를 `identity_mapping` 기준으로 재평가하고, 해소된 이벤트를 다시 `pending` 으로 승격한 뒤 같은 `pull` 실행 안에서 재표준화되도록 연결했다.
- 2026-04-07 `organization_master`, `work_item_type`, `project`, `work_item` 최소 코어 테이블과 기본 시드를 추가했고, `ProjectWriteService`, `WorkItemWriteService` 를 통해 `pull` 실행 결과가 실제 도메인 테이블까지 반영되도록 연결했다.
- 2026-04-08 `work_item_status_history` 최소 테이블을 추가했고, `WorkItemWriteService` 가 payload 상태를 `work_item.current_*` 와 최신 상태 이력에 함께 반영하도록 보강했다.
- 2026-04-08 `work_item_hierarchy` 최소 테이블을 추가했고, `WorkItemWriteService` 가 payload 의 `parent_key` 를 읽어 단일 부모 기준 `work_item_hierarchy` 를 upsert 하도록 보강했다.
- 2026-04-08 `iteration`, `work_item_plan_link` 최소 테이블을 추가했고, `WorkItemWriteService` 가 payload 의 `iteration_name` 을 읽어 계획 단위를 upsert 하고 `work_item_plan_link` 를 함께 반영하도록 보강했다.

## 5. 구현 착수 전 최종 게이트

다음 조건을 모두 만족하면 실제 백엔드 구현을 시작할 수 있다.

- 마이그레이션 체계가 준비되어 있다.
- 필수 시드 범위가 고정되어 있다.
- `ingestion_api` 와 `sync-runs` 최소 계약이 팀 내에서 합의되어 있다.
- `pull`/`push` 공통 적재 경로와 최신성 판정 규칙이 구현 기준으로 확정되어 있다.
- 첫 스프린트 범위가 저장 기초선과 원시 적재 최소선으로 제한되어 있다.

## 6. 후속 보완 우선순위

### 6.1 구현과 병행 보완

- `ingestion_api` 인증/서명 정책
- 운영 배포 체크리스트
- 자격증명 암호화 키 회전 정책
- 워커 슬롯 설정의 환경별 구성 방식
- `Rust` 기준 런타임/프레임워크 매핑
- `axum + sqlx` 골격의 DB 연결 및 마이그레이션 실제 이식

### 6.2 1차 구현 이후 보완

- 읽기 모델/집계 모델 상세화
- 운영 대시보드 조회 최적화
- 기능 플래그와 호환성 전환 절차
