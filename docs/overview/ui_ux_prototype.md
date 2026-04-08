# 통합 중앙 관리 시스템 UI/UX 프로토타입

- 문서 목적: 현재 기안서와 요구사항을 바탕으로 빠르게 열람 가능한 UI/UX 프로토타입의 의도와 구성 화면을 설명한다.
- 범위: 정적 프로토타입의 화면 구성, 반영 기준, 열람 방법
- 대상 독자: 프로젝트 스폰서, 기획자, 디자이너, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-08
- 관련 문서: `docs/overview/project_proposal.md`, `docs/overview/project_execution_plan.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 개요 위키: [./README.md](./README.md)
- 관련 문서: [./project_proposal.md](./project_proposal.md)
- 조직 관리 레퍼런스: [./organization_admin_ui_reference_research.md](./organization_admin_ui_reference_research.md)

## 1. 프로토타입 목적

이 프로토타입은 현재 기안서의 핵심 메시지와 역할 기반 UX 방향을 화면 관점에서 빠르게 공유하기 위한 정적 데모다. 구현 기술이나 실제 데이터 구조를 확정하기 위한 산출물이 아니라, 다음 질문에 답하기 위한 시각 초안으로 본다.

- 과제 중심 운영이 실제 화면에서는 어떻게 보이는가
- `Jira`, `Bitbucket`, `Bamboo` 중심의 `Phase 1 MVP`는 어떤 화면 묶음으로 설명할 수 있는가
- 역할별로 다른 우선순위와 권한 차이가 첫 화면과 작업면에 어떻게 반영되는가

2026-04-08 기준으로는 이 문서의 프로토타입이 최종 운영 UI 방향을 대표하지는 않는다. 특히 시스템 관리자용 데이터 관리 UI 는 별도 방향으로 재설계하기로 했으며, 그 기준은 [integrated_data_management_ui_direction.md](./integrated_data_management_ui_direction.md) 를 따른다.

## 2. 포함한 화면

- 역할 홈: 개발자, 프로젝트리더, 조직장, 시스템 관리자, `SE` 담당자, 테스트 담당자를 전환하며 첫 화면을 확인하는 메인 화면
- 과제 워크스페이스 상세: 요구사항, 코드리뷰, 빌드/배포, 품질 게이트, 결함, 참여자/캘린더를 별도 상세 영역으로 보여주는 페이지
- 프로젝트 운영 상세: 마일스톤, 위험 등록부, 승인 큐, 의존 관계, 보고 요약을 상세하게 보여주는 페이지
- 조직 운영 상세: 승인 대기, 조직 가용성, 조직 변경 영향, 조직 마스터, 인력 기준정보, 공유 캘린더, 과제 분포를 상세하게 보여주는 페이지
- 품질 검증 상세: `SE` 담당자용 환경/배포 검토와 테스트 담당자용 검증/결함 보드를 함께 보여주는 페이지
- 관리자 콘솔 상세: 연계 연결 설정, `sync-runs`, 조직/인력 기준정보 반영, `인사 DB -> 내부 데이터 -> Crowd` 흐름, 매핑 이상, 권한/감사 정보를 상세하게 보여주는 시스템 관리자 페이지
- 자체 `ALM` 데이터 관리: 내부 표준 `project`, `work_item` 를 별도 작업면에서 조회하는 페이지
- 외부 연결 시스템 데이터 관리: `Jira`, `Bitbucket`, `Bamboo`, `Confluence` 를 시스템별로 분리해 조회하는 페이지
- 조직 관리: 조직 마스터를 별도 작업면에서 조회하는 페이지
- 인력 관리: 조직별 인력 리스트를 별도 작업면에서 조회하는 페이지

2026-04-08 현재 통합 데이터 관리 UI 는 더 이상 별도 `센터 홈` 을 두지 않고, 각 하위 관리 화면으로 직접 진입하는 구조를 기준으로 삼는다. 하위 페이지들은 공통적으로 `목록 -> 대표 상세 -> 운영 액션 -> 영향 범위` 흐름을 가지도록 보강됐다. 아직 실제 편집 폼은 일부 화면에 한정되지만, 운영자가 판단에 필요한 컨텍스트를 같은 페이지 안에서 연속적으로 볼 수 있게 하는 것이 목표다.

2026-04-08 후속 구현 기준으로 `조직 관리`, `인력 관리` 화면에는 실제 관리자 API 와 연결되는 최소 액션 폼도 추가됐다. 현재 가능한 범위는 다음과 같다.

- 조직 등록/수정/삭제
- 상위 조직 지정과 다단계 계층 구성
- 계층 순환 방지
- 조직 구성원 등록
- 조직 구성원 조직 이동
- 조직 구성원 비활성화
- `사업부 -> 팀 -> 그룹 -> 파트` 구조의 더미 데이터 생성
- 조직 트리 시각화와 선택 조직 기준 인력 관리 연동
- 조직 관리 화면 내부에서 직속 구성원 등록/이동/비활성화 실행
- 선택 조직 기준 조직 변경 이력 조회
- 선택 조직 기준 구성원 이동 이력 조회
- 선택 조직 기준 상위 경로, 직속 하위 조직, 하위 포함 인원 규모 조회

조직 관리 화면은 현재 `목록 -> 트리 -> 구조 요약 -> 직속 구성원 -> 조직/구성원 액션 -> 변경 이력` 순서로 읽히도록 정리돼 있다. 즉 조직을 선택하면 같은 화면 안에서 상위 경로, 직속 하위 조직, 직속 인력, 변경 액션, 최근 변경 이력을 연속적으로 확인할 수 있다.

2026-04-08 후속 디자인 개편에서는 조직 관리 화면을 `Directory + Inspector + Action Rail` 구조로 재배치했다. 이 방향은 Atlassian 관리 콘솔의 목록 중심 그룹 관리, Microsoft Entra/Microsoft 365 관리자 화면의 좌측 탐색 + 중앙 상세 작업면, Okta 의 `joiner / mover / leaver` 운영 액션, IBM 데이터 거버넌스 화면의 관계/영향 패널 구성을 참고해 정리했다. 상세 레퍼런스는 [organization_admin_ui_reference_research.md](./organization_admin_ui_reference_research.md) 에 정리한다.

같은 개편 단계에서 데이터 관리 하위 화면 전체도 같은 톤으로 맞췄다. 현재 [data_workforce.html](../../src/ui_prototype/data_workforce.html), [data_alm.html](../../src/ui_prototype/data_alm.html), 외부 시스템 4개 화면은 모두 상단 명령 덱, 중앙 인스펙터, 우측 보조 패널 또는 액션 레일을 갖는 엔터프라이즈 관리 콘솔 문법으로 재구성되어 있다.

## 3. 열람 방법

프로토타입은 정적 파일로 구성되어 있으며 브라우저에서 바로 열 수 있다. 현재는 메뉴별 상세 페이지 구조를 사용한다.

- 진입 파일: [../../src/ui_prototype/index.html](../../src/ui_prototype/index.html)
- 과제 워크스페이스 상세: [../../src/ui_prototype/tasks.html](../../src/ui_prototype/tasks.html)
- 프로젝트 운영 상세: [../../src/ui_prototype/delivery.html](../../src/ui_prototype/delivery.html)
- 조직 운영 상세: [../../src/ui_prototype/organization.html](../../src/ui_prototype/organization.html)
- 품질 검증 상세: [../../src/ui_prototype/quality.html](../../src/ui_prototype/quality.html)
- 관리자 콘솔 상세: [../../src/ui_prototype/admin.html](../../src/ui_prototype/admin.html)
- 자체 `ALM` 데이터 관리: [../../src/ui_prototype/data_alm.html](../../src/ui_prototype/data_alm.html)
- 외부 `Jira` 데이터 관리: [../../src/ui_prototype/data_external_jira.html](../../src/ui_prototype/data_external_jira.html)
- 외부 `Bitbucket` 데이터 관리: [../../src/ui_prototype/data_external_bitbucket.html](../../src/ui_prototype/data_external_bitbucket.html)
- 외부 `Bamboo` 데이터 관리: [../../src/ui_prototype/data_external_bamboo.html](../../src/ui_prototype/data_external_bamboo.html)
- 외부 `Confluence` 데이터 관리: [../../src/ui_prototype/data_external_confluence.html](../../src/ui_prototype/data_external_confluence.html)
- 조직 관리: [../../src/ui_prototype/data_organizations.html](../../src/ui_prototype/data_organizations.html)
- 인력 관리: [../../src/ui_prototype/data_workforce.html](../../src/ui_prototype/data_workforce.html)
- 스타일 파일: [../../src/ui_prototype/styles.css](../../src/ui_prototype/styles.css)
- 스크립트 파일: [../../src/ui_prototype/app.js](../../src/ui_prototype/app.js)
- 렌더링 출력 경로: `output/playwright/ui_prototype/<version>/`

2026-04-08 기준 `organization.html`, `admin.html` 과 데이터 관리 하위 페이지들은 정적 설명 화면에 머물지 않고 실제 운영 API 와 연결할 수 있는 최소 바인딩을 갖는다. 기본 API 기준 URL 은 `http://127.0.0.1:8080/api/v1` 이며, 화면 상단 입력값 또는 `?apiBase=` 쿼리로 변경할 수 있다.

