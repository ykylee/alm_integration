# 시스템 통합 DB 마이그레이션 및 시드 데이터 운영 전략 초안

- 문서 목적: 시스템 통합 DB 구축용 백엔드의 초기 릴리스 스키마 변경, 시드 데이터, 환경 승격, 롤백 기준을 정리한다.
- 범위: 마이그레이션 단위, 스키마 버전 관리, 시드 데이터 분류, 환경별 승격 절차, 롤백 원칙, 운영 가드레일
- 대상 독자: 백엔드 개발자, `DBA`, 아키텍트, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/initial_release_ddl_draft.md`, `docs/architecture/initial_release_physical_model_draft.md`, `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_backend_api_and_batch_contract_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 목적

시스템 통합 DB 백엔드는 테이블 수가 빠르게 늘고, 기준 코드와 운영 정책 데이터도 함께 관리해야 한다. 이때 스키마 변경과 시드 데이터 반영이 뒤섞이면 환경별 상태가 쉽게 어긋난다. 본 문서는 초기 릴리스에서 스키마 변경과 시드 반영을 어떻게 분리하고, 어떤 절차로 개발/검증/운영 환경에 승격할지 정의한다.

## 2. 운영 원칙

- 스키마 변경과 시드 데이터 반영은 같은 배포에 포함될 수 있어도 논리적으로는 분리한다.
- 모든 스키마 변경은 순차 버전의 마이그레이션 파일로 관리한다.
- 기준 코드와 정책 데이터는 수동 `insert` 가 아니라 관리 가능한 시드 단위로 반영한다.
- 운영 환경에서는 파괴적 변경을 즉시 적용하지 않고 단계적 전환을 우선한다.
- 롤백은 스키마 전체 되돌리기보다 “비호환 변경 회피 + 기능 비활성화 + 보정 마이그레이션”을 우선한다.

## 3. 마이그레이션 단위 전략

### 3.1 권장 분리 단위

- `schema`: 테이블, 컬럼, 제약, 인덱스 생성/변경
- `seed`: 기준 코드, 기본 정책, 기본 프로세스 모델, 기본 역할 정책 반영
- `backfill`: 기존 데이터 보정, 신규 컬럼 초기 채우기, 집계 재생성

이 세 단위를 하나의 파일에 섞지 않는 것이 원칙이다.

### 3.2 파일 구조 권장

- `migrations/schema/`
- `migrations/seed/`
- `migrations/backfill/`

권장 파일명 예시:

- `20260407_001_create_project_tables.sql`
- `20260407_002_create_work_item_tables.sql`
- `20260407_s001_seed_default_work_item_types.sql`
- `20260407_b001_backfill_work_item_status.sql`

## 4. 스키마 버전 관리 원칙

### 4.1 버전 추적

- 각 마이그레이션은 단방향 순차 적용을 기본으로 한다.
- 별도 `schema_migration_history` 테이블에 적용 여부를 남긴다.
- 기록 필드 최소 기준:
  - `migration_id`
  - `migration_type`
  - `applied_at`
  - `applied_by`
  - `checksum`
  - `execution_status`

### 4.2 실행 순서

1. `schema`
2. `seed`
3. `backfill`
4. 검증 쿼리

`backfill` 이 오래 걸릴 가능성이 있으면 기능 활성화와 분리해 배치로 수행한다.

## 5. 시드 데이터 분류 전략

### 5.1 초기 필수 시드

- `work_item_type`
- `process_model_definition`
- `workflow_scheme`
- `workflow_status_definition`
- `planning_scheme`
- `view_scheme`
- `role_policy`
- `permission_scope`
- 시스템 기본 조직/관리자 역할 코드

### 5.2 환경별로 달라질 수 있는 시드

- 외부 시스템 연결 정의
- 기본 관리자 계정 매핑
- 조직 초기 데이터 일부
- 원천 시스템 코드별 인증 메타데이터

이 범주는 공통 시드와 분리하고 환경 설정으로 다룬다.

비밀번호, `token`, `client secret` 같은 민감 자격증명은 시드 데이터로 평문 반영하지 않는다.

### 5.3 시드 변경 원칙

- 코드값은 가능하면 immutable 하게 유지한다.
- `display_name`, 설명, 활성 여부는 업데이트 가능하다.
- 삭제 대신 `is_active=false` 비활성화를 우선한다.
- 기존 데이터와 연결된 코드값은 rename 보다 별도 신규 코드 추가 후 전환을 우선한다.

