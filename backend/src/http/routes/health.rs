use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn router() -> Router<crate::app_state::AppState> {
    Router::new().route("/health", get(get_health))
}

async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