로컬에서 간단히 확인하려면 저장소 루트에서 다음 명령으로 열면 된다.

```bash
python3 -m http.server
```

그 뒤 브라우저에서 `http://localhost:8000/src/ui_prototype/index.html` 로 접속하면 된다.

실데이터를 함께 보려면 별도 터미널에서 Rust 백엔드를 먼저 실행한다.

```bash
cargo run --manifest-path backend/Cargo.toml
```

렌더 스크립트를 사용할 때는 버전 폴더를 구분해 저장한다. 기본 버전은 현재 메뉴별 상세 페이지 구조를 반영한 `v3_detail_pages` 다.

```bash
UI_PROTOTYPE_RENDER_VERSION=v3_detail_pages node src/ui_prototype/render_screenshots.js
```

## 4. 반영 기준

- 과제가 프로젝트보다 앞에 오는 구조
- 역할별 홈과 역할 전환 구조
- `Phase 1 MVP`의 1차 핵심 연계 `Jira`, `Bitbucket`, `Bamboo`
- 조직/인력 기준정보와 관리자 승인형 마이그레이션
- 백엔드 운영에 필요한 연계 설정, 동기화 실행, 기준정보 관리 흐름
- 공유 캘린더와 부재 기반 지표 보정
- 품질/검증과 관리자 기능의 별도 작업면 분리
- 외부 도구 실행 계층은 유지하고 내부에서는 연결, 조회, 운영 경험을 강화하는 방향

## 5. 다음 보완 후보

- 기존 관리자/조직 운영 프로토타입을 유지 보수하기보다, 시스템 관리자 전용 통합 데이터 관리 UI 를 별도 구조로 재설계
- 역할별 세부 권한에 따른 버튼/행동 가능 상태 반영
- 실제 운영 API 와 연결되는 조직/인사 기준정보 편집 흐름 반영
- `sync-runs`, `master-data`, `projects`, `work-items` 조회를 넘어 등록/수정/취소 액션까지 연결
- 모바일 대응 세부 조정
- 실제 데이터 모델과 연결되는 테이블/필드 초안 반영
- 승인본 기안서 기준 KPI와 경고 상태 반영
