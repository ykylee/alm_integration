# 참조 정합성 대시보드 UI 및 접근 제어 초안

- 문서 목적: 참조 정합성 운영 대시보드의 화면 구조와 역할별 접근 범위를 정의한다.
- 범위: 운영 대시보드 정보구조, 화면 구성, 역할별 메뉴/권한, 기본 필터와 액션 제한
- 대상 독자: 아키텍트, 기획자, 디자이너, 개발자, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/reference_integrity_dashboard_query_model_draft.md`, `docs/overview/role_based_ux_direction.md`, `docs/architecture/application_and_governance_architecture_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 참조 정합성 대시보드 조회 모델 초안: [./reference_integrity_dashboard_query_model_draft.md](./reference_integrity_dashboard_query_model_draft.md)
- 역할 기반 UX 구조 방향: [../overview/role_based_ux_direction.md](../overview/role_based_ux_direction.md)

## 1. 목적

참조 정합성 운영 대시보드는 단순 조회 화면이 아니라, 오류 인지, 우선순위 판단, 재처리 통제, 운영 이력 확인을 함께 수행하는 작업면이다. 본 문서는 이미 정리된 조회 모델을 실제 화면 구조와 역할별 접근 정책으로 연결해, 이후 상세 UI 설계와 권한 정책 구현의 기준으로 사용한다.

## 2. 대시보드 배치 원칙

- 일반 업무 화면과 분리된 운영 대시보드로 배치한다.
- 시스템 관리자 관점이 기본이지만, 조직 운영/프로젝트 운영 관점의 제한된 진입도 허용한다.
- 쓰기 액션과 운영 통제 액션은 명확히 분리한다.
- 같은 오류라도 역할에 따라 기본 필터, 기본 정렬, 허용 액션이 달라야 한다.

권장 경로:

- `/admin/reference-integrity`
- `/admin/reference-integrity/issues`
- `/admin/reference-integrity/runs`
- `/admin/reference-integrity/retry-queue`
- `/admin/reference-integrity/issues/:id`

## 3. 화면 정보구조 초안

### 3.1 운영 대시보드 홈

- 상단 요약 카드
  - 미해결 오류 수
  - `critical` 오류 수
  - 오늘 신규 오류 수
  - 재처리 대기 건수
  - 최근 배치 실패 건수
- 중단 차트 영역
  - 오류 유형 분포
  - 심각도 분포
  - 최근 7일 추세
- 하단 작업 리스트
  - 미해결 `critical`, `high` 오류
  - 장기 미해결 오류

### 3.2 오류 목록 화면

- 기본 컬럼:
  - 오류 유형
  - 심각도
  - 원천 엔터티 유형
  - 원천 엔터티 식별자
  - 상태
  - 감지 시각
  - 경과 시간
  - 최근 조치
- 기본 액션:
  - 상세 보기
  - 확인 처리
  - 재처리 요청
  - 무시 처리

### 3.3 배치 실행 화면

- 기본 컬럼:
  - 작업 코드
  - 실행 상태
  - 시작/종료 시각
  - 점검 건수
  - 오류 건수
  - 심각도별 건수
- 기본 액션:
  - 실행 상세 보기
  - 실패 실행 재시도

### 3.4 재처리 큐 화면

- 기본 컬럼:
  - 재처리 유형
  - 상태
  - 재시도 횟수
  - 다음 재시도 시각
  - 마지막 오류 메시지
- 기본 액션:
  - 즉시 재처리
  - 재시도 취소
  - 상세 보기

### 3.5 오류 상세 화면

- 헤더:
  - 오류 유형
  - 심각도
  - 현재 상태
  - 감지 시각
- 본문 섹션:
  - 원천 엔터티 정보
  - 참조 정보
  - 오류 메시지와 진단 정보
  - 최근 배치/재처리 이력
  - 운영자 조치 이력
- 우측 액션 패널:
  - 확인
  - 재처리 요청
  - 해결 처리
  - 무시 처리

## 4. 역할별 접근 범위 초안

### 4.1 시스템 관리자

- 접근 범위:
  - 전체 대시보드
  - 전체 오류 목록
  - 전체 배치 실행 이력
  - 전체 재처리 큐
  - 오류 상세와 모든 운영 액션
