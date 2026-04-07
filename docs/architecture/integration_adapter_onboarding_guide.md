# 외부 시스템 어댑터 연동 가이드

- 문서 목적: 새 외부 시스템을 시스템 통합 DB 백엔드에 연동하기 위해 `pull`/`push` 어댑터를 추가하는 절차와 구현 기준을 정리한다.
- 범위: `Rust axum + sqlx` 백엔드 기준 어댑터 구조, 파일 배치, 구현 순서, 테스트 기준, 운영 연결 포인트
- 대상 독자: 백엔드 개발자, 아키텍트, 연계 개발자
- 상태: draft
- 최종 수정일: 2026-04-07
- 관련 문서: `docs/architecture/integration_backend_component_draft.md`, `docs/architecture/integration_backend_design_plan.md`, `docs/architecture/integration_data_ingestion_sequence_draft.md`, `docs/architecture/integration_backend_api_and_batch_contract_draft.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 아키텍처 위키: [./README.md](./README.md)
- 상위 설계 플랜: [./integration_backend_design_plan.md](./integration_backend_design_plan.md)

## 1. 목적

이 가이드는 새 외부 시스템을 연동할 때 어떤 파일을 만들고, 어떤 인터페이스를 구현하고, 어떤 테스트를 먼저 작성해야 하는지를 빠르게 파악할 수 있도록 만든 개발자용 안내서다. 현재 백엔드는 `Jira`, `Bitbucket`, `Bamboo`, `Confluence` 를 concrete adapter 예시로 가지고 있으며, 이후 다른 시스템도 같은 패턴으로 확장할 수 있어야 한다.

## 2. 현재 구조 요약

관련 코드 위치:

- 공통 어댑터 인터페이스와 레지스트리: [mod.rs](/home/yklee/repos/alm_integration/backend/src/adapters/mod.rs)
- 수신 경로: [ingestion.rs](/home/yklee/repos/alm_integration/backend/src/http/routes/ingestion.rs)
- `pull` 실행 경로: [pull_sync.rs](/home/yklee/repos/alm_integration/backend/src/services/pull_sync.rs)
- 예시 시스템:
  - [jira.rs](/home/yklee/repos/alm_integration/backend/src/adapters/jira.rs)
  - [bitbucket.rs](/home/yklee/repos/alm_integration/backend/src/adapters/bitbucket.rs)
  - [bamboo.rs](/home/yklee/repos/alm_integration/backend/src/adapters/bamboo.rs)
  - [confluence.rs](/home/yklee/repos/alm_integration/backend/src/adapters/confluence.rs)

현재 구조의 핵심 원칙:

- 외부 시스템별 차이는 adapter 안에만 둔다.
- 내부 저장 경로는 `RawIngestionRepository`, `PullSyncOrchestrator`, `SyncRunRepository` 로 공통화한다.
- 새 시스템을 붙일 때 공통 서비스나 HTTP 라우트 로직은 가능하면 건드리지 않고, adapter 와 registry 등록만으로 끝내는 방향을 우선한다.
- 구현은 `TDD` 로 진행하고, adapter 단위 테스트를 먼저 작성한다.
- 기본 registry 생성은 `build_default_registry()` 에서 수행하고, 설정 레코드 기반 조립은 `build_registry_from_endpoint_configs()` 로 수행한다.

## 3. 어댑터 유형

### 3.1 `pull` 어댑터

역할:

- 외부 시스템 API 를 호출한다.
- 시스템별 증분 조회 URL 과 요청 파라미터를 조합한다.
- 외부 응답을 내부 `PullRecordInput` 목록으로 변환한다.

구현 대상 인터페이스:

- `PullSourceAdapter`

주요 입력:

- `PullAdapterRequest.mode`
- `PullAdapterRequest.scope`

주요 출력:

- `Vec<PullRecordInput>`

### 3.2 `push` 어댑터

역할:

- 외부 시스템 webhook 또는 수신 payload 를 해석한다.
- 공통 원시 적재 입력인 `CreateRawIngestionEventInput` 으로 변환한다.

구현 대상 인터페이스:

- `PushEventAdapter`

주요 입력:

- `PushAdapterRequest`

주요 출력:

- `CreateRawIngestionEventInput`

## 4. 새 시스템 추가 절차

### 4.1 파일 생성

새 시스템 `foo` 를 붙인다고 가정하면 다음 파일을 추가한다.

- `backend/src/adapters/foo.rs`

필요하면 테스트 편의를 위한 내부 helper 를 같은 파일에 `#[cfg(test)]` 로 둔다.

