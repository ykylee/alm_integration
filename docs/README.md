# 문서 위키 홈

- 문서 목적: 저장소 문서를 wiki 형태로 탐색할 수 있는 홈과 문서 체계를 제공한다.
- 범위: `docs/` 아래 모든 문서
- 대상 독자: 기획자, 개발자, 운영자, 프로젝트 참여자
- 상태: draft
- 최종 수정일: 2026-04-04
- 관련 문서: `AGENTS.md`

## 문서 읽기 흐름

문서를 처음 읽는 경우 다음 순서를 기본 경로로 사용한다.

1. 프로젝트 배경과 방향 확인: `overview`
2. 컨셉 수준 요구 확인: `CRS`
3. 상세 요구 확인: `SRS`
4. 운영 규칙과 작업 이력 확인: `operations`

## 빠른 이동

- 프로젝트 개요: [overview/README.md](./overview/README.md)
- 요구사항 위키: [requirements/README.md](./requirements/README.md)
- 연계 위키: [integrations/README.md](./integrations/README.md)
- 운영 위키: [operations/README.md](./operations/README.md)
- 작업 백로그 인덱스: [operations/work_backlog.md](./operations/work_backlog.md)

## 카테고리 안내

- `overview/`: 프로젝트 목적, 배경, 목표, 범위, 이해관계자, 핵심 용어
- `requirements/`: `CRS`, `SRS`, 요구사항 초안, 상세 요구사항, 유스케이스, 우선순위
- `architecture/`: 시스템 구조, 컴포넌트, 데이터 흐름, 연계 설계
- `operations/`: 운영 정책, 배포 절차, 권한, 장애 대응, 작업 관리, 백로그
- `integrations/`: 외부 시스템별 연계 정책, API, 데이터 매핑
- `decisions/`: 설계 의사결정과 ADR

## 현재 핵심 문서

- 프로젝트 개요: [overview/project_overview.md](./overview/project_overview.md)
- 통합 중앙 관리 시스템 CRS: [requirements/system_crs.md](./requirements/system_crs.md)
- 통합 중앙 관리 시스템 SRS: [requirements/system_srs.md](./requirements/system_srs.md)
- SDLC 시스템 카테고리 정의: [integrations/sdlc_system_categories.md](./integrations/sdlc_system_categories.md)

## 운영 원칙

- 문서는 한국어로 작성하되 기술 식별자는 원문을 유지한다.
- 파일명은 `snake_case`를 사용한다.
- `docs/README.md` 를 wiki 홈으로 유지하고, 카테고리별 인덱스 문서를 통해 하위 문서로 이동할 수 있게 구성한다.
- 핵심 문서는 관련 문서와 다음 문서 링크를 유지해 탐색 경로가 끊기지 않게 한다.
- 확정되지 않은 내용은 `가정` 또는 `미정`으로 표시한다.
- 새로운 카테고리를 추가하면 `AGENTS.md`와 이 문서를 함께 갱신한다.
- 세션 시작 시에는 작업 전에 백로그 인덱스와 최근 날짜 백로그를 먼저 확인해 진행 중 작업과 후속 작업을 파악한다.
- 작업 시작 전 브리핑과 백로그 등록을 수행한다.
- 작업 중간 현황과 완료 결과는 날짜별 백로그 문서 `docs/operations/backlog/YYYY-MM-DD.md` 에 반영한다.
- `docs/operations/work_backlog.md` 는 날짜별 백로그 문서 인덱스와 운영 기준 문서로 사용한다.
- 과제 컨셉이 먼저 정리되는 작업은 `CRS`를 우선 작성하고, 이후 세분화한 내용을 `SRS`에 반영한다.
- 요구사항 문서가 재정리될 때는 초안 문서를 유지하기보다 `CRS`와 `SRS` 기준으로 역할을 분리해 중복을 줄인다.
