# 참조 정합성 점검 배치 및 오류 큐 초안

- 문서 목적: 다형 참조와 간접 검증이 필요한 영역에 대해 정합성 점검 배치와 오류 큐 운영 모델을 정의한다.
- 범위: 참조 정합성 점검 배치, 오류 큐, 재처리 흐름, 운영 지표, 최소 엔터티 초안
- 대상 독자: 아키텍트, 개발자, 운영자, 데이터 모델러
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/polymorphic_reference_validation_draft.md`, `docs/architecture/initial_release_ddl_draft.md`, `docs/architecture/application_and_governance_architecture_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 다형 참조 무결성 검증 초안: [./polymorphic_reference_validation_draft.md](./polymorphic_reference_validation_draft.md)
- 초기 릴리스 `DDL` 초안: [./initial_release_ddl_draft.md](./initial_release_ddl_draft.md)

## 1. 목적

초기 릴리스에서는 일부 참조를 정규 외래키로 강제하지 않기 때문에, 저장 시점 검증만으로는 모든 정합성 문제를 막기 어렵다. 본 문서는 이 공백을 메우기 위해 야간 또는 주기 배치 기반의 참조 정합성 점검과 오류 큐 처리 구조를 정의한다. 목표는 운영자가 오류를 인지하고, 원인을 구분하고, 재처리 여부를 통제할 수 있게 하는 것이다.

## 2. 적용 대상

### 2.1 다형 참조 점검 대상

- `work_item_plan_link.plan_type + plan_id`
- `role_assignment.subject_type + subject_id`
- `audit_log.actor_type + actor_id`
- `audit_log.target_entity_type + target_entity_id`

### 2.2 간접 규칙 점검 대상

- `work_item_plan_link` 와 `planning_scheme` 규칙 일치 여부
- `role_assignment.role_type` 과 대상 유형 허용 조합 일치 여부
- `project_process_model.is_primary` 단일성 위반 여부
- `work_item.current_*` 와 `work_item_status_history` 최신 이력 불일치 여부

## 3. 운영 시나리오

### 3.1 배치 점검 시나리오

1. 배치가 점검 대상 테이블의 변경분 또는 전체 범위를 읽는다.
2. 참조 대상 존재 여부를 확인한다.
3. 상위 컨텍스트 일치 여부를 확인한다.
4. 정책/규칙 충돌 여부를 확인한다.
5. 오류를 유형별로 분류해 결과를 적재한다.
6. 임계치 초과 시 운영 알림을 발생시킨다.

### 3.2 오류 큐 시나리오

1. 실시간 저장 또는 배치 점검 중 오류가 감지된다.
2. 오류를 정규화된 레코드로 오류 큐에 적재한다.
3. 운영자는 오류를 확인하고 `open`, `acknowledged`, `resolved`, `ignored` 상태로 관리한다.
4. 재처리 가능 오류는 수동 또는 자동 재시도를 수행한다.
5. 재처리 성공 시 오류 상태를 `resolved` 로 전환하고 결과를 이력으로 남긴다.

## 4. 운영 구성 요소 초안

### 4.1 `reference_integrity_check_job`

- 목적: 정합성 점검 배치 정의
- 역할:
  - 점검 대상 범위 정의
  - 스케줄 관리
  - 활성 여부 관리
- 핵심 속성 예시:
  - `check_job_id`
  - `job_code`
  - `job_name`
  - `check_scope_type`
  - `schedule_expression`
  - `is_active`

### 4.2 `reference_integrity_check_run`

- 목적: 정합성 점검 실행 이력
- 역할:
  - 실행 상태 기록
  - 점검 건수와 오류 건수 요약
  - 재실행 기준 제공
- 핵심 속성 예시:
  - `check_run_id`
  - `check_job_id`
  - `started_at`
  - `ended_at`
  - `run_status`
  - `checked_count`
  - `issue_count`

### 4.3 `reference_integrity_issue`

- 목적: 참조 정합성 문제 저장
- 역할:
  - 오류 큐의 기본 레코드
  - 원인, 대상, 심각도, 현재 상태 관리
