# 시스템 통합 DB 인증 및 서명 정책 초안

- 문서 목적: 외부 수신 API, 관리자 운영 API, 외부 시스템 자격증명 관리에 적용할 인증 및 서명 정책의 기준을 정리한다.
- 범위: `ingestion_api` 인증/서명, 관리자 API 인증, 자격증명 암호화 키 사용 원칙, 로그/감사 마스킹 기준
- 대상 독자: 백엔드 개발자, 보안 담당자, 아키텍트, 운영자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_backend_api_and_batch_contract_draft.md`, `docs/architecture/integration_data_ingestion_sequence_draft.md`, `docs/architecture/integration_backend_implementation_rollout_and_checklist_draft.md`, `docs/requirements/system_srs.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 계획 문서: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 목적

본 문서는 시스템 통합 DB 백엔드의 두 가지 민감 경로를 분리해 다룬다. 첫째는 외부 시스템이 `push` 방식으로 데이터를 보내는 수신 경로다. 둘째는 관리자가 외부 시스템 연결 정보와 자격증명을 등록·수정하는 운영 경로다. 두 경로는 위협 모델이 다르므로 인증과 서명 기준도 다르게 가져가야 한다.

## 2. 정책 분리 원칙

- 외부 수신 `ingestion_api` 는 시스템 대 시스템 인증을 사용한다.
- 관리자 운영 API 는 사용자 인증과 역할 기반 권한을 사용한다.
- 외부 시스템 자격증명은 저장 시 암호화하고, 복호화는 실제 연계 호출 계층으로 제한한다.
- 민감정보는 요청 본문, 응답, 로그, 감사 메시지에서 마스킹 또는 비노출을 기본으로 한다.

## 3. 외부 수신 `ingestion_api` 인증 정책

### 3.1 기본 원칙

- 원천 시스템별로 독립된 인증 수단을 사용한다.
- 인증이 성공해도 요청 본문 무결성 확인이 실패하면 수신을 거부한다.
- 가능하면 전송 보안은 `TLS` 를 기본으로 한다.

### 3.2 권장 인증 방식 우선순위

1. `HMAC` 서명 기반 웹훅 인증
2. `mTLS` 또는 네트워크 제어가 결합된 시스템 토큰
3. 제한된 수명의 `Bearer Token`

초기 릴리스에서는 원천 시스템 특성에 따라 복수 방식을 허용하되, 최소한 `HMAC` 또는 이에 준하는 요청 서명 검증을 기본 옵션으로 제공하는 편이 좋다.

### 3.3 권장 헤더

- `X-Source-System`
- `X-Request-Id`
- `X-Signature`
- `X-Signature-Timestamp`
- `X-Idempotency-Key`

### 3.4 서명 검증 원칙

- 서명 대상 문자열은 최소 `timestamp + method + path + body_hash` 를 포함한다.
- 허용 시간 편차를 둬 재전송 공격을 줄인다.
- 같은 `X-Idempotency-Key` 와 같은 서명 조합은 중복 요청으로 처리한다.
- 서명 실패, 시간 초과, 원천 시스템 코드 불일치는 즉시 `401` 또는 `403` 계열로 거부한다.

## 4. 관리자 운영 API 인증 정책

### 4.1 기본 원칙

- 관리자 API 는 사용자 인증 기반으로 보호한다.
- 역할 기반 권한과 데이터 범위 정책을 같이 적용한다.
- 민감한 실행 API 는 추가 검증 또는 재인증이 가능하도록 여지를 둔다.

### 4.2 적용 대상

- `sync_operation_api`
- `integrity_operation_api`
- `master_data_api`
- `integration_admin_api`
- `policy_admin_api`

### 4.3 권장 보호 수준

- 조회 API: 관리자 세션 + 역할 확인
- 실행 API: 관리자 세션 + 역할 확인 + 감사 필드 강제
- 자격증명 등록/변경 API: 시스템 관리자 권한 + 필요 시 재인증 또는 추가 확인 절차

## 5. 외부 시스템 연결 정보 및 자격증명 보호 정책

### 5.1 저장 원칙

- `base_url`, `principal_id`, 인증 유형 같은 비민감 속성은 일반 컬럼 저장 가능
- `password`, `token`, `client secret`, `api key` 는 암호화 저장
- 원문 자격증명은 저장 후 다시 조회하지 않는다.

### 5.2 암호화 원칙

- 애플리케이션 레벨 암호화를 기본으로 한다.
- 암호화 키 식별자는 데이터와 함께 저장하되, 실제 키 값은 별도 보안 저장소 또는 환경 보안 경로로 관리한다.
- `secret_key_version` 을 함께 저장해 키 회전 가능성을 확보한다.

### 5.3 키 회전 원칙

- 신규 등록 시 최신 키 버전을 사용한다.
- 기존 자격증명은 점진적으로 재암호화할 수 있어야 한다.
- 키 폐기 전에는 관련 자격증명이 모두 새 버전으로 교체되었는지 확인해야 한다.

## 6. 로그 및 감사 정책

### 6.1 비노출 원칙

다음 값은 로그와 감사 메시지에 직접 남기지 않는다.

- `password`
- `token`
- `client secret`
- `api key`
- 서명 원문

### 6.2 허용되는 기록 범위

- `integration_system_id`
- `integration_endpoint_id`
- `credential_type`
- `principal_id` 일부 또는 마스킹 값
- 변경 주체
- 변경 시각
- 검증 성공/실패 여부

## 7. 실패 처리 기준

### 7.1 외부 수신 실패

- 인증 실패: 즉시 거부
- 서명 실패: 즉시 거부
- 시간 초과: 즉시 거부
- 스키마 오류: `400` 계열 반환 후 원시 적재 생략 가능

### 7.2 관리자 자격증명 등록 실패

- 암호화 실패: 저장 금지
- 권한 부족: 즉시 거부
- 연결 검증 실패: 저장은 허용할 수 있으나 상태를 `validation_failed` 로 표시

## 8. 초기 구현 권장 기준

- `ingestion_api` 는 `HMAC` 서명 검증을 기본 구현으로 시작
- 관리자 API 는 기존 관리자 인증 체계를 재사용하되, `integration_admin_api` 는 시스템 관리자 전용으로 제한
- 자격증명은 애플리케이션 레벨 암호화 + `secret_key_version` 저장 방식으로 시작
- 응답과 로그에서는 민감정보 원문 비노출을 강제

## 9. 후속 상세화 후보

- `HMAC` 서명 문자열 규격 상세
- 키 보관소 연계 방식
- 관리자 재인증 정책
- 자격증명 검증 실패 알림 정책
