# 마이그레이션 디렉터리

- 문서 목적: 초기 백엔드 스캐폴딩 기준의 마이그레이션 디렉터리 구조와 사용 의도를 설명한다.
- 범위: `schema`, `seed`, `backfill` 디렉터리 역할
- 대상 독자: 백엔드 개발자, `DBA`, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_db_migration_and_seed_strategy_draft.md`, `docs/architecture/initial_release_ddl_draft.md`

## 구조

- `schema/`: 스키마 생성과 변경 마이그레이션
- `seed/`: 필수 기준 코드와 메타데이터 시드
- `backfill/`: 운영 데이터 보정과 대량 백필 작업

현재는 디렉터리 골격만 생성했고, 실제 Alembic 환경 파일과 첫 마이그레이션은 다음 단계에서 추가한다.