- 핵심 속성 예시:
  - `reference_integrity_issue_id`
  - `issue_type`
  - `severity`
  - `source_entity_type`
  - `source_entity_id`
  - `reference_type`
  - `reference_code`
  - `issue_status`
  - `detected_at`
  - `last_checked_at`

### 4.4 `reference_integrity_issue_action`

- 목적: 운영자 조치 이력 저장
- 역할:
  - 확인, 무시, 재처리 요청, 해결 조치 이력 관리
- 핵심 속성 예시:
  - `issue_action_id`
  - `reference_integrity_issue_id`
  - `action_type`
  - `action_by`
  - `action_note`
  - `action_at`

### 4.5 `reference_retry_queue`

- 목적: 재처리 대상 큐 저장
- 역할:
  - 재시도 대기
  - 재시도 횟수 관리
  - 마지막 오류 메시지 보관
- 핵심 속성 예시:
  - `reference_retry_queue_id`
  - `reference_integrity_issue_id`
  - `retry_type`
  - `retry_status`
  - `retry_count`
  - `next_retry_at`
  - `last_error_message`

## 5. 오류 유형 분류 초안

### 5.1 `issue_type`

- `missing_target_reference`
- `invalid_reference_type`
- `cross_project_reference_mismatch`
- `policy_rule_violation`
- `duplicate_primary_assignment`
- `overlapping_effective_period`
- `unresolvable_audit_actor`
- `unresolvable_audit_target`
- `status_history_mismatch`

### 5.2 `severity`

- `critical`: 업무 흐름 또는 승인 통제에 직접 영향
- `high`: 운영자 즉시 확인 필요
- `medium`: 배치 재처리 또는 후속 조치 필요
- `low`: 기록성 이슈, 추세 감시 대상

## 6. 권장 상태 모델

### 6.1 `reference_integrity_issue.issue_status`

- `open`
- `acknowledged`
- `retry_scheduled`
- `resolved`
- `ignored`

### 6.2 `reference_retry_queue.retry_status`

- `queued`
- `running`
- `succeeded`
- `failed`
- `cancelled`

## 7. 점검 규칙 예시

### 7.1 `work_item_plan_link`

- `plan_type` 이 허용 코드값인지 확인
- `plan_id` 대상이 실제 존재하는지 확인
- `work_item.project_id` 와 계획 단위 `project_id` 가 일치하는지 확인
- `planning_scheme` 상 허용되지 않은 연결 조합인지 확인
- 동일 `work_item + plan_type + link_role` 에서 대표 연결 중복이 있는지 확인

### 7.2 `role_assignment`

- `subject_type` 이 허용 코드값인지 확인
- 대상 엔터티가 존재하는지 확인
- `role_type` 과 `subject_type` 조합이 허용되는지 확인
- 유효기간이 겹치는 동일 역할 배정이 있는지 확인

### 7.3 `audit_log`

- `actor_type` 이 허용 코드값인지 확인
- `target_entity_type` 이 허용 코드값인지 확인
- 이벤트 정의상 필수인데 참조가 누락됐는지 확인
- 해석 가능한 대상인데 현재 기준에서 대상을 찾을 수 없는지 확인

## 8. 운영 지표 초안

- 일별 신규 정합성 오류 건수
- 미해결 오류 건수
- 오류 유형별 분포
- 평균 해결 시간
- 자동 재처리 성공률
- 동일 오류 재발률

## 9. 초기 릴리스 권장 운영안

- 야간 1회 전체 점검 배치 실행
- 저장 시점 주요 오류는 즉시 실패 처리
- 비동기 보완 대상은 오류 큐로 적재
- `critical`, `high` 오류는 운영 알림 연계
- `resolved`, `ignored` 전환 시 운영자 조치 이력 필수

## 10. 후속 상세화 후보

- 오류 큐 엔터티를 `DDL` 초안으로 하향
- 운영 대시보드 조회 모델 초안 작성
- 재처리 정책과 백오프 규칙 문서화
- 알림 연계 채널과 임계치 정책 문서화
