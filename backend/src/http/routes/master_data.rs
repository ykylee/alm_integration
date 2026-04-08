use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{patch, post},
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::services::master_data::{
    MasterDataRepository, MasterDataRepositoryError, MasterDataStoreError,
    OrganizationChangeLogRecord, OrganizationListFilter, OrganizationRecord,
    OrganizationStructureSnapshotRecord, UpdateOrganizationInput, UpdateWorkforceInput,
    UpsertOrganizationInput, UpsertWorkforceInput, WorkforceChangeLogRecord, WorkforceListFilter,
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

#[derive(Debug, Deserialize)]
struct UpsertOrganizationMemberRequest {
    employee_number: String,
    display_name: String,
    employment_status: String,
    job_family: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateOrganizationRequest {
    organization_name: Option<String>,
    parent_organization_code: Option<Option<String>>,
    organization_status: Option<String>,
    effective_from: Option<Option<String>>,
    effective_to: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
struct UpdateWorkforceRequest {
    display_name: Option<String>,
    employment_status: Option<String>,
    primary_organization_code: Option<String>,
    job_family: Option<Option<String>>,
    email: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
struct OrganizationListResponse {
    items: Vec<OrganizationRecord>,
}

#[derive(Debug, Serialize)]
struct WorkforceListResponse {
    items: Vec<WorkforceRecord>,
}

#[derive(Debug, Serialize)]
struct OrganizationHistoryResponse {
    items: Vec<OrganizationChangeLogRecord>,
}

#[derive(Debug, Serialize)]
struct WorkforceHistoryResponse {
    items: Vec<WorkforceChangeLogRecord>,
}

#[derive(Debug, Serialize)]
struct OrganizationStructureResponse {
    item: OrganizationStructureSnapshotRecord,
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
            "/master-data/organizations/{organization_code}",
            patch(update_organization).delete(delete_organization),
        )
        .route(
            "/master-data/organizations/{organization_code}/history",
            axum::routing::get(list_organization_history),
        )
        .route(
            "/master-data/organizations/{organization_code}/structure",
            axum::routing::get(get_organization_structure),
        )
        .route(
            "/master-data/organizations/{organization_code}/members",
            post(upsert_organization_member).get(list_organization_members),
        )
        .route(
            "/master-data/organizations/{organization_code}/member-history",
            axum::routing::get(list_organization_member_history),
        )
        .route(
            "/master-data/organizations/{organization_code}/members/{employee_number}",
            patch(update_organization_member).delete(delete_organization_member),
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

async fn update_organization(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
    Json(request): Json<UpdateOrganizationRequest>,
) -> Result<Json<OrganizationRecord>, (StatusCode, &'static str)> {
    let input = UpdateOrganizationInput {
        organization_name: request.organization_name,
        parent_organization_code: request.parent_organization_code,
        organization_status: request.organization_status,
        effective_from: request.effective_from,
        effective_to: request.effective_to,
    };

    let record = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .update_organization(&organization_code, input)
            .await
            .map_err(map_repository_error)?
    } else {
        let mut store = state.master_data_store.write().await;
        store
            .update_organization(&organization_code, input)
            .map_err(map_store_error)?
    };

    Ok(Json(record))
}

async fn delete_organization(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .soft_delete_organization(&organization_code)
            .await
            .map_err(map_repository_error)?;
    } else {
        let mut store = state.master_data_store.write().await;
        store
            .soft_delete_organization(&organization_code)
            .map_err(map_store_error)?;
    };

    Ok(StatusCode::NO_CONTENT)
}

async fn list_organization_history(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
) -> Result<Json<OrganizationHistoryResponse>, (StatusCode, &'static str)> {
    let items = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .list_organization_history(&organization_code)
            .await
            .map_err(map_repository_error)?
    } else {
        let store = state.master_data_store.read().await;
        store.list_organization_history(&organization_code)
    };

    Ok(Json(OrganizationHistoryResponse { items }))
}

async fn get_organization_structure(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
) -> Result<Json<OrganizationStructureResponse>, (StatusCode, &'static str)> {
    let item = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .get_organization_structure(&organization_code)
            .await
            .map_err(map_repository_error)?
    } else {
        let store = state.master_data_store.read().await;
        store
            .get_organization_structure(&organization_code)
            .map_err(map_store_error)?
    };

    Ok(Json(OrganizationStructureResponse { item }))
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

async fn list_organization_members(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
    Query(mut filter): Query<WorkforceListFilter>,
) -> Result<Json<WorkforceListResponse>, (StatusCode, &'static str)> {
    filter.primary_organization_code = Some(organization_code);
    list_workforce(State(state), Query(filter)).await
}

async fn upsert_organization_member(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
    Json(request): Json<UpsertOrganizationMemberRequest>,
) -> Result<(StatusCode, Json<WorkforceRecord>), (StatusCode, &'static str)> {
    let input = UpsertWorkforceInput {
        employee_number: request.employee_number,
        display_name: request.display_name,
        employment_status: request.employment_status,
        primary_organization_code: organization_code,
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

async fn update_organization_member(
    State(state): State<AppState>,
    Path((_organization_code, employee_number)): Path<(String, String)>,
    Json(request): Json<UpdateWorkforceRequest>,
) -> Result<Json<WorkforceRecord>, (StatusCode, &'static str)> {
    let input = UpdateWorkforceInput {
        display_name: request.display_name,
        employment_status: request.employment_status,
        primary_organization_code: request.primary_organization_code,
        job_family: request.job_family,
        email: request.email,
    };

    let record = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .update_workforce(&employee_number, input)
            .await
            .map_err(map_repository_error)?
    } else {
        let mut store = state.master_data_store.write().await;
        store
            .update_workforce(&employee_number, input)
            .map_err(map_store_error)?
    };

    Ok(Json(record))
}

async fn delete_organization_member(
    State(state): State<AppState>,
    Path((_organization_code, employee_number)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .soft_delete_workforce(&employee_number)
            .await
            .map_err(map_repository_error)?;
    } else {
        let mut store = state.master_data_store.write().await;
        store
            .soft_delete_workforce(&employee_number)
            .map_err(map_store_error)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn list_organization_member_history(
    State(state): State<AppState>,
    Path(organization_code): Path<String>,
) -> Result<Json<WorkforceHistoryResponse>, (StatusCode, &'static str)> {
    let items = if let Some(pool) = state.db_pool.clone() {
        MasterDataRepository::new(pool)
            .list_organization_member_history(&organization_code)
            .await
            .map_err(map_repository_error)?
    } else {
        let store = state.master_data_store.read().await;
        store.list_organization_member_history(&organization_code)
    };

    Ok(Json(WorkforceHistoryResponse { items }))
}

fn map_store_error(error: MasterDataStoreError) -> (StatusCode, &'static str) {
    match error {
        MasterDataStoreError::InvalidTimestamp => (StatusCode::BAD_REQUEST, "INVALID_TIMESTAMP"),
        MasterDataStoreError::InvalidReference => (StatusCode::BAD_REQUEST, "INVALID_REFERENCE"),
        MasterDataStoreError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND"),
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
        MasterDataRepositoryError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
        MasterDataRepositoryError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
        }
    }
}
