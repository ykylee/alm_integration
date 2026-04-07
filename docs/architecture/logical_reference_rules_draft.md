# 논리 참조 규칙 초안

- 문서 목적: 초기 아키텍처 초안에서 직접 참조, 다형 참조, 간접 상속 관계를 어떻게 표현하고 해석할지 기준을 정리한다.
- 범위: 핵심 엔터티 간 참조 방식, `ERD` 표현 원칙, 구현 시 유의사항
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, 기획자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/domain_entity_definition_draft.md`, `docs/architecture/initial_release_erd_draft.md`, `docs/architecture/architecture_drafting_plan.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 초기 릴리스 `ERD`: [./initial_release_erd_draft.md](./initial_release_erd_draft.md)

## 1. 목적

현재 초기 릴리스 `ERD` 는 물리 `FK` 설계가 아니라 논리 모델 초안에 가깝다. 이 단계에서는 모든 관계를 단순한 직접 참조로 그리면 다형 참조와 간접 상속 구조가 왜곡되기 쉽다. 본 문서는 참조 유형을 구분하고, 각 유형을 문서와 `ERD` 에서 어떻게 다룰지 기준을 고정하기 위해 작성한다.

## 2. 참조 유형 분류

### 2.1 직접 참조

- 정의: 엔터티가 다른 엔터티의 식별자를 명시적으로 가진다.
- 예시:
  - `work_item.project_id -> project.project_id`
  - `project_process_model.workflow_scheme_id -> workflow_scheme.workflow_scheme_id`
  - `permission_scope.role_policy_id -> role_policy.role_policy_id`
- `ERD` 표현 원칙:
  - 직접 관계선을 사용한다.
  - 가능하면 엔터티 속성에도 `FK` 성격을 함께 드러낸다.

### 2.2 다형 참조

- 정의: 엔터티가 `type + id` 조합으로 여러 종류의 대상 중 하나를 참조한다.
- 예시:
  - `work_item_plan_link.plan_type + plan_id`
  - `role_assignment.subject_type + subject_id`
  - `audit_log.actor_type + actor_id`
  - `audit_log.target_entity_type + target_entity_id`
- `ERD` 표현 원칙:
  - 특정 엔터티로 가는 직접 관계선을 강하게 그리지 않는다.
  - 속성으로 참조 구조를 표시하고, 설명 절에서 대상 후보 집합을 문서화한다.
  - 필요한 경우 점선 관계 또는 보조 메모로만 표현한다.

### 2.3 간접 상속 참조

- 정의: 엔터티가 다른 엔터티를 직접 참조하지 않지만, 상위 설정 엔터티를 통해 규칙이나 기본값을 상속한다.
- 예시:
  - `work_item` 이 `project_process_model` 을 통해 `workflow_scheme`, `planning_scheme` 을 상속
  - `iteration`, `release`, `milestone` 이 프로젝트의 `project_process_model -> planning_scheme` 규칙을 따름
- `ERD` 표현 원칙:
  - 직접 소속 관계선으로 단순화하지 않는다.
  - 상속 출발점이 되는 엔터티를 중심으로 설명 메모를 둔다.
  - 구현 규칙은 별도 설명 절이나 제약 메모에서 정의한다.

## 3. 현재 초안에 적용하는 기준

### 3.1 `work_item_plan_link`

- 참조 유형: 다형 참조
- 참조 대상 후보:
  - `iteration`
  - `release`
  - `milestone`
  - 후속 확장 시 `wbs_node`
- 해석 기준:
  - 저장 모델에는 `plan_type + plan_id` 와 최소 제약만 둔다.
  - 화면 표현은 읽기 모델에서 재구성한다.

### 3.2 `role_assignment`

- 참조 유형: 다형 참조
- 참조 대상 후보:
  - `organization_master`
  - `project`
  - `work_item`
