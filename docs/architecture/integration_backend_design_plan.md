# 시스템 통합 DB 백엔드 설계 플랜

- 문서 목적: 시스템 통합 DB 구축용 백엔드 개발을 우선순위로 두고, 필요한 아키텍처 보완 항목과 설계 진행 순서를 정리한다.
- 범위: 백엔드 런타임 구조, 연계 수집/정규화/저장 흐름, 서비스 경계, 데이터 저장 전략, 운영 배치, 구현 준비 산출물
- 대상 독자: 아키텍트, 백엔드 개발자, 데이터 모델러, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/system_architecture_draft.md`, `docs/architecture/system_context_and_integration_draft.md`, `docs/architecture/initial_release_physical_model_draft.md`, `docs/architecture/initial_release_ddl_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./architecture_drafting_plan.md](./architecture_drafting_plan.md)

## 1. 배경

현재 아키텍처 문서는 도메인 모델, `ERD`, 물리 모델, `DDL`, 운영 보완 구조까지는 상당 부분 정리된 상태다. 다만 실제 개발 우선순위를 시스템 통합 DB 구축용 백엔드에 두려면, 문서를 구현 관점으로 다시 묶어야 한다. 특히 다음 질문에 바로 답할 수 있어야 한다.

- 어떤 백엔드 서비스가 어떤 쓰기 책임을 가지는가
- 외부 시스템 수집 데이터는 어떤 단계로 저장되고 정규화되는가
- API, 배치, 재처리, 감사는 어떤 런타임 경계로 분리되는가
- 초기 릴리스에서 어떤 테이블과 어떤 서비스부터 구현하는가

본 문서는 위 질문에 답하기 위한 설계 플랜이다.

## 2. 현 시점 검토 결과

### 2.1 이미 비교적 정리된 항목

- 도메인 경계와 핵심 엔터티
- 초기 릴리스 `ERD`
- 초기 릴리스 물리 모델과 `DDL`
- 다형 참조 무결성 검증 원칙
- 운영 보완용 오류 큐, 배치, 대시보드 구조

### 2.2 현재까지 보완 완료된 항목

- 백엔드 런타임 서비스 분해
- `pull`/`push` 공통 수집 파이프라인과 충돌 방지 규칙
- 초기 운영 API 와 내부 배치 계약
- `DB` 마이그레이션 및 시드 데이터 운영 전략
- `FastAPI + SQLAlchemy + Alembic` 기준의 초기 백엔드 스캐폴딩
- `axum + sqlx` 기준의 초기 Rust 골격, 첫 마이그레이션, `PgPool` 초기화 경로
- 빈 `PostgreSQL` 기준 부트스트랩 및 migration 통합 테스트
- `sync-runs` 경로의 `sqlx` 리포지토리 연결과 `integration_run` API 저장 필드 확장
- `POST /api/v1/ingestion/events` 원시 적재 경로와 멱등 처리 저장소 연결
- `PullSyncOrchestrator` 기반 수동 `pull` 실행의 원시 적재 경로와 실행 상태 종료 처리
- 시스템별 `pull`/`push` 세부 구현을 분리하기 위한 어댑터 레지스트리와 공통 인터페이스
- `Jira`, `Bitbucket`, `Bamboo`, `Confluence` 기준 concrete `pull`/`push` 어댑터와 공통 `HTTP` 전송 계층
- 새 외부 시스템 추가 시 따라갈 수 있는 adapter 온보딩 가이드
- 환경변수 기반 기본 등록과 별도로 `AdapterEndpointConfig` 배열에서 registry 를 조립하는 설정 레코드 builder

위 스캐폴딩은 구조 검증용 임시 골격이다. 2026-04-07 기준 최종 구현 스택의 1차 검토안은 프론트엔드 `React`, 백엔드 `Rust` 이다.

### 2.3 현재 남아 있는 보완 포인트

