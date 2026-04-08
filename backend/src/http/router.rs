use axum::Router;

use crate::app_state::AppState;
use crate::http::routes::{health, ingestion, master_data, sync_runs};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", health::router())
        .nest("/api/v1", ingestion::router())
        .nest("/api/v1/admin", sync_runs::router())
        .nest("/api/v1/admin", master_data::router())
        .with_state(state)
}
