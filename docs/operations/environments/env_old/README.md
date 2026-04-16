# env_old 환경 기록

- 문서 목적: 작업 호스트와 IP가 명확하지 않은 기존 환경 의존 기록을 별도 보관해 추적 기준을 분리한다.
- 범위: 기존 문서와 백로그에 남아 있는 비정형 환경 기록
- 대상 독자: 개발자, 운영자, 리뷰어
- 상태: draft
- 최종 수정일: 2026-04-15
- 관련 문서: `docs/operations/environments/README.md`, `docs/operations/development_environment.md`

## 설명

이 폴더는 과거 작업 기록 중 어느 호스트에서 수행했는지 명확하지 않은 환경 관련 이력을 모아두는 용도다. 기존 문서를 삭제하지는 않고, 참조 지점을 목록화해 이후 호스트별 구조로 점진적으로 이전한다.

## 현재 확인된 레거시 참조

- [../../backlog/2026-04-07.md](../../backlog/2026-04-07.md)
  - `TASK-115 로컬 개발 및 테스트 환경 구축`
  - `docker`, `docker compose`, `colima`, `cargo` 설치와 검증 이력이 있으나 호스트명과 IP가 기록돼 있지 않다.
- [../../../architecture/current_backend_implementation_status_summary.md](../../../architecture/current_backend_implementation_status_summary.md)
  - 구현 상태 표의 개발 환경 행에 과거 검증 완료 상태가 요약돼 있으나 작업 호스트가 명시돼 있지 않다.
- [../../backend_operation_ui_render_and_scenarios.md](../../backend_operation_ui_render_and_scenarios.md)
  - 로컬 재현 절차와 실제 확인 결과가 남아 있으나 당시 실행 호스트와 네트워크 정보가 명시돼 있지 않다.

## 정리 원칙

- 앞으로 새 환경 기록은 이 폴더에 추가하지 않는다.
- 후속 작업에서 출처가 확인되면 해당 항목은 적절한 `hostname-ip` 폴더로 옮기거나 링크를 대체한다.
