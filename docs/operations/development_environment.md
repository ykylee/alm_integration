# 개발 환경 및 테스트 환경 가이드

- 문서 목적: 이 저장소를 처음 사용하는 개발자가 로컬 개발 환경과 격리된 테스트 환경을 재현할 수 있도록 기준 절차를 제공한다.
- 범위: Python 임시 골격, Rust 백엔드, PostgreSQL, 컨테이너 기반 테스트 실행 경로
- 대상 독자: 개발자, 리뷰어, 운영자
- 상태: draft
- 최종 수정일: 2026-04-15
- 관련 문서: `README.md`, `docs/operations/work_backlog.md`, `docs/operations/environments/README.md`, `docs/architecture/current_backend_implementation_status_summary.md`

## 문서 개요

현재 저장소는 임시 Python 골격과 실제 1차 구현 기준인 Rust 백엔드를 함께 포함한다. 로컬 설치 경로와 컨테이너 격리 경로를 모두 제공하되, 테스트는 가능하면 컨테이너에서 실행하는 것을 기준으로 한다.

## 현재 기준 환경

- Python: `3.12` 이상
- Rust: `cargo` 가 포함된 stable toolchain
- 데이터베이스: `PostgreSQL 17`
- 컨테이너 런타임: `docker` 우선, `podman` 대체 가능

## 환경 파일

- 로컬 호스트 실행용: [../../.env.example](../../.env.example)
- 컨테이너 네트워크 실행용: [../../.env.docker.example](../../.env.docker.example)

차이점:

- `.env.example` 는 DB 호스트를 `localhost` 로 둔다.
- `.env.docker.example` 는 컨테이너 간 통신을 위해 DB 호스트를 `alm-postgres` 로 둔다.

## 권장 시작 순서

1. Python 의존성이 필요하면 `make install-dev` 를 실행한다.
2. 로컬 DB 가 필요하면 `CONTAINER_RUNTIME=docker make infra-up` 또는 `CONTAINER_RUNTIME=podman make infra-up` 을 실행한다.
3. Python 임시 골격은 `make test` 로 검증한다.
4. Rust 백엔드는 로컬 툴체인이 준비돼 있으면 `make backend-test` 로 검증한다.
5. 격리된 테스트는 `make container-test-python`, `make container-test-rust`, `make container-test` 순서로 사용한다.

## 컨테이너 기반 개발 및 테스트

`Makefile` 은 `CONTAINER_RUNTIME` 변수로 런타임을 바꿀 수 있다.

`podman` 호환 기본값:

- 기본 `PostgreSQL` 이미지는 short-name 해석 이슈를 피하기 위해 `docker.io/library/postgres:17` 을 사용한다.
- 컨테이너 소스 마운트는 SELinux 환경을 고려해 `:Z` 옵션을 사용한다.
- Rust 테스트 이미지는 저장소 의존성과 맞추기 위해 `docker.io/library/rust:1.94-bookworm` 기준으로 유지한다.

예시:

```bash
CONTAINER_RUNTIME=docker make infra-up
CONTAINER_RUNTIME=docker make container-test-python
CONTAINER_RUNTIME=docker make container-test-rust
CONTAINER_RUNTIME=podman make infra-up
CONTAINER_RUNTIME=podman make container-test-rust
```

주요 대상:

- `infra-up`: PostgreSQL 컨테이너와 전용 네트워크 생성
- `infra-wait`: DB readiness 확인
- `container-test-python`: Python 테스트를 전용 이미지에서 실행
- `container-test-rust`: Rust 테스트를 전용 이미지와 컨테이너 DB에서 실행
- `container-test`: Python + Rust 테스트 일괄 실행
- `container-backend-run`: Rust 백엔드를 컨테이너에서 실행

## Compose 사용 경로

저장소 루트의 [../../docker-compose.yml](../../docker-compose.yml) 은 `docker compose` 사용자를 위한 기준 구성을 유지한다.

예시:

```bash
docker compose up -d postgres
docker compose --profile test run --rm python-test
docker compose --profile test run --rm backend-test
docker compose --profile dev up backend-dev
```

단, 현재 환경에 따라 `compose` 플러그인이 없을 수 있으므로, 그런 경우에는 `Makefile` 의 `container-*` 대상 사용을 우선 권장한다.

## 검증 기준

- Python 테스트: `tests/test_health.py` 가 통과해야 한다.
- Rust 테스트: `backend/tests/http.rs` 같은 인메모리 기반 테스트와, `ALM_BACKEND_TEST_DATABASE_ADMIN_URL` 을 사용하는 PostgreSQL 연동 테스트가 모두 실행 가능해야 한다.
- 백엔드 실행: `GET /api/v1/health` 가 `200` 을 반환해야 한다.

## 호스트별 환경 기록

실제 설치 상태와 검증 결과는 공통 가이드와 분리해 호스트별 폴더에 기록한다.

- 환경 기록 인덱스: [./environments/README.md](./environments/README.md)
- 현재 확인된 호스트 기록: [./environments/bazzite-192.168.0.122/README.md](./environments/bazzite-192.168.0.122/README.md)
- 출처 불명 레거시 기록: [./environments/env_old/README.md](./environments/env_old/README.md)

## 다음에 읽을 문서

- 운영 위키: [README.md](./README.md)
- 작업 백로그 인덱스: [work_backlog.md](./work_backlog.md)
