# Rust `axum + sqlx` 채택 계획 초안

- 문서 목적: 백엔드 1차 기술 스택 검토안 중 `Rust` 구현 조합을 `axum + sqlx` 로 구체화하고, 실제 구현 전환 계획을 정리한다.
- 범위: 선택 배경, 대안 비교, 권장 조합, 단계별 전환 순서, 임시 Python 골격 처리 원칙, 남은 의사결정 항목
- 대상 독자: 아키텍트, 백엔드 개발자, 기술 리드, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_backend_implementation_rollout_and_checklist_draft.md`, `docs/architecture/integration_backend_api_and_batch_contract_draft.md`, `docs/architecture/initial_release_ddl_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 설계 플랜: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 권장 조합

현재 1차 권장 조합은 다음과 같다.

- 웹 프레임워크: `axum`
- 런타임: `tokio`
- DB 접근 계층: `sqlx`
- 마이그레이션: `sqlx migrate`
- 직렬화: `serde`
- 로깅/추적: `tracing`, `tracing-subscriber`

## 2. 선택 배경

현재 백엔드 요구는 다음 특성이 강하다.

- `push`/`pull` 혼합 수집
- 운영 API 와 배치/워커 동시 지원
- raw SQL 과 `DDL` 중심 설계
- 멱등성, 최신성 판정, 취소/재시도, 워커 슬롯 정책 같은 운영 로직 중심

이 조건에서는 ORM 중심보다 SQL 제어권이 높은 조합이 더 적합하다. 따라서 `axum + sqlx` 를 우선 조합으로 본다.

## 3. 대안 비교 요약

### 3.1 `axum + sqlx`

- 장점:
  - `tokio`, `tower`, `hyper` 생태계와 정렬이 좋다.
  - 미들웨어, 인증, 추적, 백그라운드 작업 구조를 붙이기 쉽다.
  - 현재 `DDL` 초안과 SQL 중심 운영 로직을 자연스럽게 구현할 수 있다.
- 단점:
  - ORM 수준 자동화는 적고, 쿼리와 리포지토리 코드를 더 직접 작성해야 한다.

### 3.2 `actix-web + sqlx`

- 장점:
  - 성숙했고 성능 사례가 많다.
- 단점:
  - 현재 요구에서는 `axum` 대비 구조적 이점이 크지 않다.

### 3.3 `axum + SeaORM`

- 장점:
  - CRUD 와 관계 모델링은 상대적으로 편하다.
- 단점:
  - 현재 프로젝트는 수집, 배치, 운영성, SQL 제어 요구가 강해 ORM 중심 접근이 과할 수 있다.

## 4. Python 임시 골격 처리 원칙

현재 저장소의 Python 백엔드 골격은 다음 역할만 유지한다.

- API 형태 빠른 검토
- 운영 API 계약 확인
- 디렉터리 구조 비교 참고

다음 역할은 맡기지 않는다.

- 최종 구현 기준
- 실서비스 서버 기반
- 최종 마이그레이션 실행 체계

즉, Rust 골격이 들어가기 시작하면 Python 코드는 임시 검증용 또는 제거 대상으로 본다.

## 5. 단계별 전환 계획

### STEP-RS-01 조합 확정

- `axum + sqlx + PostgreSQL + sqlx migrate` 를 1차 구현 기준으로 문서 확정

### STEP-RS-02 Rust 골격 생성

- `cargo` 프로젝트 생성
- 설정, 로깅, 헬스체크, 기본 라우터 추가

### STEP-RS-03 운영 API 최소선 이식

- `sync-runs` 실행/조회/취소/재시도 API 최소선 이식

### STEP-RS-04 DB 연결과 마이그레이션 이식

- `sqlx` 연결
- `sqlx migrate` 초기 구조 생성
- 핵심 테이블 일부 우선 이식

### STEP-RS-05 수집/정규화 서비스 구조 이식

- `pull`/`push` 공통 적재 경로
- 상태 전이 모델
- 참조 정합성 서비스 경계 반영

## 6. 권장 디렉터리 구조 초안

```text
backend/
  Cargo.toml
  src/
    main.rs
    config.rs
    http/
      router.rs
      routes/
    application/
    domain/
    infrastructure/
      db/
      repository/
    jobs/
  migrations/
  tests/
```

프로젝트 위치는 `backend/` 하위 격리로 확정한다.

## 7. 남은 결정 항목

- `sqlx` offline mode 사용 여부
- 마이그레이션 디렉터리를 기존 `migrations/` 와 통합할지 분리할지
- 테스트용 `PostgreSQL` 실행 방식을 무엇으로 둘지

## 8. 바로 다음 작업 제안

1. `axum + sqlx + PostgreSQL + sqlx migrate` 조합을 공식 1차 구현 기준으로 확정
2. Rust 골격 생성
3. `sync-runs` 운영 API 최소선 이식
4. `sqlx` offline mode 여부와 마이그레이션 운영 방식을 결정

지금 단계에서는 후보 비교를 더 늘리기보다 Rust 골격 착수 준비를 바로 시작하는 편이 맞다.
