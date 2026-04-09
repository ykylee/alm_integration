use axum::Router;
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::app_state::AppState;
use crate::config::default_cors_allowed_origins;
use crate::http::routes::{domain_data, health, ingestion, master_data, sync_runs};

pub fn build_router(state: AppState) -> Router {
    let default_origins = default_cors_allowed_origins();
    build_router_with_origins(state, &default_origins)
}

pub fn build_router_with_origins(state: AppState, cors_allowed_origins: &[String]) -> Router {
    Router::new()
        .nest("/api/v1", health::router())
        .nest("/api/v1", ingestion::router())
        .nest("/api/v1/admin", sync_runs::router())
        .nest("/api/v1/admin", master_data::router())
        .nest("/api/v1/admin", domain_data::router())
        .layer(build_cors_layer(cors_allowed_origins))
        .with_state(state)
}

fn build_cors_layer(cors_allowed_origins: &[String]) -> CorsLayer {
    let allowed_origins = cors_allowed_origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
}
