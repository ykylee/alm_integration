# 통합 중앙 관리 시스템 아키텍처 초안

- 문서 목적: `CRS`, `SRS`, 역할 기반 UX를 구현 가능한 시스템 구조로 연결하기 위한 상위 아키텍처 초안을 제시한다.
- 범위: Phase 1 통합 관리 플랫폼 아키텍처의 상위 요약, 문서 분리 기준, 세부 초안 문서 연결
- 대상 독자: 아키텍트, 기획자, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-06
- 관련 문서: `docs/architecture/architecture_drafting_plan.md`, `docs/requirements/system_srs.md`, `docs/overview/role_based_ux_direction.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 계획 문서: [./architecture_drafting_plan.md](./architecture_drafting_plan.md)
- 시스템 컨텍스트 및 연계: [./system_context_and_integration_draft.md](./system_context_and_integration_draft.md)
- 핵심 도메인 모델: [./domain_model_draft.md](./domain_model_draft.md)
- 응용 서비스 및 운영 거버넌스: [./application_and_governance_architecture_draft.md](./application_and_governance_architecture_draft.md)

## 1. 초안 목적

본 문서는 구현 기술 선택 이전에 시스템의 큰 경계와 책임 분리를 먼저 고정하기 위한 초안이다. 현재 단계에서는 컴포넌트 책임, 주요 데이터 흐름, 외부 시스템 연계 방식, 운영/관리 경계의 일관성을 확보하는 데 목적이 있다.

## 2. 설계 입력

아키텍처 초안은 다음 입력을 기준으로 작성한다.

- [project_overview.md](../overview/project_overview.md): 제품 목표와 단계별 방향
- [system_crs.md](../requirements/system_crs.md): 상위 요구와 범위
- [system_srs.md](../requirements/system_srs.md): 기능, 비기능, 운영, 연계 요구
- [role_based_ux_direction.md](../overview/role_based_ux_direction.md): 역할별 화면과 운영 경계
- [ui_ux_prototype.md](../overview/ui_ux_prototype.md): 현재 프로토타입 구조

## 3. 우선 정의할 아키텍처 질문

- 과제 중심 모델을 어떤 핵심 도메인과 서비스로 나눌 것인가
- 외부 연계 수집 계층과 내부 조회/운영 계층을 어떻게 분리할 것인가
- 조직/인력 원천 반영과 내부 운영 마스터를 어떤 데이터 경계로 나눌 것인가
- 역할 기반 홈, 품질 작업면, 관리자 콘솔을 어떤 응용 서비스 경계로 연결할 것인가
- `AI` 리뷰, `AI` 테스트, `AI` `CI` 초안 기능을 어떤 보조 서비스 계층에 둘 것인가

## 4. 문서 분리 기준

아키텍처 초안은 다음 기준으로 분리한다.

1. 시스템 경계, 외부 연계, 논리 컴포넌트, 핵심 흐름은 [system_context_and_integration_draft.md](./system_context_and_integration_draft.md)에서 관리한다.
2. 도메인 경계와 주요 데이터 책임은 [domain_model_draft.md](./domain_model_draft.md)에서 관리한다.
3. 역할 기반 응용 서비스 경계와 관리자 운영/감사 구조는 [application_and_governance_architecture_draft.md](./application_and_governance_architecture_draft.md)에서 관리한다.
4. 본 문서는 전체 구조 요약과 문서 간 연결 기준을 유지한다.

## 5. 현재 상태

- 본 문서는 아키텍처 초안의 상위 요약 문서다.
- 세부 내용은 주제별 분리 문서로 이동했다.
- 다음 업데이트에서는 분리 문서 간 중복을 줄이고, 상세 설계 후보를 독립 문서로 확장한다.

## 6. 아키텍처 원칙 초안

- 과제 중심 도메인을 최상위 업무 경계로 둔다.
- 외부 도구는 실행과 원본 저장을 담당하고, 내부 시스템은 연결, 정규화, 조회, 운영 통제를 담당한다.
- 일반 업무면과 관리자 운영면은 화면뿐 아니라 응용 서비스 경계에서도 분리 가능해야 한다.
- 조직/인력 원천 데이터와 내부 운영 마스터는 동일시하지 않고 매핑 계층을 둔다.
- `AI` 기능은 독립 실행보다는 리뷰, 테스트, `CI` 설계 보조 서비스로 배치하고 모든 결과는 감사 가능해야 한다.

## 7. 분리 문서별 역할

- [system_context_and_integration_draft.md](./system_context_and_integration_draft.md): 시스템 경계, 외부 연계 범위, 논리 컴포넌트, 핵심 데이터 흐름
- [domain_model_draft.md](./domain_model_draft.md): 핵심 도메인 경계와 데이터 책임
- [application_and_governance_architecture_draft.md](./application_and_governance_architecture_draft.md): 역할 기반 응용 서비스 경계, 운영/감사 구조, 예외 처리 원칙

세 문서는 서로 연결되지만 중복 서술을 줄이기 위해 책임을 분리한다.

## 8. 다음 설계 상세화 제안

- 먼저 시스템 컨텍스트 다이어그램과 논리 컴포넌트 다이어그램을 문서화한다.
- 그 다음 핵심 데이터 엔터티와 식별자/매핑 규칙을 별도 문서로 분리한다.
- 이후 연계 아키텍처, 권한/감사 아키텍처, 배포/운영 구조 문서를 독립적으로 상세화한다.

## 9. 다음 상세 설계 후보

- 시스템 컨텍스트 다이어그램 문서
- 논리 컴포넌트 분해 문서
- 핵심 데이터 엔터티 초안
- 연계 아키텍처 초안
- 권한/감사 아키텍처 초안
- 배포 및 운영 구조 초안
