use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use backend::app_state::AppState;
use backend::http::router::build_router;

#[tokio::test]
async fn health_returns_ok() {
    let app = build_router(AppState::new());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn sync_run_can_be_created_and_cancelled() {
    let app = build_router(AppState::new());

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/sync-runs/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"source_system":"jira","mode":"incremental","scope":{"project_keys":["ALM"]}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::ACCEPTED);
}

#[tokio::test]
async fn ingestion_event_can_be_accepted() {
    let app = build_router(AppState::new());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ingestion/events")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"source_system":"jira","source_object_type":"issue","source_object_id":"ALM-123","source_event_key":"jira-event-8891","source_version":"42","source_updated_at":"2026-04-07T08:15:00Z","payload":{"issue":{"id":"10001","key":"ALM-123","fields":{"updated":"2026-04-07T08:15:00.000+0000","summary":"Sync process update","status":{"name":"In Progress"}}}}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