### 4.2 `pull` 어댑터 구현

기본 패턴:

1. `FooPullAdapter` 구조체를 만든다.
2. 공통 `HttpTransport` 를 주입받는다.
3. `base_url`, `token` 또는 기타 접속 설정을 구조체 필드로 가진다.
4. `from_env()` 또는 후속 설정 로더용 생성자를 만든다.
5. `build_*_url()` 로 조회 URL 조합 책임을 분리한다.
6. `parse_*_response()` 로 외부 응답을 `PullRecordInput` 으로 변환한다.
7. `PullSourceAdapter` 의 `pull()` 에서는 `HTTP` 호출과 파싱만 수행한다.

최소 구현 예시:

```rust
pub struct FooPullAdapter {
    transport: Arc<dyn HttpTransport>,
    base_url: Option<String>,
    bearer_token: Option<String>,
}

#[async_trait]
impl PullSourceAdapter for FooPullAdapter {
    fn source_system(&self) -> &'static str {
        "foo"
    }

    async fn pull(
        &self,
        request: PullAdapterRequest,
    ) -> Result<Vec<PullRecordInput>, AdapterError> {
        let url = self.build_list_url(&request)?;
        let response = self.transport.get_json(AdapterHttpRequest {
            url,
            bearer_token: self.bearer_token.clone(),
        }).await?;

        Self::parse_list_response(response)
    }
}
```

구현 시 필수 확인:

- `source_object_type` 를 명확히 고정한다.
- `source_object_id` 는 외부 시스템에서 안정적으로 재사용되는 식별자를 사용한다.
- `source_event_key` 는 멱등 판정에 쓸 수 있도록 객체 식별자와 변경 시점을 조합한다.
- `source_updated_at` 는 가능하면 `RFC3339` 로 변환한다.

### 4.3 `push` 어댑터 구현

기본 패턴:

1. `FooPushAdapter` 구조체를 만든다.
2. webhook payload 에서 내부 기준 식별자와 변경 시각을 추출한다.
3. `CreateRawIngestionEventInput` 으로 바로 변환한다.

최소 구현 예시:

```rust
pub struct FooPushAdapter;

impl PushEventAdapter for FooPushAdapter {
    fn source_system(&self) -> &'static str {
        "foo"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        let object_id = request
            .payload
            .get("entity")
            .and_then(|entity| entity.get("id"))
            .and_then(|value| value.as_str())
            .ok_or_else(|| AdapterError::InvalidPayload("foo entity.id is missing".to_string()))?;

        Ok(CreateRawIngestionEventInput {
            source_system: "foo".to_string(),
            source_object_type: "entity".to_string(),
            source_object_id: object_id.to_string(),
            source_event_key: format!("foo-entity-{object_id}"),
            source_version: request.source_version,
            source_updated_at: request.source_updated_at,
            payload: request.payload,
        })
    }
}
```

구현 시 필수 확인:

- 기존 수신 API 의 평탄한 요청 필드를 무시하고 payload 기준으로 다시 식별자를 만들지 결정해야 한다.
- webhook payload 에 없는 필드는 외부 시스템 API 조회 없이 만들어낼 수 있는 값만 사용한다.
- 잘못된 payload 는 `AdapterError::InvalidPayload` 로 반환한다.

### 4.4 레지스트리 등록

새 adapter 를 만들었으면 [mod.rs](/home/yklee/repos/alm_integration/backend/src/adapters/mod.rs)의 `build_registry_from_endpoint_configs()` 와 `build_default_registry()` 경로를 함께 확인한다.

현재 구조:

- `build_registry_from_endpoint_configs()`: `integration_endpoint` 성격의 설정 레코드 배열을 받아 registry 를 조립한다.
- `build_default_registry()`: 환경변수 기반 기본 설정을 `AdapterEndpointConfig` 로 만든 뒤 위 함수를 호출한다.

예시:

```rust
let configs = vec![AdapterEndpointConfig {
    source_system: "foo".to_string(),
    base_url: Some("https://foo.example.com".to_string()),
    bearer_token: Some("foo-token".to_string()),
    enable_pull: true,
    enable_push: true,
}];

let registry = build_registry_from_endpoint_configs(&configs, transport)?;
```

등록 시 기준:

- `source_system()` 반환값과 API 요청의 `source_system` 이 정확히 같아야 한다.
- `pull` 만 지원하는 시스템이면 `register_pull_adapter` 만 추가한다.
- `push` 만 지원하는 시스템이면 `register_push_adapter` 만 추가한다.

