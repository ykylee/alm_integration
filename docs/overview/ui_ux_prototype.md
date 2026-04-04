# 통합 중앙 관리 시스템 UI/UX 프로토타입

- 문서 목적: 현재 기안서와 요구사항을 바탕으로 빠르게 열람 가능한 UI/UX 프로토타입의 의도와 구성 화면을 설명한다.
- 범위: 정적 프로토타입의 화면 구성, 반영 기준, 열람 방법
- 대상 독자: 프로젝트 스폰서, 기획자, 디자이너, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-05
- 관련 문서: `docs/overview/project_proposal.md`, `docs/overview/project_execution_plan.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 개요 위키: [./README.md](./README.md)
- 관련 문서: [./project_proposal.md](./project_proposal.md)

## 1. 프로토타입 목적

이 프로토타입은 현재 기안서의 핵심 메시지를 화면 관점에서 빠르게 공유하기 위한 정적 데모다. 구현 기술이나 실제 데이터 구조를 확정하기 위한 산출물이 아니라, 다음 질문에 답하기 위한 시각 초안으로 본다.

- 과제 중심 운영이 실제 화면에서는 어떻게 보이는가
- `Jira`, `Bitbucket`, `Bamboo` 중심의 `Phase 1 MVP`는 어떤 화면 묶음으로 설명할 수 있는가
- 조직/인력 변경 검토와 공유 캘린더 요구가 메인 경험 안에 어떻게 들어오는가

## 2. 포함한 화면

- 통합 현황: 핵심 운영 상태와 `Phase 1 MVP` 범위를 한눈에 보여주는 메인 화면
- 과제 등록: 일반 과제와 관리 외 프로젝트를 구분하는 등록 경험
- 과제 상세: 요구, 변경, 빌드, 참여자, 연계 상태를 함께 보는 작업면
- 조직 변경 검토: `인사 DB -> 내부 데이터 -> Crowd` 변경 흐름을 검토하는 관리자 화면
- 공유 캘린더: 부재 일정과 코드리뷰 가용성 계산을 함께 보는 운영 화면

## 3. 열람 방법

프로토타입은 정적 파일로 구성되어 있으며 브라우저에서 바로 열 수 있다.

- 진입 파일: [../../src/ui_prototype/index.html](../../src/ui_prototype/index.html)
- 스타일 파일: [../../src/ui_prototype/styles.css](../../src/ui_prototype/styles.css)
- 스크립트 파일: [../../src/ui_prototype/app.js](../../src/ui_prototype/app.js)

로컬에서 간단히 확인하려면 저장소 루트에서 다음 명령으로 열면 된다.

```bash
python3 -m http.server
```

그 뒤 브라우저에서 `http://localhost:8000/src/ui_prototype/index.html` 로 접속하면 된다.

## 4. 반영 기준

- 과제가 프로젝트보다 앞에 오는 구조
- `Phase 1 MVP`의 1차 핵심 연계 `Jira`, `Bitbucket`, `Bamboo`
- 조직/인력 기준정보와 관리자 승인형 마이그레이션
- 공유 캘린더와 부재 기반 지표 보정
- 외부 도구 실행 계층은 유지하고 내부에서는 연결, 조회, 운영 경험을 강화하는 방향

## 5. 다음 보완 후보

- 화면별 상세 항목 정의 연결
- 모바일 대응 세부 조정
- 실제 데이터 모델과 연결되는 테이블/필드 초안 반영
- 승인본 기안서 기준 KPI와 경고 상태 반영
