use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::services::sync_runs::{
    CancelSyncRunInput, CreateSyncRunInput, SyncRunRecord, SyncRunRepository,
    SyncRunRepositoryError, SyncRunStoreError,
};

#[derive(Debug, Deserialize)]
struct CreateSyncRunRequest {
    source_system: String,
    mode: String,
    #[serde(default)]
    scope: serde_json::Value,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RetrySyncRunRequest {
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CancelSyncRunRequest {
    reason: Option<String>,
    cancel_reason_code: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateSyncRunResponse {
    request_id: String,
    accepted: bool,
    run_id: String,
    status: String,
    status_reason_code: String,
}

#[derive(Debug, Serialize)]
struct CancelSyncRunResponse {
    request_id: String,
    accepted: bool,
    run_id: String,
    status: String,
    status_reason_code: String,
    cancel_requested_at: Option<String>,
    message: String,
}

#[derive(Debug, Serialize)]
struct SyncRunListResponse {
    items: Vec<SyncRunRecord>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sync-runs", post(create_sync_run).get(list_sync_runs))
        .route("/sync-runs/", post(create_sync_run).get(list_sync_runs))
        .route("/sync-runs/{run_id}", get(get_sync_run))
        .route("/sync-runs/{run_id}/retry", post(retry_sync_run))
        .route("/sync-runs/{run_id}/cancel", post(cancel_sync_run))
}

async fn create_sync_run(
    State(state): State<AppState>,
    Json(request): Json<CreateSyncRunRequest>,
) -> (StatusCode, Json<CreateSyncRunResponse>) {
    let input = CreateSyncRunInput {
        source_system: request.source_system,
        mode: request.mode,
        scope: request.scope,
        reason: request.reason,
    };
    let record = if let Some(pool) = state.db_pool.clone() {
        SyncRunRepository::new(pool)
            .create(input)
            .await
            .expect("sync run creation should succeed")
    } else {
        let mut store = state.sync_run_store.write().await;
        store.create(input)
    };

    (
        StatusCode::ACCEPTED,
        Json(CreateSyncRunResponse {
            request_id: record.run_id.clone(),
            accepted: true,
            run_id: record.run_id.clone(),
            status: record.run_status.clone(),
            status_reason_code: record.status_reason_code.clone(),
        }),
    )
}

async fn list_sync_runs(State(state): State<AppState>) -> Json<SyncRunListResponse> {
    let items = if let Some(pool) = state.db_pool.clone() {
        SyncRunRepository::new(pool)
            .list()
            .await
            .expect("sync run listing should succeed")
    } else {
        let store = state.sync_run_store.read().await;
        store.list()
    };
    Json(SyncRunListResponse { items })
}

async fn get_sync_run(
    State(state): State<AppState>,
    Path(run_id): Path<String>,
) -> Result<Json<SyncRunRecord>, StatusCode> {
    if let Some(pool) = state.db_pool.clone() {
        let repository = SyncRunRepository::new(pool);
        repository
            .get(&run_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    } else {
        let store = state.sync_run_store.read().await;
        store.get(&run_id).map(Json).ok_or(StatusCode::NOT_FOUND)
    }
}

async fn retry_sync_run(
    State(state): State<AppState>,
    Path(run_id): Path<String>,
    Json(request): Json<RetrySyncRunRequest>,
) -> Result<(StatusCode, Json<CreateSyncRunResponse>), (StatusCode, &'static str)> {
    let result = if let Some(pool) = state.db_pool.clone() {
        SyncRunRepository::new(pool)
            .retry(&run_id, request.reason)
            .await
            .map_err(map_repository_error)
    } else {
        let mut store = state.sync_run_store.write().await;
        store
            .retry(&run_id, request.reason)
            .map_err(map_store_error)
    };
    match result {
        Ok(record) => Ok((
            StatusCode::ACCEPTED,
            Json(CreateSyncRunResponse {
                request_id: record.run_id.clone(),
                accepted: true,
                run_id: record.run_id.clone(),
                status: record.run_status.clone(),
                status_reason_code: record.status_reason_code.clone(),
            }),
        )),
        Err(error) => Err(error),
    }
}

async fn cancel_sync_run(
    State(state): State<AppState>,
    Path(run_id): Path<String>,
    headers: HeaderMap,
    Json(request): Json<CancelSyncRunRequest>,
) -> Result<(StatusCode, Json<CancelSyncRunResponse>), (StatusCode, &'static str)> {
    let requested_by = headers
        .get("X-User-Id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("system.admin")
        .to_string();

    let input = CancelSyncRunInput {
        requested_by,
        reason: request.reason,
        cancel_reason_code: request.cancel_reason_code,
    };
    let result = if let Some(pool) = state.db_pool.clone() {
        SyncRunRepository::new(pool)
            .cancel(&run_id, input)
            .await
            .map_err(map_repository_error)
    } else {
        let mut store = state.sync_run_store.write().await;
        store.cancel(&run_id, input).map_err(map_store_error)
    };

    match result {
        Ok(result) => Ok((
            if result.accepted {
                StatusCode::ACCEPTED
            } else {
                StatusCode::OK
            },
            Json(CancelSyncRunResponse {
                request_id: run_id,
                accepted: result.accepted,
                run_id: result.record.run_id.clone(),
                status: result.record.run_status.clone(),
                status_reason_code: result.reason_code.clone(),
                cancel_requested_at: result.record.cancel_requested_at.clone(),
                message: result.message.clone(),
            }),
        )),
        Err(error) => Err(error),
    }
}

fn map_store_error(error: SyncRunStoreError) -> (StatusCode, &'static str) {
    match error {
        SyncRunStoreError::NotFound => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
        SyncRunStoreError::NotRetriable => (StatusCode::CONFLICT, "RUN_NOT_RETRIABLE"),
        SyncRunStoreError::NotCancellable => (StatusCode::CONFLICT, "RUN_NOT_CANCELLABLE"),
    }
}

fn map_repository_error(error: SyncRunRepositoryError) -> (StatusCode, &'static str) {
    match error {
        SyncRunRepositoryError::NotFound => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND"),
        SyncRunRepositoryError::NotRetriable => (StatusCode::CONFLICT, "RUN_NOT_RETRIABLE"),
        SyncRunRepositoryError::NotCancellable => (StatusCode::CONFLICT, "RUN_NOT_CANCELLABLE"),
        SyncRunRepositoryError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
        }
    }
}