- 기본 필터:
  - 없음
- 허용 액션:
  - 확인
  - 재처리 요청/실행
  - 무시
  - 해결 처리
  - 배치 재실행

### 4.2 조직 운영 담당자

- 접근 범위:
  - 조직/역할 배정 관련 오류 중심 제한 조회
  - `role_assignment`, 조직 참조, 인력 관련 오류 목록
  - 관련 오류 상세
- 기본 필터:
  - `source_entity_type in ('organization_master')`
  - `issue_type in ('policy_rule_violation', 'overlapping_effective_period')`
- 허용 액션:
  - 확인
  - 재처리 요청
  - 운영 메모 기록
- 제한:
  - 배치 재실행 불가
  - 시스템 전역 오류 조회 불가

### 4.3 프로젝트 운영 담당자

- 접근 범위:
  - 프로젝트/업무 항목 관련 오류 제한 조회
  - `work_item_plan_link`, 프로젝트 참조 불일치, 상태 불일치 오류
- 기본 필터:
  - `source_entity_type in ('project', 'work_item')`
- 허용 액션:
  - 확인
  - 재처리 요청
  - 관련 프로젝트 상세 이동
- 제한:
  - 권한/조직 관련 오류 조치 불가
  - 배치 실행 제어 불가

### 4.4 운영 관리자

- 접근 범위:
  - 전체 집계와 장기 미해결 추세
  - 오류 목록 조회
  - 운영 지표 확인
- 기본 필터:
  - `issue_status in ('open', 'acknowledged', 'retry_scheduled')`
- 허용 액션:
  - 확인
  - 해결 승인
- 제한:
  - 직접 재처리 실행은 시스템 관리자 권한 필요

## 5. 역할별 기본 랜딩 초안

- 시스템 관리자: 운영 대시보드 홈
- 조직 운영 담당자: 조직/역할 오류 필터가 적용된 오류 목록
- 프로젝트 운영 담당자: 프로젝트/업무 오류 필터가 적용된 오류 목록
- 운영 관리자: 장기 미해결 및 추세 중심 홈

복수 역할 사용자는 상단 역할 전환 스위처로 접근 관점을 바꾸는 편이 적절하다.

## 6. 권장 필터 프리셋

### 6.1 시스템 관리자 프리셋

- `critical_and_high_open`
- `retry_failed_recent`
- `latest_failed_runs`

### 6.2 조직 운영 프리셋

- `organization_assignment_conflicts`
- `role_policy_violations`

### 6.3 프로젝트 운영 프리셋

- `cross_project_reference_mismatch`
- `status_history_mismatch`
- `planning_rule_violation`

## 7. 액션 권한 정책 초안

- `view_dashboard_summary`: 시스템 관리자, 운영 관리자
- `view_issue_list_limited`: 조직 운영 담당자, 프로젝트 운영 담당자
- `view_issue_detail`: 허용된 범위 내 모든 운영 역할
- `acknowledge_issue`: 시스템 관리자, 운영 관리자, 범위 제한 운영 담당자
- `request_retry`: 시스템 관리자, 조직 운영 담당자, 프로젝트 운영 담당자
- `execute_retry`: 시스템 관리자
- `rerun_check_batch`: 시스템 관리자
- `resolve_issue`: 시스템 관리자, 운영 관리자
- `ignore_issue`: 시스템 관리자

## 8. UX 리스크와 대응

- 시스템 관리자와 조직 운영 담당자를 같은 대시보드 홈으로 묶으면 전역 운영 오류와 조직 오류가 섞여 집중도가 떨어질 수 있다.
- 프로젝트 운영 담당자에게 재처리 실행 권한까지 주면 운영 통제 경계가 흐려진다.
- 장기 미해결 오류와 실시간 재처리 큐를 한 리스트로 합치면 우선순위 판단이 어려워진다.

권장 대응:

- 홈은 역할별로 다르게 시작한다.
- 읽기와 실행 액션은 분리한다.
- 오류 목록, 배치 실행, 재처리 큐를 별도 탭으로 나눈다.

## 9. 후속 상세화 후보

- 운영 대시보드 와이어프레임 초안 작성
- 역할별 메뉴 노출 규칙을 권한 매트릭스에 반영
- 대시보드 액션별 감사 로그 항목 정의
