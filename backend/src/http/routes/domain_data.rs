use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::app_state::AppState;
use crate::services::domain_read::{
    DomainReadRepository, DomainReadRepositoryError, ProjectListFilter, ProjectSummaryRecord,
    WorkItemListFilter, WorkItemSummaryRecord,
};

#[derive(Debug, Serialize)]
struct ProjectListResponse {
    items: Vec<ProjectSummaryRecord>,
}

#[derive(Debug, Serialize)]
struct WorkItemListResponse {
    items: Vec<WorkItemSummaryRecord>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", axum::routing::get(list_projects))
        .route("/projects/", axum::routing::get(list_projects))
        .route("/work-items", axum::routing::get(list_work_items))
        .route("/work-items/", axum::routing::get(list_work_items))
}

async fn list_projects(
    State(state): State<AppState>,
    Query(filter): Query<ProjectListFilter>,
) -> Result<Json<ProjectListResponse>, (StatusCode, &'static str)> {
    let pool = state
        .db_pool
        .clone()
        .ok_or((StatusCode::SERVICE_UNAVAILABLE, "DATABASE_REQUIRED"))?;
    let items = DomainReadRepository::new(pool)
        .list_projects(&filter)
        .await
        .map_err(map_repository_error)?;

    Ok(Json(ProjectListResponse { items }))
}

async fn list_work_items(
    State(state): State<AppState>,
    Query(filter): Query<WorkItemListFilter>,
) -> Result<Json<WorkItemListResponse>, (StatusCode, &'static str)> {
    let pool = state
        .db_pool
        .clone()
        .ok_or((StatusCode::SERVICE_UNAVAILABLE, "DATABASE_REQUIRED"))?;
    let items = DomainReadRepository::new(pool)
        .list_work_items(&filter)
        .await
        .map_err(map_repository_error)?;

    Ok(Json(WorkItemListResponse { items }))
}

fn map_repository_error(error: DomainReadRepositoryError) -> (StatusCode, &'static str) {
    match error {
        DomainReadRepositoryError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
        }
    }
}