1. 외부 수신 인증/서명 정책 상세 보완 필요  
초기 `HMAC` 서명 검증과 필수 헤더 검사는 구현했지만, 원천 시스템별 헤더 규격 차이와 키 보관소 연계 방식은 후속 상세화가 더 필요하다.

2. 읽기 모델/집계 모델 상세 부족  
초기 구현에는 큰 문제가 없지만, 운영 화면과 조회 최적화를 위해 후속 설계가 필요하다.

3. 자격증명 암호화 키 관리 상세 부족  
외부 시스템 연결 자격증명 암호화 저장 원칙은 반영됐지만, 키 보관/회전/폐기 정책은 후속 보조 문서가 필요하다.

4. 워커 슬롯 상한의 실제 런타임 설정 방식 확정 필요  
정책 초안과 취소 감사 컬럼 반영은 완료됐지만, 환경별 슬롯 상한값과 큐 우선순위 설정을 어떤 구성 방식으로 관리할지는 구현 직전에 다시 확정해야 한다.

5. `Rust` 기준 런타임/DB 부트스트랩 보강 필요  
`backend/` 하위 `axum + sqlx` 골격과 첫 마이그레이션, `PgPool` 초기화 경로는 추가됐지만, 실제 `PostgreSQL` 연결 검증, 빈 데이터베이스 부트스트랩 테스트, 시드 데이터 적용 흐름은 후속 구현 단계에서 이어서 닫아야 한다.

6. 시스템별 어댑터 자격증명 복호화 계층 보강 필요  
`secret_ciphertext` 를 복호화해 adapter 및 `push` 서명 검증 경로에 주입하는 애플리케이션 계층은 구현했지만, 실제 운영용 외부 키 보관소 연계와 키 회전 자동화는 후속 구현으로 남아 있다.

## 3. 설계 목표

- 초기 릴리스 백엔드의 실행 가능한 서비스 경계를 먼저 고정한다.
- 시스템 통합 DB 저장 흐름을 수집, 표준화, 운영 통제로 나눠 책임을 분리한다.
- 초기 구현 범위를 API, 배치, `DDL`, 운영 보완으로 분리해 개발 순서를 명확히 한다.
- 이후 UI/프로토타입 작업보다 먼저 백엔드 저장 구조와 서비스 계약을 안정화한다.

## 4. 우선 설계 대상

### 4.1 백엔드 서비스 경계

- 연계 수집 서비스
- 표준화/식별자 매핑 서비스
- 업무 항목/프로젝트 저장 서비스
- 조직/인력 기준정보 서비스
- 운영 통제 서비스
- 참조 정합성 운영 서비스

### 4.2 저장 파이프라인

- 원시 적재
- 정규화
- 식별자 매핑
- 업무 엔터티 반영
- 오류 적재
- 재처리

### 4.3 초기 운영 인터페이스

- 동기화 실행/조회 API
- 정합성 오류 조회/조치 API
- 프로젝트/업무 항목 조회 API
- 기준정보 관리 API
- 운영 배치 실행 인터페이스

## 5. 단계별 설계 플랜

| 단계 | 목표 | 상태 | 산출물 |
| --- | --- | --- | --- |
| STEP-BE-01 | 백엔드 우선 관점의 보완 필요 항목 검토와 계획 수립 | done | 본 문서 |
| STEP-BE-02 | 런타임 서비스 경계와 백엔드 컴포넌트 분해 | done | `integration_backend_component_draft.md` |
| STEP-BE-03 | 연계 수집/정규화/저장 시퀀스 상세화 | done | `integration_data_ingestion_sequence_draft.md` |
| STEP-BE-04 | 초기 API/배치 인터페이스 초안 작성 | done | `integration_backend_api_and_batch_contract_draft.md` |
| STEP-BE-05 | `DB` 마이그레이션 및 시드 데이터 운영 전략 정리 | done | `integration_db_migration_and_seed_strategy_draft.md` |
| STEP-BE-06 | 초기 구현 순서와 개발 착수 기준 확정 | done | `integration_backend_implementation_rollout_and_checklist_draft.md` |