## 6. 환경 승격 절차

### 6.1 권장 환경 흐름

- `local`
- `dev`
- `staging`
- `prod`

### 6.2 승격 절차

1. `local` 에서 마이그레이션 실행과 기본 시드 검증
2. `dev` 에서 수집 파이프라인과 API 기준 회귀 확인
3. `staging` 에서 운영과 유사한 데이터량 기준 성능/락 영향 검토
4. `prod` 에서 승인된 마이그레이션만 순차 적용

### 6.3 승격 체크 항목

- 신규 `DDL` 이 현재 애플리케이션 버전과 호환되는가
- 시드 데이터가 중복 적용되지 않는가
- 장시간 락을 유발하는 변경이 없는가
- 백필 작업이 온라인 수행 가능한가
- 장애 시 기능 비활성화 또는 우회가 가능한가

## 7. 파괴적 변경 대응 원칙

### 7.1 금지 또는 지양 항목

- 운영 중인 컬럼 즉시 삭제
- 대형 테이블 전면 재작성
- 긴 트랜잭션 안에서 대량 백필
- 코드값 직접 삭제

### 7.2 권장 절차

1. 신규 컬럼/테이블 추가
2. 애플리케이션 이중 쓰기 또는 호환 읽기 적용
3. 백필 수행
4. 검증 완료 후 구 경로 비활성화
5. 충분한 유예 후 구 컬럼/구 제약 제거

## 8. 롤백 및 장애 대응 원칙

### 8.1 롤백 우선순위

- 기능 플래그 비활성화
- 후속 보정 마이그레이션 적용
- 읽기 경로 우회
- 최후 수단으로 제한된 스키마 롤백

운영 환경에서는 “전체 down migration” 보다 “forward fix” 를 우선한다.

### 8.2 롤백 준비 항목

- 각 마이그레이션의 영향 범위 문서화
- 복구용 스냅샷 또는 백업 확보
- 시드 변경 전후 diff 기록
- 장시간 배치 중단/재개 절차 확보

## 9. 초기 릴리스 기준 권장 적용 순서

### 9.1 1차 스키마

- `organization_master`
- `workforce_master`
- `project`
- `work_item_type`
- `work_item`
- `work_item_hierarchy`

### 9.2 2차 스키마

- `process_model_definition`
- `workflow_scheme`
- `workflow_status_definition`
- `workflow_transition_definition`
- `planning_scheme`
- `view_scheme`
- `project_process_model`

### 9.3 3차 스키마

- `work_item_status_history`
- `work_item_plan_link`
- `role_policy`
- `permission_scope`
- `role_assignment`
- `audit_log`

### 9.4 4차 운영 스키마

- 원시 적재/동기화 실행 이력
- 참조 정합성 이슈
- 재처리 큐
- 운영 대시보드용 보조 테이블 또는 집계 객체

## 10. 검증 기준

### 10.1 스키마 검증

- 모든 `schema` 마이그레이션이 빈 데이터베이스에서 순차 적용 가능해야 한다.
- 필수 시드까지 포함한 부트스트랩이 실패 없이 완료되어야 한다.

### 10.2 회귀 검증

- `ingestion_api` 수신 후 원시 적재 테이블과 도메인 테이블 반영이 정상이어야 한다.
- `sync_run` 생성과 정합성 오류 적재가 스키마 변경 후에도 유지되어야 한다.
- 기본 조회 API 가 신규 인덱스를 활용할 수 있어야 한다.

### 10.3 운영 검증

- `staging` 에서 마이그레이션 실행 시간과 락 시간을 측정해야 한다.
- 운영 배포 전 `seed diff` 와 예상 영향 보고를 남겨야 한다.

## 11. 운영 가드레일

- 운영 마이그레이션은 승인된 배포 윈도우에서만 수행한다.
- 동일 배포에서 고위험 스키마 변경과 대량 백필을 동시에 수행하지 않는다.
- 시드 변경은 감사 가능한 변경 이력과 함께 반영한다.
- 정책/코드값 변경은 가능하면 API 와 시드 둘 중 하나의 단일 경로로만 수행한다.
- 민감 자격증명은 운영 시드가 아니라 관리자 입력 API 또는 별도 보안 주입 절차로만 등록한다.

## 12. 후속 상세화 후보

- `schema_migration_history` 테이블 `DDL`
- 초기 시드 파일 목록과 소유자 정의
- `staging` 검증 체크리스트
- 기능 플래그 및 호환성 전환 전략
