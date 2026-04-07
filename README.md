# Schubert

여러 운영 시스템에 흩어진 정보를 한 곳에 연결하고 표준화해 관리하기 위한 통합 중앙 관리 시스템 프로젝트 저장소입니다.

## 프로젝트 소개

조직 내에서는 ALM, 형상관리 시스템, 문서관리 시스템, CI/CD, 분석 도구 등이 각각 독립적으로 운영되는 경우가 많습니다. 이 저장소의 목표는 이런 분산 환경을 대체하는 것이 아니라, 각 시스템의 핵심 정보를 연결하고 공통 기준으로 정리해 더 높은 가시성과 통제를 제공하는 통합 관리 체계를 설계하고 구현하는 것입니다.

이 프로젝트는 다음과 같은 문제를 해결하는 데 초점을 둡니다.

- 여러 도구에 분산된 프로젝트 현황과 산출물 상태를 통합 조회
- 변경, 문서, 빌드, 배포, 품질 정보 사이의 추적성 확보
- 시스템별로 다른 식별자와 메타데이터를 표준 모델로 정규화
- 운영 보고, 감사 대응, 상태 점검에 필요한 기준 정보 일관화

## 지향점

- 외부 시스템의 고유 기능을 복제하기보다 정보를 연결하고 표준화하는 통합 계층 제공
- 프로젝트, 애플리케이션, 산출물, 변경, 배포, 품질 정보를 한 기준으로 관리
- 대시보드, 검색, 보고를 통해 운영 가시성과 의사결정 속도 향상
- 권한, 연계 상태, 데이터 품질을 중앙에서 통제할 수 있는 기반 마련

## 문서 읽기 시작점

상세 배경과 요구사항은 `docs/` 아래 위키 구조로 관리합니다.

- 문서 위키 홈: [`docs/README.md`](./docs/README.md)
- 프로젝트 개요: [`docs/overview/project_overview.md`](./docs/overview/project_overview.md)
- UI/UX 프로토타입 설명: [`docs/overview/ui_ux_prototype.md`](./docs/overview/ui_ux_prototype.md)
- 컨셉 수준 요구사항 `CRS`: [`docs/requirements/system_crs.md`](./docs/requirements/system_crs.md)
- 상세 요구사항 `SRS`: [`docs/requirements/system_srs.md`](./docs/requirements/system_srs.md)
- 작업 백로그 인덱스: [`docs/operations/work_backlog.md`](./docs/operations/work_backlog.md)

## 현재 저장소 구조

- `docs/` 프로젝트 개요, 요구사항, 운영 문서, 의사결정 기록
- `src/` 애플리케이션 또는 라이브러리 코드
- `tests/` 자동화 테스트
- `assets/` 이미지, 샘플 데이터, fixture 등 정적 자산
- `migrations/` 스키마, 시드, 백필 마이그레이션 골격

## 개발 명령

- `make install-dev` 임시 Python 골격 개발 의존성 설치
- `make run` 임시 Python API 서버 실행
- `make test` 임시 Python 테스트 실행
- `cargo run --manifest-path backend/Cargo.toml` Rust 백엔드 실행
- `cargo test --manifest-path backend/Cargo.toml` Rust 백엔드 테스트 실행

## 기술 스택 검토 상태

- 프론트엔드 1차 검토안: `React`
- 백엔드 1차 검토안: `Rust`
- 현재 저장소의 백엔드 스캐폴딩은 `FastAPI + SQLAlchemy + Alembic` 기준의 임시 검증 골격이며, 최종 구현 스택 확정 전까지 구조 검토와 API 형태 확인 용도로만 사용합니다.
- 실제 1차 구현 골격은 `backend/` 하위의 `axum + sqlx` 프로젝트를 기준으로 발전시킵니다.
- 현재 `sync-runs` 운영 API 는 인메모리 stub 으로만 동작합니다.

## 참고

최신 작업 이력과 운영 규칙은 `docs/operations/` 아래 문서를 기준으로 확인합니다.
