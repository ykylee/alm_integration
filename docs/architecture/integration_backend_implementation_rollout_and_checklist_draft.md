# 시스템 통합 DB 백엔드 초기 구현 순서 및 개발 착수 체크리스트

- 문서 목적: 현재까지 작성된 백엔드 설계 문서를 기준으로 초기 구현 순서, 선행조건, 개발 착수 체크리스트를 정리한다.
- 범위: 문서 리뷰 요약, 초기 구현 단계, 단계별 산출물, 개발 착수 체크리스트, 남은 보완 과제
- 대상 독자: 백엔드 개발자, 아키텍트, `DBA`, 운영자, 기술 리드
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_backend_component_draft.md`, `docs/architecture/integration_data_ingestion_sequence_draft.md`, `docs/architecture/integration_backend_api_and_batch_contract_draft.md`, `docs/architecture/integration_db_migration_and_seed_strategy_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 문서 리뷰 요약

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

### 3.1 첫 스프린트 권장 범위

- 마이그레이션 실행 기반
- 핵심 테이블 생성
- 필수 시드 반영
- 원시 적재 테이블
- `ingestion_api` 단건 수신
- `sync_run` 생성/조회 최소선

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

- [ ] 선택할 `RDBMS` 와 마이그레이션 도구를 확정했다.
- [ ] 개발 환경에서 빈 데이터베이스 생성과 초기화 절차를 합의했다.
- [ ] 마이그레이션 파일 디렉터리 구조(`schema/seed/backfill`)를 준비했다.
- [ ] 필수 시드 목록과 소유자를 정했다.

### 4.2 저장 모델 준비

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

- [ ] `ingestion_api` 인증 방식 초안을 정했다.
- [ ] 외부 시스템 연결 정보 등록/수정 UI 또는 관리자 API 경로를 정했다.
- [ ] 자격증명(`password`, `token`, `client secret`) 암호화 저장 방식을 정했다.
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

- [ ] 빈 데이터베이스 부트스트랩 테스트 시나리오를 정의했다.
- [ ] `push` 수신 후 원시 적재까지의 통합 테스트 시나리오를 정의했다.
- [ ] `pull` 실행 후 표준화/반영까지의 통합 테스트 시나리오를 정의했다.
- [ ] 중복 이벤트, 늦게 도착한 이벤트, 참조 누락 이벤트 테스트 케이스를 정의했다.

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

### 6.2 1차 구현 이후 보완

- 읽기 모델/집계 모델 상세화
- 운영 대시보드 조회 최적화
- 기능 플래그와 호환성 전환 절차
