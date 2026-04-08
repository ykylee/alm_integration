use axum::Router;
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::app_state::AppState;
use crate::http::routes::{domain_data, health, ingestion, master_data, sync_runs};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", health::router())
        .nest("/api/v1", ingestion::router())
        .nest("/api/v1/admin", sync_runs::router())
        .nest("/api/v1/admin", master_data::router())
        .nest("/api/v1/admin", domain_data::router())
        .layer(build_cors_layer())
        .with_state(state)
}

fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://127.0.0.1:8000"),
            HeaderValue::from_static("http://localhost:8000"),
            HeaderValue::from_static("http://127.0.0.1:8001"),
            HeaderValue::from_static("http://localhost:8001"),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
}
