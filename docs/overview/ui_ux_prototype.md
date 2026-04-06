# 통합 중앙 관리 시스템 UI/UX 프로토타입

- 문서 목적: 현재 기안서와 요구사항을 바탕으로 빠르게 열람 가능한 UI/UX 프로토타입의 의도와 구성 화면을 설명한다.
- 범위: 정적 프로토타입의 화면 구성, 반영 기준, 열람 방법
- 대상 독자: 프로젝트 스폰서, 기획자, 디자이너, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-06
- 관련 문서: `docs/overview/project_proposal.md`, `docs/overview/project_execution_plan.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 개요 위키: [./README.md](./README.md)
- 관련 문서: [./project_proposal.md](./project_proposal.md)

## 1. 프로토타입 목적

이 프로토타입은 현재 기안서의 핵심 메시지와 역할 기반 UX 방향을 화면 관점에서 빠르게 공유하기 위한 정적 데모다. 구현 기술이나 실제 데이터 구조를 확정하기 위한 산출물이 아니라, 다음 질문에 답하기 위한 시각 초안으로 본다.

- 과제 중심 운영이 실제 화면에서는 어떻게 보이는가
- `Jira`, `Bitbucket`, `Bamboo` 중심의 `Phase 1 MVP`는 어떤 화면 묶음으로 설명할 수 있는가
- 역할별로 다른 우선순위와 권한 차이가 첫 화면과 작업면에 어떻게 반영되는가

## 2. 포함한 화면

- 역할 홈: 개발자, 프로젝트리더, 조직장, 시스템 관리자, `SE` 담당자, 테스트 담당자를 전환하며 첫 화면을 확인하는 메인 화면
- 과제 워크스페이스 상세: 요구사항, 코드리뷰, 빌드/배포, 품질 게이트, 결함, 참여자/캘린더를 별도 상세 영역으로 보여주는 페이지
- 프로젝트 운영 상세: 마일스톤, 위험 등록부, 승인 큐, 의존 관계, 보고 요약을 상세하게 보여주는 페이지
- 조직 운영 상세: 승인 대기, 조직 가용성, 조직 변경 영향, 공유 캘린더, 과제 분포를 상세하게 보여주는 페이지
- 품질 검증 상세: `SE` 담당자용 환경/배포 검토와 테스트 담당자용 검증/결함 보드를 함께 보여주는 페이지
- 관리자 콘솔 상세: `인사 DB -> 내부 데이터 -> Crowd` 흐름, 연계 상태, 매핑 이상, 권한/감사 정보를 상세하게 보여주는 시스템 관리자 페이지

## 3. 열람 방법

프로토타입은 정적 파일로 구성되어 있으며 브라우저에서 바로 열 수 있다. 현재는 메뉴별 상세 페이지 구조를 사용한다.

- 진입 파일: [../../src/ui_prototype/index.html](../../src/ui_prototype/index.html)
- 과제 워크스페이스 상세: [../../src/ui_prototype/tasks.html](../../src/ui_prototype/tasks.html)
- 프로젝트 운영 상세: [../../src/ui_prototype/delivery.html](../../src/ui_prototype/delivery.html)
- 조직 운영 상세: [../../src/ui_prototype/organization.html](../../src/ui_prototype/organization.html)
- 품질 검증 상세: [../../src/ui_prototype/quality.html](../../src/ui_prototype/quality.html)
- 관리자 콘솔 상세: [../../src/ui_prototype/admin.html](../../src/ui_prototype/admin.html)
- 스타일 파일: [../../src/ui_prototype/styles.css](../../src/ui_prototype/styles.css)
- 스크립트 파일: [../../src/ui_prototype/app.js](../../src/ui_prototype/app.js)
- 렌더링 출력 경로: `output/playwright/ui_prototype/<version>/`

로컬에서 간단히 확인하려면 저장소 루트에서 다음 명령으로 열면 된다.

```bash
python3 -m http.server
```

그 뒤 브라우저에서 `http://localhost:8000/src/ui_prototype/index.html` 로 접속하면 된다.

렌더 스크립트를 사용할 때는 버전 폴더를 구분해 저장한다. 기본 버전은 현재 메뉴별 상세 페이지 구조를 반영한 `v3_detail_pages` 다.

```bash
UI_PROTOTYPE_RENDER_VERSION=v3_detail_pages node src/ui_prototype/render_screenshots.js
```

## 4. 반영 기준

- 과제가 프로젝트보다 앞에 오는 구조
- 역할별 홈과 역할 전환 구조
- `Phase 1 MVP`의 1차 핵심 연계 `Jira`, `Bitbucket`, `Bamboo`
- 조직/인력 기준정보와 관리자 승인형 마이그레이션
- 공유 캘린더와 부재 기반 지표 보정
- 품질/검증과 관리자 기능의 별도 작업면 분리
- 외부 도구 실행 계층은 유지하고 내부에서는 연결, 조회, 운영 경험을 강화하는 방향

## 5. 다음 보완 후보

- 역할별 세부 권한에 따른 버튼/행동 가능 상태 반영
- 모바일 대응 세부 조정
- 실제 데이터 모델과 연결되는 테이블/필드 초안 반영
- 승인본 기안서 기준 KPI와 경고 상태 반영
