use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::adapters::{AdapterError, PushAdapterRequest};
use crate::app_state::AppState;
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
    Json(request): Json<IngestionEventRequest>,
) -> Result<(StatusCode, Json<IngestionEventResponse>), (StatusCode, &'static str)> {
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
