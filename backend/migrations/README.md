# Rust 백엔드 마이그레이션 디렉터리

- 문서 목적: `backend/` 하위 Rust 프로젝트에서 사용할 `sqlx migrate` 기준 마이그레이션 위치를 설명한다.
- 범위: Rust 전용 마이그레이션 디렉터리 역할과 기존 루트 `migrations/` 와의 관계
- 대상 독자: 백엔드 개발자, `DBA`, 기술 리드
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/rust_axum_sqlx_adoption_plan.md`, `docs/architecture/integration_db_migration_and_seed_strategy_draft.md`

현재는 Rust 전용 마이그레이션 위치를 `backend/migrations/` 로 두고, 실제 `sqlx migrate add` 파일은 다음 단계에서 추가한다.
