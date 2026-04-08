# 시스템 서비스 분류 및 `MSA` 적용 판단 초안

- 문서 목적: 현재까지 작성된 문서와 구현 범위를 중간점검하고, 시스템이 제공하는 서비스를 분류한 뒤 `MSA` 채택 적합성을 평가한다.
- 범위: 문서/구현 현황 요약, 서비스 분류, 경계 후보, `MSA` 장단점, 권장 아키텍처 방향
- 대상 독자: 아키텍트, 백엔드 개발자, 기술 리드, 운영자
- 상태: draft
- 최종 수정일: 2026-04-08
- 관련 문서: `docs/architecture/integration_backend_component_draft.md`, `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_backend_implementation_rollout_and_checklist_draft.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 컴포넌트 문서: [./integration_backend_component_draft.md](./integration_backend_component_draft.md)

## 1. 중간점검 요약

### 1.1 현재 문서화 수준

현재 저장소는 개요, `CRS`, `SRS`, 백엔드 컴포넌트, 수집 시퀀스, 초기 `DDL`, 물리 모델, 운영 통제 구조, 인증/서명 정책, 워커 정책까지 아키텍처 골격이 비교적 폭넓게 정리된 상태다. 특히 백엔드 구현 관점에서는 다음 문서 축이 이미 연결돼 있다.

- 제품 목적과 범위: `overview`, `requirements`
- 런타임 컴포넌트와 수집 흐름: `integration_backend_component_draft.md`, `integration_data_ingestion_sequence_draft.md`
- 저장 구조 기준: `initial_release_ddl_draft.md`, `initial_release_physical_model_draft.md`
- 구현 순서와 체크리스트: `integration_backend_implementation_rollout_and_checklist_draft.md`
- 운영 보완 구조: 참조 정합성, 오류 큐, 재처리, 대시보드 초안

즉, 문서 공백 때문에 구현을 못 하는 단계는 아니고, 오히려 구현 진행에 맞춰 문서를 현실화해야 하는 단계로 볼 수 있다.

### 1.2 현재 구현 수준

2026-04-08 기준 구현은 다음 최소 수직 슬라이스까지 닫혀 있다.

- 외부 시스템 `pull`/`push` 어댑터 레지스트리와 concrete adapter
- `sync_run`, `raw_ingestion_event`, 연결 설정, 자격증명 복호화
- `ingestion` 서명 검증
- `normalization_pipeline`
- `identity_mapping_service`
- `reference_resolution_service`
- `project_write_service`
- `work_item_write_service`
- `work_item_status_history`
- `work_item_hierarchy`

즉, 현재 백엔드는 “외부 수집 -> 원시 적재 -> 표준화 -> 식별자 매핑 -> 참조 해소 -> 최소 도메인 반영”까지 하나의 실행 흐름으로 연결된 상태다.

### 1.3 아직 비어 있는 주요 영역

- `work_item_plan_link`
- `release`, `milestone`, `iteration` 실제 반영
- `organization_master_service`, `workforce_master_service`
- `reference_integrity_service`, `integrity_issue_service`, `retry_queue_service`
- 조회 모델/운영 대시보드 최적화
- 관리자용 기준정보/운영 API 확장

따라서 현재는 “통합 수집 백엔드의 쓰기 파이프라인 최소선”은 확보했지만, 운영 통제와 계획 도메인까지 완결된 상태는 아니다.

## 2. 시스템 제공 서비스 분류

현재 문서와 구현을 합치면, 이 시스템은 기능적으로 다음 서비스를 제공하는 것으로 분류할 수 있다.

### 2.1 연계 수집 서비스

- 외부 시스템 연결 설정 관리
- 외부 시스템 `pull` 실행
- 외부 이벤트 `push` 수신
- 수집 실행 이력 관리
- 원시 적재와 멱등 판정

현재 구현 상태:

- 대부분 구현됨
- concrete adapter, `sync_runs`, `raw_ingestion`, `ingestion auth` 까지 동작

### 2.2 표준화 및 기준키 매핑 서비스

- 외부 payload 표준화
- 외부 식별자와 내부 기준키 매핑
- 참조 누락 이벤트 재평가

현재 구현 상태:

- 최소선 구현됨
- `normalization_pipeline`, `identity_mapping_service`, `reference_resolution_service` 존재

### 2.3 업무 도메인 반영 서비스

- `project` 반영
- `work_item` 반영
- 업무 상태 이력 반영
- 부모-자식 계층 반영
- 향후 계획 링크 반영

현재 구현 상태:

- `project`, `work_item`, `work_item_status_history`, `work_item_hierarchy` 최소선 구현
- `work_item_plan_link` 와 계획 단위 반영은 미구현

### 2.4 기준정보 관리 서비스

- 조직 마스터 관리
- 인력 마스터 관리
- 워크플로우/프로세스/계획 규칙 관리
- 외부 시스템 연결 정책 관리

현재 구현 상태:

- 연결 설정 일부 구현
- 조직/인력/워크플로우 기준정보 반영은 대부분 미구현

### 2.5 운영 통제 서비스

- 참조 정합성 검증
- 오류 큐 저장
- 재처리 큐 운영
- 실행 취소, 재시도, 감사

현재 구현 상태:

- `sync-runs` 취소/재시도, 일부 정합성 판정 구현
- 본격적인 `reference_integrity_service`, 운영 이슈/재처리 큐는 미구현

### 2.6 관리자/운영 API 서비스

- 수동 동기화 실행
- 실행 이력 조회
- 정합성 이슈 조회/재처리
- 연결 설정 등록/수정
- 기준정보 관리

현재 구현 상태:

- `ingestion`, `sync-runs` 중심의 최소 API 구현
- 운영 통제 API와 기준정보 관리 API는 후속

## 3. 서비스 경계 후보 정리

기능 분류만 보면 다음 6개 정도의 큰 경계가 자연스럽다.

1. `integration-ingestion`
설정, 어댑터, `pull`/`push`, 원시 적재, 실행 이력

2. `integration-normalization`
표준화, 식별자 매핑, 참조 해소

3. `domain-work-management`
`project`, `work_item`, 상태 이력, 계층, 계획 링크

4. `master-data`
조직, 인력, 역할, 정책, 워크플로우, 계획 스킴

5. `operations-control`
정합성 이슈, 재처리 큐, 감사, 운영 알림

6. `admin-api`
관리자 진입점, 운영 조회, 설정 관리

이 분류는 향후 서비스 분리 후보를 보는 데는 유용하지만, 현재 시점에 곧바로 별도 배포 단위로 쪼개는 것이 최선이라는 뜻은 아니다.

## 4. `MSA` 채택 적합성 판단

### 4.1 결론

현 시점에서는 전체 시스템에 대해 본격적인 `MSA` 를 바로 채택하는 것보다, 강한 모듈 경계를 가진 `modular monolith` 를 우선 유지하는 편이 더 적절하다.

### 4.2 그렇게 판단하는 이유

1. 현재 핵심 흐름이 강하게 순차 결합돼 있다.
`pull` 또는 `push` 실행은 원시 적재, 표준화, 식별자 매핑, 참조 해소, 도메인 반영이 하나의 파이프라인으로 강하게 이어진다. 지금 이를 서비스 경계마다 물리적으로 분리하면 네트워크 호출, 비동기 보상, 분산 추적, 재시도 설계가 급격히 복잡해진다.

2. 현재 저장 모델의 일관성 요구가 높다.
`sync_run`, `raw_ingestion_event`, `normalized_record_reference`, `identity_mapping`, `project`, `work_item` 은 같은 실행 컨텍스트 안에서 해석될 때 이점이 크다. 특히 참조 해소와 도메인 쓰기는 강한 정합성 요구를 가진다.

3. 도메인 경계는 보이지만 아직 안정화 단계다.
문서상 서비스 경계는 제안 가능하지만, 실제 구현은 아직 `project`/`work_item` 최소 반영 단계다. 이 시점의 조기 분해는 잘못된 경계를 굳히는 위험이 있다.

4. 중앙 통합 DB라는 제품 성격상 쓰기 중심 허브가 필요하다.
이 시스템은 개별 도구를 대체하기보다 분산된 정보를 중앙에서 연결하고 표준화하는 허브 성격이 강하다. 초기에는 하나의 일관된 쓰기 허브를 두는 편이 운영과 장애 분석에 유리하다.

5. 운영 복잡도 대비 이득이 아직 크지 않다.
`MSA` 로 가면 배포 파이프라인, 서비스 디스커버리, 분산 로깅, 계약 버전 관리, 장애 격리, 메시지 보상 로직이 따라온다. 현재 기능 성숙도 대비 이 운영 비용이 더 크다.

### 4.3 현재 단계 권장안

권장안은 다음과 같다.

- 런타임은 하나의 `backend` 로 유지한다.
- 코드 구조는 서비스 경계를 명확히 반영하는 모듈로 분리한다.
- DB 스키마도 경계별로 점진적으로 나눈다.
- 서비스 간 호출처럼 보이는 부분은 일단 내부 모듈 호출로 유지한다.
- 이벤트/큐 기반 전환 가능 지점만 미리 식별해 둔다.

즉, “지금은 `MSA` 를 하지 말자”가 아니라, “지금은 `MSA` 가능성을 열어둔 모듈형 단일 백엔드가 최적”이라는 판단이다.

## 5. 향후 `MSA` 전환이 유효해지는 조건

다음 조건이 충족되면 일부 경계는 별도 서비스로 분리할 가치가 커진다.

### 5.1 분리 우선 후보

- `integration-ingestion`
외부 시스템별 트래픽, 재시도, 스케줄링 부하가 커질 때

- `operations-control`
정합성 이슈 처리와 재처리 큐가 운영적으로 독립 수명주기를 가질 때

- `admin-api`
관리자 기능이 별도 접근 제어, 배포 주기, 감사 규칙을 요구할 때

### 5.2 분리를 늦추는 편이 좋은 후보

- `integration-normalization`
- `domain-work-management`

이 둘은 현재 가장 강한 데이터 일관성과 실행 순서 의존성을 가지므로, 성급히 분리하면 비용이 크다.

### 5.3 전환 조건 예시

- 팀이 서비스별로 나뉘어 독립 배포를 실제로 원할 때
- `pull`/`push` 트래픽이 커져 수집 워커를 따로 확장해야 할 때
- 운영 통제 기능이 본체와 다른 보안/배포 정책을 요구할 때
- 읽기 부하와 쓰기 부하의 확장 전략이 완전히 달라질 때
- 메시지 버스와 분산 추적 체계를 감당할 운영 역량이 확보됐을 때

## 6. 권장 아키텍처 방향

현재 권장 방향은 다음과 같다.

1. 단일 `backend` 유지
배포 단위는 하나로 유지한다.

2. 내부 모듈 경계 강화
`services/` 아래를 수집, 표준화, 도메인 쓰기, 운영 통제로 더 분명히 나눈다.

3. 비동기 전환 지점 명시
향후 큐 기반으로 분리할 후보 메서드와 이벤트를 문서화한다.

4. 운영 통제 기능을 먼저 독립 가능한 모듈로 설계
이슈 큐, 재처리, 감사는 나중에 분리하기 쉬운 구조로 만든다.

5. 조회 모델은 쓰기 파이프라인과 느슨하게
대시보드/리포트 계층은 읽기 최적화 방향으로 별도 모델을 준비한다.

## 7. 최종 판단

현재 시스템은 기능적으로는 여러 서비스로 분류할 수 있고, 장기적으로는 일부 경계를 `MSA` 로 분리할 여지가 있다. 하지만 2026-04-08 시점의 구현 성숙도와 데이터 일관성 요구를 기준으로 보면, 지금 즉시 `MSA` 를 채택하는 것은 과한 선택이다.

따라서 현재의 최적 해법은 다음과 같다.

- 지금: 모듈형 단일 백엔드
- 중기: 운영 통제/수집 계층 중심 선택적 분리 검토
- 장기: 조직 구조와 트래픽, 운영 복잡도가 충분히 커졌을 때 제한적 `MSA`

즉, 이 시스템은 “`MSA` 가 불가능한 시스템”은 아니지만, “지금 당장 `MSA` 로 시작하는 것이 합리적인 시스템”도 아니다.
