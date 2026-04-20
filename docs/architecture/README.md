# 아키텍처 위키

- 문서 목적: 시스템 아키텍처 설계 문서의 진입점과 작성 기준을 제공한다.
- 범위: `docs/architecture/` 아래의 아키텍처 초안, 구성도, 데이터 흐름, 설계 계획 문서
- 대상 독자: 아키텍트, 기획자, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-17
- 관련 문서: `docs/README.md`, `docs/requirements/system_srs.md`, `docs/overview/project_overview.md`

## 이 카테고리에서 다루는 내용

- 시스템 컨텍스트와 경계
- 논리 아키텍처와 핵심 컴포넌트
- 데이터 흐름, 연계 구조, 운영 경계
- 설계 초안 작성 계획과 진행 상태

## 문서 목록

- 아키텍처 설계 초안 작성 계획: [architecture_drafting_plan.md](./architecture_drafting_plan.md)
- 시스템 아키텍처 초안: [system_architecture_draft.md](./system_architecture_draft.md)
- 시스템 통합 DB 백엔드 설계 플랜: [integration_backend_design_plan.md](./integration_backend_design_plan.md)
- 시스템 통합 DB 백엔드 컴포넌트 초안: [integration_backend_component_draft.md](./integration_backend_component_draft.md)
- 시스템 통합 DB 데이터 수집 시퀀스 초안: [integration_data_ingestion_sequence_draft.md](./integration_data_ingestion_sequence_draft.md)
- 시스템 통합 DB 백엔드 API 및 배치 계약 초안: [integration_backend_api_and_batch_contract_draft.md](./integration_backend_api_and_batch_contract_draft.md)
- 시스템 통합 DB 마이그레이션 및 시드 데이터 운영 전략 초안: [integration_db_migration_and_seed_strategy_draft.md](./integration_db_migration_and_seed_strategy_draft.md)
- 시스템 통합 DB 초기 구현 순서 및 개발 착수 체크리스트: [integration_backend_implementation_rollout_and_checklist_draft.md](./integration_backend_implementation_rollout_and_checklist_draft.md)
- 외부 시스템 어댑터 연동 가이드: [integration_adapter_onboarding_guide.md](./integration_adapter_onboarding_guide.md)
- Rust `axum + sqlx` 채택 계획 초안: [rust_axum_sqlx_adoption_plan.md](./rust_axum_sqlx_adoption_plan.md)
- 시스템 통합 DB 인증 및 서명 정책 초안: [integration_auth_and_signing_policy_draft.md](./integration_auth_and_signing_policy_draft.md)
- 시스템 통합 DB 동기화 실행 상태 전이 초안: [integration_run_state_transition_draft.md](./integration_run_state_transition_draft.md)
- 시스템 통합 DB 워커 슬롯 및 실행 취소/롤백 정책 초안: [integration_worker_slot_and_cancellation_policy_draft.md](./integration_worker_slot_and_cancellation_policy_draft.md)
- 시스템 서비스 분류 및 `MSA` 적용 판단 초안: [service_classification_and_msa_assessment_draft.md](./service_classification_and_msa_assessment_draft.md)
- 통합 백엔드 구현 현황 요약: [current_backend_implementation_status_summary.md](./current_backend_implementation_status_summary.md)
- 백엔드 코드 인덱스: [backend_code_index.md](./backend_code_index.md)
- 운영 UI 코드 인덱스: [admin_ui_code_index.md](./admin_ui_code_index.md)
- 시스템 컨텍스트 및 연계 아키텍처 초안: [system_context_and_integration_draft.md](./system_context_and_integration_draft.md)
- 핵심 도메인 모델 초안: [domain_model_draft.md](./domain_model_draft.md)
- 핵심 엔터티 정의 초안: [domain_entity_definition_draft.md](./domain_entity_definition_draft.md)
- 초기 릴리스 ERD 초안: [initial_release_erd_draft.md](./initial_release_erd_draft.md)
- 초기 릴리스 물리 모델 초안: [initial_release_physical_model_draft.md](./initial_release_physical_model_draft.md)
- 초기 릴리스 DDL 초안: [initial_release_ddl_draft.md](./initial_release_ddl_draft.md)
- 논리 참조 규칙 초안: [logical_reference_rules_draft.md](./logical_reference_rules_draft.md)
- 다형 참조 무결성 검증 초안: [polymorphic_reference_validation_draft.md](./polymorphic_reference_validation_draft.md)
- 참조 정합성 점검 배치 및 오류 큐 초안: [reference_integrity_batch_and_error_queue_draft.md](./reference_integrity_batch_and_error_queue_draft.md)
- 참조 정합성 운영 DDL 초안: [reference_integrity_operations_ddl_draft.md](./reference_integrity_operations_ddl_draft.md)
- 참조 정합성 대시보드 조회 모델 초안: [reference_integrity_dashboard_query_model_draft.md](./reference_integrity_dashboard_query_model_draft.md)
- 참조 정합성 대시보드 UI 및 접근 제어 초안: [reference_integrity_dashboard_ui_and_access_draft.md](./reference_integrity_dashboard_ui_and_access_draft.md)
- 응용 서비스 및 운영 거버넌스 아키텍처 초안: [application_and_governance_architecture_draft.md](./application_and_governance_architecture_draft.md)

## 다음에 읽을 문서

- 프로젝트 개요: [../overview/project_overview.md](../overview/project_overview.md)
- 통합 중앙 관리 시스템 CRS: [../requirements/system_crs.md](../requirements/system_crs.md)
- 통합 중앙 관리 시스템 SRS: [../requirements/system_srs.md](../requirements/system_srs.md)
