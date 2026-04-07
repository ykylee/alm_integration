# 다형 참조 무결성 검증 초안

- 문서 목적: 초기 릴리스에서 사용하는 다형 참조의 무결성 검증 책임과 처리 방식을 정리한다.
- 범위: `work_item_plan_link`, `role_assignment`, `audit_log` 의 다형 참조 저장 규칙, 검증 흐름, 오류 처리, 운영 보완 방안
- 대상 독자: 아키텍트, 개발자, 데이터 모델러, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/logical_reference_rules_draft.md`, `docs/architecture/initial_release_physical_model_draft.md`, `docs/architecture/initial_release_ddl_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 논리 참조 규칙 초안: [./logical_reference_rules_draft.md](./logical_reference_rules_draft.md)
- 초기 릴리스 `DDL` 초안: [./initial_release_ddl_draft.md](./initial_release_ddl_draft.md)

## 1. 목적

초기 릴리스 모델에는 정규 외래키로 닫지 않는 다형 참조가 포함된다. 이 구조는 프로세스 모델 확장성과 공통 저장 모델 유지에는 유리하지만, 반대로 저장 시점과 변경 시점 무결성 관리가 약해질 수 있다. 본 문서는 다형 참조를 어디서, 어떤 수준으로 검증할지 기준을 정해 이후 구현과 운영이 흔들리지 않게 하는 것을 목적으로 한다.

## 2. 대상 다형 참조

### 2.1 `work_item_plan_link`

- 참조 구조: `plan_type + plan_id`
- 대상 후보:
  - `iteration`
  - `release`
  - `milestone`
  - 후속 확장 시 `wbs_node`
- 핵심 리스크:
  - 존재하지 않는 계획 단위 연결
  - 허용되지 않은 `plan_type` 입력
  - 프로젝트가 다른 계획 단위와의 오연결
  - `planning_scheme` 규칙에 맞지 않는 다중 연결

### 2.2 `role_assignment`

- 참조 구조: `subject_type + subject_id`
- 대상 후보:
  - `organization_master`
  - `project`
  - `work_item`
- 핵심 리스크:
  - 대상 엔터티 미존재
  - 역할과 대상 유형 불일치
  - 유효기간이 겹치는 중복 배정
  - 조직/프로젝트/업무 범위 정책과 충돌

### 2.3 `audit_log`

- 참조 구조:
  - `actor_type + actor_id`
  - `target_entity_type + target_entity_id`
- 대상 후보:
  - 행위자: `workforce_master`, 시스템 계정, 연계 프로세스
  - 대상: `project`, `work_item`, 운영 엔터티
- 핵심 리스크:
  - 감사 이벤트는 남았지만 참조 대상이 해석되지 않음
  - 시스템/배치 주체를 잘못 업무 사용자로 가정
  - 삭제 또는 상태 변경 후 참조 일관성 약화

## 3. 검증 책임 분리 원칙

### 3.1 저장 계층 책임

- 허용 코드값 검증
- 필수 컬럼 존재 검증
- 대표 인덱스 제공
- 최소 중복 방지 제약

저장 계층에서는 일반 `FK` 수준 무결성을 강제하지 않고, 구조적으로 명백한 오류만 차단한다.

### 3.2 응용 서비스 책임

- 대상 엔터티 존재 여부 확인
- 대상과 상위 컨텍스트 일치 여부 확인
- 역할/계획 규칙 적합성 검증
- 기간 중복 또는 다중 대표 연결 충돌 검증

실제 업무 무결성은 응용 서비스가 1차 책임을 가진다.

### 3.3 운영 보완 책임

- 정합성 점검 배치
- 오류 이벤트 적재 및 재처리
- 감사 로그 해석 실패 모니터링

실시간 검증으로 모두 막기 어려운 경우를 대비해 운영 계층에서 정기 점검을 수행한다.

## 4. 저장 시점 검증 기준

### 4.1 `work_item_plan_link`

- `plan_type` 은 허용 코드값 집합과 일치해야 한다.
- `work_item_id` 는 반드시 존재해야 한다.
- `plan_id` 는 빈 값이면 안 된다.
- 동일 `work_item` 에 대해 `(plan_type, plan_id, link_role)` 중복은 허용하지 않는다.
- `is_primary=true` 인 연결은 동일 `work_item + plan_type + link_role` 기준 1건만 허용하는 방향을 권장한다.

### 4.2 `role_assignment`