## 5. 테스트 작성 순서

### 5.1 `TDD` 원칙

새 adapter 는 반드시 테스트부터 작성한다.

권장 순서:

1. 응답 파싱 단위 테스트 작성
2. webhook payload 변환 단위 테스트 작성
3. 필요 시 registry 통합 테스트 또는 라우트/오케스트레이터 통합 테스트 추가
4. 구현
5. `cargo fmt`, `cargo test` 실행

### 5.2 최소 테스트 목록

`pull` adapter:

- 정상 응답 1건 이상을 `PullRecordInput` 으로 변환한다.
- 필수 필드가 없으면 `InvalidPayload` 를 반환한다.
- 시간 필드가 있으면 `RFC3339` 로 변환한다.

`push` adapter:

- 정상 webhook payload 를 `CreateRawIngestionEventInput` 으로 변환한다.
- 필수 payload 가 없으면 `InvalidPayload` 를 반환한다.

registry/runtime:

- `build_default_registry()` 에서 새 `source_system` 을 조회할 수 있다.
- 필요하면 `ingestion` 라우트 또는 `PullSyncOrchestrator` 가 실제로 새 adapter 를 사용한다.

검증 명령:

```bash
cargo fmt --manifest-path backend/Cargo.toml --all
cargo test --manifest-path backend/Cargo.toml
```

## 6. 필드 설계 기준

### 6.1 `source_event_key`

권장 기준:

- 객체 식별자 + 변경 시각
- 또는 외부 시스템이 제공하는 이벤트 고유 키

피해야 할 기준:

- 요청 시점마다 달라지는 임의 UUID
- 정렬만 가능한 값이고 멱등 보장에 쓸 수 없는 카운터

### 6.2 `source_object_id`

권장 기준:

- 외부 시스템의 안정적인 업무 객체 식별자

예시:

- `Jira`: `issue key`
- `Bitbucket`: `repository slug + pull request id` 또는 내부 정책상 안정적인 조합
- `Bamboo`: `build result key`
- `Confluence`: `page id`

### 6.3 `source_updated_at`

권장 기준:

- 외부 시스템 원본 시각을 보존하되, 저장 전에는 `RFC3339` 로 정규화한다.

## 7. 설정 연결 원칙

현재 concrete adapter 는 `from_env()` 기반으로 `base_url`, `token` 을 읽는다. 다만 후속 구현에서는 관리자 API 에서 관리하는 `integration_endpoint`, `integration_credential` 과 연결해야 한다.

현재 구현 상태:

- `DbAdapterConfigLoader` 가 `integration_system`, `integration_endpoint`, `integration_credential` 를 읽어 `AdapterEndpointConfig` 목록을 만든다.
- 앱 시작 시 DB 설정이 존재하면 이 로더로 registry 를 구성하고, DB 설정이 없거나 로더가 실패하면 환경변수 기반 기본 registry 로 fallback 한다.

따라서 새 adapter 는 다음 원칙을 따른다.

- 환경변수 생성자는 초기 개발과 테스트용 진입점으로만 본다.
- registry 조립의 표준 경로는 `AdapterEndpointConfig` 기반 builder 로 본다.
- 실제 운영 연결은 후속 설정 로더가 adapter 생성자에 `base_url`, 인증 타입, 암호화 해제된 자격증명을 주입하는 방향으로 확장한다.
- adapter 내부에서는 자격증명 저장소나 `DB` 를 직접 조회하지 않는다.

## 8. 권장 리뷰 체크리스트

- `source_system()` 이름이 API 계약과 일치하는가
- `source_event_key` 가 멱등 판정에 충분한가
- `source_updated_at` 정규화가 일관적인가
- `InvalidPayload` 와 `ExternalCall` 이 구분돼 있는가
- 공통 저장 경로를 재사용하고 있는가
- adapter 가 비즈니스 쓰기 로직까지 가져가고 있지 않은가
- 단위 테스트와 최소 통합 테스트가 함께 있는가

## 9. 다음 단계

새 시스템 adapter 를 붙인 뒤에는 보통 다음 순서로 이어진다.

1. `build_default_registry()` 등록
2. `DbAdapterConfigLoader` 를 통해 `integration_endpoint`/`integration_credential` 와 연결
3. `pull` 실행 운영 API 와 실제 시스템 설정 연결
4. 원시 적재 이후 `normalization_pipeline` 규칙 추가
5. 운영 문서와 백로그 갱신
