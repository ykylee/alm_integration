# 참조 정합성 운영 DDL 초안

- 문서 목적: 참조 정합성 점검 배치와 오류 큐 운영 모델을 `DDL` 스타일로 구체화한다.
- 범위: `reference_integrity_check_job`, `reference_integrity_check_run`, `reference_integrity_issue`, `reference_integrity_issue_action`, `reference_retry_queue` 의 `DDL` 초안
- 대상 독자: 아키텍트, 개발자, 운영자, 데이터 모델러, `DBA`
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/reference_integrity_batch_and_error_queue_draft.md`, `docs/architecture/initial_release_ddl_draft.md`, `docs/architecture/polymorphic_reference_validation_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 참조 정합성 점검 배치 및 오류 큐 초안: [./reference_integrity_batch_and_error_queue_draft.md](./reference_integrity_batch_and_error_queue_draft.md)
- 초기 릴리스 `DDL` 초안: [./initial_release_ddl_draft.md](./initial_release_ddl_draft.md)

## 1. 문서 사용 원칙

- 본 문서는 운영 보완용 테이블만 대상으로 한 `DDL` 초안이다.
- 특정 `RDBMS` 최종 문법이 아니라 `PostgreSQL` 계열을 가정한 준-표준 문법을 사용한다.
- 오류 큐와 배치 실행 이력은 append-heavy 성격을 가지므로 조회 인덱스와 상태 전환 추적을 우선 고려한다.
- 다형 참조는 이 문서에서도 정규 외래키보다 `type + id` 조합, 코드값 검증, 운영 이력 보존을 우선한다.

## 2. 공통 가정

```sql
-- 공통 가정
-- 1) 모든 PK는 uuid 사용
-- 2) 상태/유형/심각도는 varchar 코드값 사용
-- 3) 대용량 검색이 필요한 조합에는 복합 인덱스를 둔다
-- 4) 운영 이력은 삭제보다 상태 전환과 이력 누적을 우선한다
```

## 3. 정합성 점검 배치

### 3.1 `reference_integrity_check_job`

```sql
create table reference_integrity_check_job (
  check_job_id uuid primary key,
  job_code varchar(50) not null,
  job_name varchar(200) not null,
  check_scope_type varchar(50) not null,
  schedule_expression varchar(100) null,
  is_active boolean not null default true,
  description text null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint ux_reference_integrity_check_job_code
    unique (job_code)
);

create index ix_reference_integrity_check_job_active
  on reference_integrity_check_job (is_active);
```

### 3.2 `reference_integrity_check_run`

```sql
create table reference_integrity_check_run (
  check_run_id uuid primary key,
  check_job_id uuid not null,
  started_at timestamptz not null,
  ended_at timestamptz null,
  run_status varchar(30) not null,
  checked_count bigint not null default 0,
  issue_count bigint not null default 0,
  critical_count bigint not null default 0,
  high_count bigint not null default 0,
  medium_count bigint not null default 0,
  low_count bigint not null default 0,
  trigger_type varchar(30) not null,
  triggered_by varchar(100) null,
  run_note text null,
  created_at timestamptz not null,
  constraint fk_reference_integrity_check_run_job
    foreign key (check_job_id)
    references reference_integrity_check_job (check_job_id),
  constraint ck_reference_integrity_check_run_status
    check (run_status in ('queued', 'running', 'succeeded', 'failed', 'cancelled'))
);

create index ix_reference_integrity_check_run_job_started_at
  on reference_integrity_check_run (check_job_id, started_at desc);

create index ix_reference_integrity_check_run_status
  on reference_integrity_check_run (run_status, started_at desc);
```

## 4. 오류 큐 및 조치 이력

### 4.1 `reference_integrity_issue`

