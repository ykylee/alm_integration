# 환경 기록 위키

- 문서 목적: 호스트별 개발 환경, 테스트 환경, 설치 이력, 환경 제약 사항을 분리해 추적할 수 있는 기준 구조를 제공한다.
- 범위: `docs/operations/environments/` 아래의 호스트별 환경 기록과 레거시 환경 기록
- 대상 독자: 개발자, 운영자, 리뷰어
- 상태: draft
- 최종 수정일: 2026-04-15
- 관련 문서: `docs/operations/development_environment.md`, `docs/operations/work_backlog.md`, `AGENTS.md`

## 운영 원칙

- 환경에 따라 달라지는 설치 상태, 패키지 버전, 런타임 가용성, 검증 결과는 호스트별 폴더에 기록한다.
- 폴더명은 `hostname-ip` 형식을 사용한다. IP는 기본적으로 확인 가능한 주 `IPv4` 를 사용한다.
- 백로그에는 항상 `호스트명`, `호스트 IP` 를 함께 적는다.
- 기존 문서 중 호스트와 IP가 명확하지 않은 환경 이력은 `env_old` 로 이동하거나 링크로 집계한다.

## 폴더 목록

- 레거시 환경 기록: [env_old](./env_old/README.md)
- 현재 호스트 기록: [bazzite-192.168.0.122](./bazzite-192.168.0.122/README.md)

## 다음에 읽을 문서

- 개발 환경 및 테스트 환경 가이드: [../development_environment.md](../development_environment.md)
- 작업 백로그 인덱스: [../work_backlog.md](../work_backlog.md)