## 6. 우선순위 판단

### 6.1 최우선

- 저장 모델과 서비스 쓰기 책임 확정
- 연계 수집 파이프라인 상세화
- 초기 `DDL`과 마이그레이션 전략 정리

### 6.2 그 다음

- 운영 배치/재처리 계약
- 조회 API와 운영 API 범위 정리
- 읽기 모델/집계 모델 생성 방식 정리

### 6.3 후순위

- 운영 대시보드 상세 UI
- 프로토타입 반영
- 고도화된 운영 알림 정책

## 7. 첫 구현 기준 제안

초기 백엔드 구현은 다음 순서가 가장 안전하다.

1. `project`, `work_item`, `work_item_type`, `project_process_model` 등 핵심 테이블과 마이그레이션 체계 구축
2. 조직/인력 기준정보 저장과 식별자 매핑 구조 구현
3. 연계 수집 실행 이력과 원시 적재 구조 구현
4. 정규화와 핵심 엔터티 반영 서비스 구현
5. 참조 정합성 오류 적재와 운영 API 구현

즉, 화면보다 먼저 “저장 구조 + 수집 파이프라인 + 운영 통제 최소선”을 닫는 방향이 적절하다.

## 8. 바로 이어서 보완할 문서

- `system_context_and_integration_draft.md`
  백엔드 컴포넌트와 런타임 경계를 더 구체화해야 한다.
- `application_and_governance_architecture_draft.md`
  운영 API와 관리자 실행 경계를 더 명확히 해야 한다.
- 신규 문서 필요:
- 필요 시 운영 배포 체크리스트 보조 문서
- 필요 시 키 보관소 연계 상세 문서

## 9. 진행 관리 원칙

- 본 문서를 백엔드 설계 작업의 기준 계획 문서로 사용한다.
- 후속 작업이 진행되면 각 단계 상태와 산출물을 본 문서에 갱신한다.
- 구현 착수 전에 최소 `STEP-BE-04` 까지는 문서 초안을 확보하는 것을 목표로 한다.
- `STEP-BE-02` 산출물은 [integration_backend_component_draft.md](./integration_backend_component_draft.md)로 관리한다.
- `STEP-BE-03` 산출물은 [integration_data_ingestion_sequence_draft.md](./integration_data_ingestion_sequence_draft.md)로 관리한다.
- `STEP-BE-04` 산출물은 [integration_backend_api_and_batch_contract_draft.md](./integration_backend_api_and_batch_contract_draft.md)로 관리한다.
- `STEP-BE-05` 산출물은 [integration_db_migration_and_seed_strategy_draft.md](./integration_db_migration_and_seed_strategy_draft.md)로 관리한다.
- `STEP-BE-06` 산출물은 [integration_backend_implementation_rollout_and_checklist_draft.md](./integration_backend_implementation_rollout_and_checklist_draft.md)로 관리한다.
- 인증/서명 보조 산출물은 [integration_auth_and_signing_policy_draft.md](./integration_auth_and_signing_policy_draft.md)로 관리한다.
- 실행 상태 전이 보조 산출물은 [integration_run_state_transition_draft.md](./integration_run_state_transition_draft.md)로 관리한다.
- 워커 슬롯 및 취소/롤백 보조 산출물은 [integration_worker_slot_and_cancellation_policy_draft.md](./integration_worker_slot_and_cancellation_policy_draft.md)로 관리한다.
- `sync-runs` 취소 API 와 실행 상세 응답 보강은 [integration_backend_api_and_batch_contract_draft.md](./integration_backend_api_and_batch_contract_draft.md)에서 계속 관리한다.
- `Rust` 구현 전환 산출물은 [rust_axum_sqlx_adoption_plan.md](./rust_axum_sqlx_adoption_plan.md)에서 관리한다.
- 새 외부 시스템 연계 절차 가이드는 [integration_adapter_onboarding_guide.md](./integration_adapter_onboarding_guide.md)에서 관리한다.
