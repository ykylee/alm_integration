# bazzite-192.168.0.122 환경 기록

- 문서 목적: `bazzite / 192.168.0.122` 호스트에서 확인한 개발 및 테스트 환경 상태를 기록한다.
- 범위: 이 호스트에서 수행한 환경 점검, 설치 상태, 제약 사항, 재현 절차
- 대상 독자: 개발자, 운영자, 리뷰어
- 상태: draft
- 최종 수정일: 2026-04-16
- 관련 문서: `docs/operations/development_environment.md`, `docs/operations/backlog/2026-04-15.md`

## 호스트 식별 정보

- 호스트명: `bazzite`
- 주 IP: `192.168.0.122`
- 확인된 추가 IP:
  - `fde6:9e94:acc1:574e:89d8:7df3:57bf:8a64`
  - `fde6:9e94:acc1:574e:ab44:fc40:fdd:3dff`

## 2026-04-15 환경 점검 요약

- 사용 가능:
  - `python3 3.14.3`
  - `podman 5.8.1`
  - `pip`
- 사용 불가 또는 미설치:
  - `docker`
  - `cargo`
  - `pytest`
  - `podman compose`

## 이 호스트에서 정리한 저장소 진입점

- 컨테이너 런타임 지정형 `Makefile` 대상 추가
- `docker` 기준 `compose` 서비스 확장
- 컨테이너 전용 환경 파일 `.env.docker.example` 추가
- Python/Rust 테스트 전용 `Dockerfile` 추가

## 후속 검증 필요 항목

- `CONTAINER_RUNTIME=podman make infra-up`
- `CONTAINER_RUNTIME=podman make container-test-python`
- `CONTAINER_RUNTIME=podman make container-test-rust`
- 로컬 또는 컨테이너 기반 `cargo` 테스트 실제 통과 여부

## 2026-04-16 추가 확인

- 사용 가능:
  - `Homebrew 5.1.6`
  - `npm 11.12.1`
- 계속 사용 불가 또는 미설치:
  - `cargo`
  - `rustc`
  - `pytest`
  - `podman-compose`
  - `docker-compose`
- 차단 원인:
  - `brew install rust` 는 최초에 Homebrew 캐시 경로 쓰기 권한 문제로 실패했다.
  - 캐시 경로를 `/tmp` 로 바꾼 뒤에는 `formulae.brew.sh` DNS 해석 실패로 설치가 중단됐다.
  - `pip` 도 외부 인덱스 접속 시 이름 해석 실패로 `pytest` 설치가 불가능했다.

## 2026-04-16 07:28 KST 네트워크 재확인

- `formulae.brew.sh`, `pypi.org`, `files.pythonhosted.org`, `github.com` DNS 해석 정상
- `https://formulae.brew.sh/api/formula.jws.json` `HTTP/2 200`
- `https://pypi.org/simple/pytest/` `HTTP/2 200`
- `https://github.com` `HTTP/2 200`
- 해석:
  - 현재 세션 기준으로 외부 네트워크와 HTTPS 접근은 정상이다.
  - 직전 설치 실패는 일시적 네트워크 문제였거나 이전 세션 상태였을 가능성이 높다.

## 2026-04-16 도구 설치 및 검증 결과

- 설치 완료:
  - `cargo 1.94.1 (Homebrew)`
  - `rustc 1.94.1 (Homebrew)`
  - `pytest 8.4.2`
  - `podman-compose 1.5.0`
- 로컬 검증:
  - `python3 -m pytest` 통과
  - `set -a && source .env.example && set +a && cargo test --manifest-path backend/Cargo.toml` 통과
- 격리 검증:
  - `CONTAINER_RUNTIME=podman make container-test-rust` 통과
- 이 호스트에서 확인한 보정 사항:
  - `podman` 은 short-name 이미지 해석에 비대화형 제약이 있어 fully qualified image 사용이 안전하다.
  - SELinux 환경에서는 소스 bind mount 에 `:Z` 옵션이 필요하다.
  - Rust 컨테이너는 현재 저장소 의존성 기준으로 `1.94` 이상이 필요하다.
  - 로그인 셸 기반 `bash -lc` 보다 `cargo` 직접 실행이 컨테이너에서 안정적이다.

## 2026-04-16 UI 실행 및 렌더링 확인

- 서버 기동:
  - Rust 백엔드 `http://127.0.0.1:8080`
  - 정적 UI 서버 `http://127.0.0.1:8000`
- HTTP 확인:
  - `GET /api/v1/health` 응답 정상
  - `admin.html`, `organization.html` 정적 HTML `HTTP 200`
- 브라우저 확인:
  - Playwright Chromium 기준 `admin.html` 상태 칩 `연결 정상`
  - Playwright Chromium 기준 `organization.html` 상태 칩 `연결 정상`
  - 실패 문구 미검출
- 산출물:
  - `output/playwright/ui_prototype/runtime_check_2026-04-16/01-admin.png`
  - `output/playwright/ui_prototype/runtime_check_2026-04-16/02-organization.png`

## 다음에 읽을 문서

- [../../development_environment.md](../../development_environment.md)
- [../../backlog/2026-04-15.md](../../backlog/2026-04-15.md)
