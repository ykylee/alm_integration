use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::post,
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::services::master_data::{
    MasterDataRepository, MasterDataRepositoryError, MasterDataStoreError, OrganizationListFilter,
    OrganizationRecord, UpsertOrganizationInput, UpsertWorkforceInput, WorkforceListFilter,
    WorkforceRecord,
};

#[derive(Debug, Deserialize)]
struct UpsertOrganizationRequest {
    organization_code: String,
    organization_name: String,
    parent_organization_code: Option<String>,
    organization_status: String,
    effective_from: Option<String>,
    effective_to: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpsertWorkforceRequest {
    employee_number: String,
    display_name: String,
    employment_status: String,
    primary_organization_code: String,
    job_family: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Serialize)]
struct OrganizationListResponse {
    items: Vec<OrganizationRecord>,
}

#[derive(Debug, Serialize)]
struct WorkforceListResponse {
    items: Vec<WorkforceRecord>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/master-data/organizations",
            post(upsert_organization).get(list_organizations),
        )
        .route(
            "/master-data/organizations/",
            post(upsert_organization).get(list_organizations),
        )
        .route(
            "/master-data/workforce",
            post(upsert_workforce).get(list_workforce),
        )
        .route(
            "/master-data/workforce/",
            post(upsert_workforce).get(list_workforce),
        )
}

async fn list_organizations(
    State(state): State<AppState>,
    Query(filter): Query<OrganizationListFilter>,
) -> Result<Json<OrganizationListResponse>, (StatusCode, &'static str)> {
    let items = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .list_organizations(&filter)
            .await
            .map_err(map_repository_error)?
    } else {
        let store = state.master_data_store.read().await;
        store.list_organizations(&filter)
    };

    Ok(Json(OrganizationListResponse { items }))
}

async fn upsert_organization(
    State(state): State<AppState>,
    Json(request): Json<UpsertOrganizationRequest>,
) -> Result<(StatusCode, Json<OrganizationRecord>), (StatusCode, &'static str)> {
    let input = UpsertOrganizationInput {
        organization_code: request.organization_code,
        organization_name: request.organization_name,
        parent_organization_code: request.parent_organization_code,
        organization_status: request.organization_status,
        effective_from: request.effective_from,
        effective_to: request.effective_to,
    };

    let record = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .upsert_organization(input)
            .await
            .map_err(map_repository_error)?
    } else {
        let mut store = state.master_data_store.write().await;
        store.upsert_organization(input).map_err(map_store_error)?
    };

    Ok((StatusCode::OK, Json(record)))
}

async fn list_workforce(
    State(state): State<AppState>,
    Query(filter): Query<WorkforceListFilter>,
) -> Result<Json<WorkforceListResponse>, (StatusCode, &'static str)> {
    let items = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .list_workforce(&filter)
            .await
            .map_err(map_repository_error)?
    } else {
        let store = state.master_data_store.read().await;
        store.list_workforce(&filter)
    };

    Ok(Json(WorkforceListResponse { items }))
}

async fn upsert_workforce(
    State(state): State<AppState>,
    Json(request): Json<UpsertWorkforceRequest>,
) -> Result<(StatusCode, Json<WorkforceRecord>), (StatusCode, &'static str)> {
    let input = UpsertWorkforceInput {
        employee_number: request.employee_number,
        display_name: request.display_name,
        employment_status: request.employment_status,
        primary_organization_code: request.primary_organization_code,
        job_family: request.job_family,
        email: request.email,
    };

    let record = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .upsert_workforce(input)
            .await
            .map_err(map_repository_error)?
    } else {
        let mut store = state.master_data_store.write().await;
        store.upsert_workforce(input).map_err(map_store_error)?
    };

    Ok((StatusCode::OK, Json(record)))
}

fn map_store_error(error: MasterDataStoreError) -> (StatusCode, &'static str) {
    match error {
        MasterDataStoreError::InvalidTimestamp => (StatusCode::BAD_REQUEST, "INVALID_TIMESTAMP"),
        MasterDataStoreError::InvalidReference => (StatusCode::BAD_REQUEST, "INVALID_REFERENCE"),
    }
}

fn map_repository_error(error: MasterDataRepositoryError) -> (StatusCode, &'static str) {
    match error {
        MasterDataRepositoryError::InvalidTimestamp(_) => {
            (StatusCode::BAD_REQUEST, "INVALID_TIMESTAMP")
        }
        MasterDataRepositoryError::InvalidReference(_) => {
            (StatusCode::BAD_REQUEST, "INVALID_REFERENCE")
        }
        MasterDataRepositoryError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
        }
    }
}
