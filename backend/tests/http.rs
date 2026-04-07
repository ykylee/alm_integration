use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::Utc;
use tower::ServiceExt;

use backend::app_state::AppState;
use backend::http::router::build_router;
use backend::security::ingestion_auth::{
    IngestionAuthConfig, IngestionAuthRegistry, sign_ingestion_payload,
};

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
    let app = build_router(AppState::with_ingestion_auth(IngestionAuthRegistry::new()));

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

#[tokio::test]
async fn ingestion_event_rejects_missing_signature_when_source_requires_auth() {
    let mut registry = IngestionAuthRegistry::new();
    registry.register(
        "jira",
        IngestionAuthConfig {
            shared_secret: "jira-webhook-secret".to_string(),
            allowed_skew_seconds: 300,
        },
    );
    let app = build_router(AppState::with_ingestion_auth(registry));

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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn ingestion_event_accepts_valid_hmac_signature() {
    let mut registry = IngestionAuthRegistry::new();
    registry.register(
        "jira",
        IngestionAuthConfig {
            shared_secret: "jira-webhook-secret".to_string(),
            allowed_skew_seconds: 300,
        },
    );
    let app = build_router(AppState::with_ingestion_auth(registry));
    let body = r#"{"source_system":"jira","source_object_type":"issue","source_object_id":"ALM-123","source_event_key":"jira-event-8891","source_version":"42","source_updated_at":"2026-04-07T08:15:00Z","payload":{"issue":{"id":"10001","key":"ALM-123","fields":{"updated":"2026-04-07T08:15:00.000+0000","summary":"Sync process update","status":{"name":"In Progress"}}}}}"#;
    let timestamp = Utc::now().to_rfc3339();
    let signature = sign_ingestion_payload(
        "jira-webhook-secret",
        &timestamp,
        "POST",
        "/api/v1/ingestion/events",
        body.as_bytes(),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ingestion/events")
                .header("content-type", "application/json")
                .header("x-source-system", "jira")
                .header("x-signature-timestamp", &timestamp)
                .header("x-signature", signature)
                .header("x-idempotency-key", "jira-event-8891")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}

#[tokio::test]
async fn ingestion_event_rejects_invalid_hmac_signature() {
    let mut registry = IngestionAuthRegistry::new();
    registry.register(
        "jira",
        IngestionAuthConfig {
            shared_secret: "jira-webhook-secret".to_string(),
            allowed_skew_seconds: 300,
        },
    );
    let app = build_router(AppState::with_ingestion_auth(registry));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ingestion/events")
                .header("content-type", "application/json")
                .header("x-source-system", "jira")
                .header("x-signature-timestamp", "2026-04-07T08:16:00Z")
                .header("x-signature", "sha256=invalid")
                .header("x-idempotency-key", "jira-event-8891")
                .body(Body::from(
                    r#"{"source_system":"jira","source_object_type":"issue","source_object_id":"ALM-123","source_event_key":"jira-event-8891","source_version":"42","source_updated_at":"2026-04-07T08:15:00Z","payload":{"issue":{"id":"10001","key":"ALM-123","fields":{"updated":"2026-04-07T08:15:00.000+0000","summary":"Sync process update","status":{"name":"In Progress"}}}}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
