use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::adapters::{AdapterError, PushAdapterRequest};
use crate::app_state::AppState;
use crate::security::ingestion_auth::{IngestionAuthError, verify_ingestion_request};
use crate::services::push_ingestion::{PushIngestionProcessor, PushIngestionProcessorError};
use crate::services::raw_ingestion::{
    CreateRawIngestionEventInput, RawIngestionRepository, RawIngestionRepositoryError,
};

#[derive(Debug, Clone, Deserialize)]
struct IngestionEventRequest {
    source_system: String,
    source_object_type: String,
    source_object_id: String,
    source_event_key: String,
    source_version: Option<String>,
    source_updated_at: Option<String>,
    payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct IngestionEventResponse {
    request_id: String,
    accepted: bool,
    run_id: String,
    status: String,
    message: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ingestion/events", post(create_ingestion_event))
        .route("/ingestion/events/", post(create_ingestion_event))
}

async fn create_ingestion_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<IngestionEventResponse>), (StatusCode, &'static str)> {
    let request: IngestionEventRequest =
        serde_json::from_slice(&body).map_err(|_| (StatusCode::BAD_REQUEST, "INVALID_JSON"))?;

    verify_ingestion_request(
        &state.ingestion_auth_registry,
        &request.source_system,
        &headers,
        &axum::http::Method::POST,
        "/api/v1/ingestion/events",
        &body,
        chrono::Utc::now(),
    )
    .map_err(map_ingestion_auth_error)?;

    let input = if let Some(adapter) = state
        .adapter_registry
        .get_push_adapter(&request.source_system)
    {
        adapter
            .adapt(PushAdapterRequest {
                source_system: request.source_system,
                source_object_type: request.source_object_type,
                source_object_id: request.source_object_id,
                source_event_key: request.source_event_key,
                source_version: request.source_version,
                source_updated_at: request.source_updated_at,
                payload: request.payload,
            })
            .map_err(map_adapter_error)?
    } else {
        CreateRawIngestionEventInput {
            source_system: request.source_system,
            source_object_type: request.source_object_type,
            source_object_id: request.source_object_id,
            source_event_key: request.source_event_key,
            source_version: request.source_version,
            source_updated_at: request.source_updated_at,
            payload: request.payload,
        }
    };

    let record = if let Some(pool) = state.db_pool.clone() {
        RawIngestionRepository::new(pool)
            .create(input)
            .await
            .map_err(map_repository_error)?
    } else {
        let mut store = state.raw_ingestion_store.write().await;
        store.create(input)
    };

    if record.accepted {
        if let Some(pool) = state.db_pool.clone() {
            PushIngestionProcessor::new(pool)
                .process_run(&record.run_id, 1)
                .await
                .map_err(map_push_processor_error)?;
        }
    }

    Ok((
        if record.accepted {
            StatusCode::ACCEPTED
        } else {
            StatusCode::OK
        },
        Json(IngestionEventResponse {
            request_id: record.request_id,
            accepted: record.accepted,
            run_id: record.run_id,
            status: record.status,
            message: record.message,
        }),
    ))
}

fn map_repository_error(error: RawIngestionRepositoryError) -> (StatusCode, &'static str) {
    match error {
        RawIngestionRepositoryError::InvalidSourceUpdatedAt(_) => {
            (StatusCode::BAD_REQUEST, "INVALID_SOURCE_UPDATED_AT")
        }
        RawIngestionRepositoryError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
        }
    }
}

fn map_adapter_error(error: AdapterError) -> (StatusCode, &'static str) {
    match error {
        AdapterError::UnsupportedSourceSystem(_) => {
            (StatusCode::BAD_REQUEST, "UNSUPPORTED_SOURCE_SYSTEM")
        }
        AdapterError::InvalidPayload(_) => (StatusCode::BAD_REQUEST, "INVALID_PAYLOAD"),
        AdapterError::ExternalCall(_) => (StatusCode::BAD_GATEWAY, "EXTERNAL_CALL_FAILED"),
    }
}

fn map_ingestion_auth_error(error: IngestionAuthError) -> (StatusCode, &'static str) {
    match error {
        IngestionAuthError::MissingHeader(_) => (StatusCode::UNAUTHORIZED, "MISSING_AUTH_HEADER"),
        IngestionAuthError::SourceSystemMismatch => {
            (StatusCode::FORBIDDEN, "SOURCE_SYSTEM_MISMATCH")
        }
        IngestionAuthError::InvalidTimestamp => {
            (StatusCode::UNAUTHORIZED, "INVALID_SIGNATURE_TIMESTAMP")
        }
        IngestionAuthError::TimestampExpired => {
            (StatusCode::UNAUTHORIZED, "SIGNATURE_TIMESTAMP_EXPIRED")
        }
        IngestionAuthError::InvalidSignature => (StatusCode::UNAUTHORIZED, "INVALID_SIGNATURE"),
    }
}

fn map_push_processor_error(error: PushIngestionProcessorError) -> (StatusCode, &'static str) {
    match error {
        PushIngestionProcessorError::Normalization(_)
        | PushIngestionProcessorError::ReferenceResolution(_)
        | PushIngestionProcessorError::OrganizationWrite(_)
        | PushIngestionProcessorError::WorkforceWrite(_)
        | PushIngestionProcessorError::ProjectWrite(_)
        | PushIngestionProcessorError::WorkItemWrite(_)
        | PushIngestionProcessorError::SyncRun(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INGESTION_POST_PROCESSING_FAILED",
        ),
    }
}