- `subject_type` 은 허용 코드값 집합과 일치해야 한다.
- `assignee_workforce_id` 는 반드시 존재해야 한다.
- `role_type` 은 허용 코드값 집합과 일치해야 한다.
- 동일 `(subject_type, subject_id, role_type, assignee_workforce_id, effective_from)` 중복은 허용하지 않는다.
- 동일 기간 중 충돌 여부는 저장 제약보다 서비스 검증을 우선한다.

### 4.3 `audit_log`

- `actor_type`, `target_entity_type` 은 허용 코드값 집합과 일치해야 한다.
- `event_type`, `occurred_at` 는 필수다.
- `actor_id`, `target_entity_id` 는 일부 이벤트에서 null 허용 가능하지만, 그 기준은 이벤트 정의에서 따로 관리해야 한다.

## 5. 서비스 계층 검증 흐름

### 5.1 `work_item_plan_link`

1. `work_item` 존재 확인
2. `plan_type` 에 맞는 대상 테이블 결정
3. 대상 엔터티 존재 확인
4. `work_item.project_id` 와 계획 단위의 `project_id` 일치 여부 확인
5. `project_process_model -> planning_scheme` 규칙에 따라 허용 연결인지 검증
6. 대표 연결 중복 여부 확인
7. 저장

### 5.2 `role_assignment`

1. `assignee_workforce_id` 존재 확인
2. `subject_type` 에 맞는 대상 테이블 결정
3. 대상 엔터티 존재 확인
4. `role_type` 과 대상 유형 조합 허용 여부 확인
5. 같은 대상/역할의 유효기간 중복 여부 확인
6. 권한 정책과 조직 범위 규칙 충돌 여부 확인
7. 저장

### 5.3 `audit_log`

1. 이벤트 유형 정의 확인
2. `actor_type` 해석 경로 결정
3. `target_entity_type` 해석 경로 결정
4. 이벤트 정의상 필수인 참조값이 누락되지 않았는지 확인
5. 저장

`audit_log` 는 업무 참조보다 “기록 보존”이 우선이므로, 해석 실패 가능성이 있더라도 이벤트 자체는 남기고 오류 플래그를 함께 적재하는 방향이 더 적절하다.

## 6. 권장 오류 처리 방식

### 6.1 동기 오류

- 허용되지 않은 코드값
- 대상 엔터티 미존재
- 상위 컨텍스트 불일치
- 기간 중복 또는 대표 연결 충돌

위 경우는 요청 시점에 즉시 실패 처리한다.

### 6.2 비동기 보완

- 연계 지연으로 대상 엔터티가 아직 적재되지 않은 경우
- 삭제 또는 상태 변경으로 과거 참조가 현재 기준과 달라진 경우
- 감사 이벤트 해석 경로가 후속 데이터 적재에 의존하는 경우

위 경우는 오류 큐, 정합성 점검 배치, 재처리 절차로 보완한다.

## 7. 운영 점검 항목

- `work_item_plan_link` 에서 `plan_type` 별 대상 미존재 건수
- `role_assignment` 에서 허용되지 않은 `subject_type + role_type` 조합 건수
- `role_assignment` 에서 유효기간 중복 건수
- `audit_log` 에서 해석 불가 `actor_type/actor_id` 건수
- `audit_log` 에서 해석 불가 `target_entity_type/target_entity_id` 건수

이 점검 결과는 운영 대시보드 또는 배치 리포트로 노출하는 편이 적절하다.

## 8. 구현 권장안

### 8.1 초기 릴리스 권장안

- 저장 계층:
  - `CHECK` 로 코드값 검증
  - 유니크 인덱스로 명백한 중복 차단
  - 다형 참조용 복합 인덱스 생성
- 응용 서비스:
  - 대상 존재/프로젝트 일치/정책 적합성 검증
  - 오류 메시지와 원인 코드 표준화
- 운영 계층:
  - 야간 정합성 점검 배치
  - 오류 건 재처리 절차

### 8.2 후속 고도화 후보

- 다형 참조 검증용 공통 서비스 컴포넌트
- 대상 유형별 해석기를 등록하는 전략 패턴
- 정합성 점검 결과를 저장하는 별도 `reference_integrity_issue` 엔터티
- 감사 로그 이벤트 타입 카탈로그

## 9. 다음 작업 후보

- 다형 참조 검증 로직을 서비스 설계 초안으로 분리
- `role_type` 과 `role_policy` 매핑 규칙을 상세화
- 정합성 점검 배치와 오류 큐 모델 초안 작성