```sql
create table reference_integrity_issue (
  reference_integrity_issue_id uuid primary key,
  check_run_id uuid null,
  issue_type varchar(50) not null,
  severity varchar(20) not null,
  source_entity_type varchar(50) not null,
  source_entity_id uuid not null,
  reference_type varchar(50) not null,
  reference_code varchar(100) null,
  issue_status varchar(30) not null,
  issue_message text not null,
  detection_channel varchar(30) not null,
  detected_at timestamptz not null,
  last_checked_at timestamptz not null,
  resolved_at timestamptz null,
  resolved_by varchar(100) null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint fk_reference_integrity_issue_check_run
    foreign key (check_run_id)
    references reference_integrity_check_run (check_run_id),
  constraint ck_reference_integrity_issue_severity
    check (severity in ('critical', 'high', 'medium', 'low')),
  constraint ck_reference_integrity_issue_status
    check (issue_status in ('open', 'acknowledged', 'retry_scheduled', 'resolved', 'ignored'))
);

create index ix_reference_integrity_issue_status_severity
  on reference_integrity_issue (issue_status, severity, detected_at desc);

create index ix_reference_integrity_issue_source_ref
  on reference_integrity_issue (source_entity_type, source_entity_id);

create index ix_reference_integrity_issue_type
  on reference_integrity_issue (issue_type, detected_at desc);
```

### 4.2 `reference_integrity_issue_action`

```sql
create table reference_integrity_issue_action (
  issue_action_id uuid primary key,
  reference_integrity_issue_id uuid not null,
  action_type varchar(30) not null,
  action_by varchar(100) not null,
  action_note text null,
  action_at timestamptz not null,
  created_at timestamptz not null,
  constraint fk_reference_integrity_issue_action_issue
    foreign key (reference_integrity_issue_id)
    references reference_integrity_issue (reference_integrity_issue_id),
  constraint ck_reference_integrity_issue_action_type
    check (action_type in ('acknowledge', 'retry_request', 'resolve', 'ignore', 'reopen'))
);

create index ix_reference_integrity_issue_action_issue
  on reference_integrity_issue_action (reference_integrity_issue_id, action_at desc);
```

### 4.3 `reference_retry_queue`

```sql
create table reference_retry_queue (
  reference_retry_queue_id uuid primary key,
  reference_integrity_issue_id uuid not null,
  retry_type varchar(50) not null,
  retry_status varchar(30) not null,
  retry_count integer not null default 0,
  max_retry_count integer not null default 3,
  next_retry_at timestamptz null,
  last_retry_at timestamptz null,
  last_error_message text null,
  locked_by varchar(100) null,
  locked_at timestamptz null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  constraint fk_reference_retry_queue_issue
    foreign key (reference_integrity_issue_id)
    references reference_integrity_issue (reference_integrity_issue_id),
  constraint ck_reference_retry_queue_status
    check (retry_status in ('queued', 'running', 'succeeded', 'failed', 'cancelled'))
);

create index ix_reference_retry_queue_status_next_retry
  on reference_retry_queue (retry_status, next_retry_at);

create index ix_reference_retry_queue_issue
  on reference_retry_queue (reference_integrity_issue_id);
```

## 5. 권장 보조 제약

```sql
-- 동일 이슈에 대해 활성 재시도 큐를 1건만 허용하는 방향을 권장
-- 실제 구현 시 partial unique index 검토
-- where retry_status in ('queued', 'running')
```

```sql
-- 미해결 이슈 조회 성능을 위해 partial index 검토
-- where issue_status in ('open', 'acknowledged', 'retry_scheduled')
```

## 6. 운영 조회 예시 관점

- 오늘 새로 발생한 `critical`, `high` 오류 목록
- 미해결 오류 상위 20건
- 동일 원천 엔터티에서 반복 발생하는 오류 추이
- 자동 재처리 실패 누적 건수
- 최근 배치 실행 결과와 오류 분포

## 7. 후속 상세화 후보

- 운영 대시보드 조회 모델 `DDL` 초안
- 알림 이벤트 적재 테이블 초안
- 재처리 백오프 정책과 잠금 전략 구체화