- 해석 기준:
  - 역할 자체는 `role_type` 로 관리한다.
  - `role_policy` 와의 직접 `FK` 는 아직 확정하지 않는다.
  - 향후 정책 모델이 성숙하면 `role_type` 과 `role_policy` 의 연결 구조를 상세화한다.

### 3.3 `audit_log`

- 참조 유형: 다형 참조
- 행위자 후보:
  - `workforce_master`
  - 시스템 계정
  - 배치/연계 프로세스
- 대상 후보:
  - `project`
  - `work_item`
  - 운영 엔터티 전반
- 해석 기준:
  - 감사 로그는 특정 업무 엔터티에 종속되지 않는다.
  - `ERD` 에서는 직접 관계선을 최소화하고, 다형 참조 규칙을 설명으로 보완한다.

### 3.4 `project_process_model`

- 참조 유형: 직접 참조 + 간접 상속
- 해석 기준:
  - `project`, `process_model_definition`, `workflow_scheme`, `planning_scheme`, `view_scheme` 와는 직접 참조
  - `work_item` 과 계획 단위는 이 설정을 기본 상속
  - 동일 시점 기본 활성 모델은 프로젝트당 하나만 허용

### 3.5 `project` 의 기본 프로세스 모델 해석

- 참조 유형: 간접 상속 기준
- 해석 기준:
  - `project` 본체에는 기본 프로세스 모델 식별자를 별도로 두지 않는다.
  - 프로젝트의 기본 프로세스 모델은 `project_process_model.is_primary=true` 레코드로 해석한다.
  - `workflow_scheme`, `planning_scheme`, `view_scheme` 도 같은 레코드에서 함께 해석한다.
  - 동일 시점 프로젝트당 기본 활성 레코드는 하나만 허용한다.

## 4. 코드값 표

### 4.1 `work_item_plan_link.plan_type`

| 코드값 | 참조 대상 | 설명 |
| --- | --- | --- |
| `iteration` | `iteration.iteration_id` | 반복 단위 연결 |
| `release` | `release.release_id` | 릴리스 범위 연결 |
| `milestone` | `milestone.milestone_id` | 일정 기준점 연결 |
| `wbs_node` | `wbs_node.wbs_node_id` | 후속 확장 시 단계형 계획 연결 |

### 4.2 `role_assignment.role_type`

| 코드값 | 의미 | 대표 연결 대상 |
| --- | --- | --- |
| `organization_head` | 조직장 역할 | `organization_master` |
| `approver` | 승인권자 역할 | `organization_master`, `project` |
| `delegate_approver` | 대행 승인자 역할 | `organization_master`, `project` |
| `project_owner` | 프로젝트 책임자 역할 | `project` |
| `work_item_owner` | 업무 항목 책임자 역할 | `work_item` |
| `quality_reviewer` | 품질 검토 책임 역할 | `project`, `work_item` |

## 5. `ERD` 표현 규칙

- 직접 참조만 실선 관계로 표현한다.
- 다형 참조는 속성과 보조 메모로 표현하고, 실선 관계는 생략한다.
- 간접 상속은 상위 설정 엔터티와의 관계만 표현하고, 상속 대상과의 직접 관계는 생략한다.
- 하나의 다이어그램에 서로 다른 성격의 관계를 모두 넣어 오해를 만들기보다, 필요하면 별도 보조 다이어그램으로 분리한다.

## 6. 구현 시 유의사항

- 다형 참조는 애플리케이션 레벨 검증 또는 제약 규칙 테이블이 필요할 수 있다.
- 간접 상속 규칙은 조회 성능을 위해 읽기 모델이나 캐시된 해석 결과가 필요할 수 있다.
- `ERD` 와 엔터티 정의 문서가 다를 경우, 현재 단계에서는 엔터티 정의 문서를 우선 기준으로 본다.

## 7. 다음 작업 후보

- `ERD` 를 직접 참조/다형 참조/간접 상속 표기 기준으로 한 번 더 보정
- 다형 참조 대상 후보를 코드값 표로 분리
- `role_type` 과 `role_policy` 연결 방식을 상세화
