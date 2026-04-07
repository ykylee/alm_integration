# 참조 정합성 대시보드 조회 모델 초안

- 문서 목적: 참조 정합성 운영 대시보드에 필요한 조회 모델과 집계 기준을 정의한다.
- 범위: 대시보드 위젯, 필터, 집계 단위, 조회 모델 초안, 갱신 방식
- 대상 독자: 아키텍트, 개발자, 운영자, 기획자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/reference_integrity_batch_and_error_queue_draft.md`, `docs/architecture/reference_integrity_operations_ddl_draft.md`, `docs/overview/role_based_ux_direction.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 참조 정합성 점검 배치 및 오류 큐 초안: [./reference_integrity_batch_and_error_queue_draft.md](./reference_integrity_batch_and_error_queue_draft.md)
- 참조 정합성 운영 `DDL` 초안: [./reference_integrity_operations_ddl_draft.md](./reference_integrity_operations_ddl_draft.md)

## 1. 목적

운영 보완 모델과 `DDL` 초안만으로는 실제 운영 화면이 어떤 집계를 필요로 하는지 바로 드러나지 않는다. 본 문서는 참조 정합성 운영 대시보드에 필요한 조회 모델을 정의해, 화면 설계와 조회 최적화, 배치 적재 전략의 기준으로 사용한다.

## 2. 대시보드 대상 사용자

- 시스템 관리자: 전체 오류 현황, 배치 실행 상태, 재처리 큐 상태 확인
- 조직 운영 담당자: 조직/역할 배정 관련 오류 확인
- 프로젝트 운영 담당자: 프로젝트/업무 항목 기준 오류 확인
- 운영 관리자: 장기 미해결 오류, 반복 오류, 재처리 실패 추이 확인

## 3. 핵심 화면 위젯 초안

### 3.1 요약 카드

- 미해결 오류 수
- `critical` 오류 수
- 오늘 신규 오류 수
- 자동 재처리 대기 건수
- 최근 배치 실패 건수

### 3.2 오류 분포 차트

- 오류 유형별 분포
- 심각도별 분포
- 원천 엔터티 유형별 분포
- 탐지 채널별 분포

### 3.3 추세 차트

- 일별 신규 오류 추이
- 일별 해결 완료 추이
- 자동 재처리 성공률 추이
- 반복 발생 오류 추이

### 3.4 운영 리스트

- 미해결 `critical`, `high` 오류 목록
- 재처리 대기 큐 목록
- 최근 배치 실행 이력
- 장기 미해결 오류 목록

## 4. 필터 기준 초안

- 기간
- 심각도
- 오류 유형
- 원천 엔터티 유형
- 원천 엔터티 식별자
- 탐지 채널
- 배치 작업 코드
- 오류 상태
- 재처리 상태

## 5. 권장 조회 모델 초안

### 5.1 `reference_integrity_dashboard_summary`

- 목적: 상단 요약 카드 제공
- 집계 기준:
  - 미해결 오류 수
  - `critical` 오류 수
  - 오늘 신규 오류 수
  - 재처리 대기 건수
  - 최근 배치 실패 건수
- 주 키 후보:
  - `as_of_date`

### 5.2 `reference_integrity_issue_distribution`

- 목적: 오류 분포 차트 제공
- 집계 축:
  - `issue_type`
  - `severity`
  - `source_entity_type`
  - `detection_channel`
- 값:
  - `issue_count`
- 주 키 후보:
  - `as_of_date + dimension_type + dimension_value`

### 5.3 `reference_integrity_issue_trend_daily`

- 목적: 일별 추세 제공
- 집계 축:
  - `event_date`
  - `metric_type`
- 값:
  - `metric_value`
- 대표 지표:
  - `new_issue_count`
  - `resolved_issue_count`
  - `retry_success_count`
  - `retry_failure_count`

### 5.4 `reference_integrity_open_issue_list`

- 목적: 운영 리스트 기본 뷰 제공
- 주요 컬럼:
  - `reference_integrity_issue_id`
  - `issue_type`
  - `severity`
  - `source_entity_type`
  - `source_entity_id`
  - `issue_status`
  - `detected_at`
  - `last_checked_at`
  - `age_hours`
  - `latest_action_type`
  - `latest_action_at`
- 정렬 기본값:
  - `severity desc`, `detected_at asc`

### 5.5 `reference_retry_queue_monitor`

- 목적: 재처리 큐 모니터링
- 주요 컬럼:
  - `reference_retry_queue_id`
  - `reference_integrity_issue_id`
  - `retry_status`
  - `retry_count`
  - `next_retry_at`
  - `last_retry_at`
  - `last_error_message`
- 정렬 기본값:
  - `next_retry_at asc`

### 5.6 `reference_integrity_check_run_monitor`

- 목적: 배치 실행 모니터링
- 주요 컬럼:
  - `check_run_id`
  - `job_code`
  - `run_status`
  - `started_at`
  - `ended_at`
  - `checked_count`
  - `issue_count`
  - `critical_count`
  - `high_count`
- 정렬 기본값:
  - `started_at desc`

## 6. 조회 모델 생성 방식

### 6.1 실시간 조회 권장

- `reference_integrity_open_issue_list`
- `reference_retry_queue_monitor`
- `reference_integrity_check_run_monitor`

운영자가 즉시 확인해야 하는 목록은 원천 테이블 또는 가벼운 뷰로 직접 조회하는 편이 적절하다.

### 6.2 배치 집계 권장

- `reference_integrity_dashboard_summary`
- `reference_integrity_issue_distribution`
- `reference_integrity_issue_trend_daily`

집계형 위젯은 일 단위 또는 시간 단위 스냅샷으로 생성하는 편이 적절하다.

## 7. 권장 갱신 주기

- 오픈 오류 목록: 실시간 또는 1분 이내
- 재처리 큐 모니터: 실시간 또는 1분 이내
- 배치 실행 모니터: 실시간
- 요약 카드: 5분 또는 15분 단위
- 일별 추세: 1시간 또는 배치 완료 후

## 8. 운영 대시보드 최소 화면 초안

### 8.1 첫 화면

- 상단 요약 카드 5개
- 중앙 좌측 오류 유형 분포
- 중앙 우측 심각도 분포
- 하단 미해결 우선순위 오류 목록

### 8.2 상세 화면

- 배치 실행 이력 탭
- 재처리 큐 탭
- 장기 미해결 오류 탭
- 원천 엔터티 기준 오류 탐색 탭

## 9. 후속 상세화 후보

- 조회 모델 `DDL` 또는 materialized view 초안 작성
- 운영 대시보드 UI 와 역할별 접근 범위 정의
- 경고 임계치와 알림 정책 문서화
