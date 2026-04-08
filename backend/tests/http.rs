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
async fn sync_run_list_can_be_filtered_by_source_system_and_status() {
    let app = build_router(AppState::new());

    let jira_response = app
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
    assert_eq!(jira_response.status(), StatusCode::ACCEPTED);

    let bitbucket_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/sync-runs/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"source_system":"bitbucket","mode":"incremental","scope":{"project_keys":["OPS"]}}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bitbucket_response.status(), StatusCode::ACCEPTED);

    let bitbucket_body = axum::body::to_bytes(bitbucket_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let bitbucket_json: serde_json::Value = serde_json::from_slice(&bitbucket_body).unwrap();
    let bitbucket_run_id = bitbucket_json["run_id"].as_str().unwrap();

    let cancel_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!(
                    "/api/v1/admin/sync-runs/{bitbucket_run_id}/cancel"
                ))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"reason":"stop","cancel_reason_code":"operator_manual_stop"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(cancel_response.status(), StatusCode::ACCEPTED);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/sync-runs?source_system=bitbucket&status=queued")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json["items"].as_array().unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["source_system"], "bitbucket");
    assert_eq!(items[0]["run_status"], "queued");
}

#[tokio::test]
async fn sync_run_detail_returns_created_record() {
    let app = build_router(AppState::new());

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/sync-runs/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"source_system":"jira","mode":"incremental","scope":{"project_keys":["ALM"]},"reason":"manual detail check"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let run_id = create_json["run_id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/v1/admin/sync-runs/{run_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["run_id"], run_id);
    assert_eq!(json["source_system"], "jira");
    assert_eq!(json["reason"], "manual detail check");
}

#[tokio::test]
async fn master_data_organization_can_be_upserted_and_listed() {
    let app = build_router(AppState::new());

    let upsert_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_code":"platform","organization_name":"Platform Center","organization_status":"active","effective_from":"2026-04-08T00:00:00Z"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(upsert_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations?organization_status=active")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json["items"].as_array().unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["organization_code"], "platform");
    assert_eq!(items[0]["organization_name"], "Platform Center");
    assert_eq!(items[0]["organization_status"], "active");
}

#[tokio::test]
async fn master_data_organization_can_be_updated_and_deleted() {
    let app = build_router(AppState::new());

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_code":"platform","organization_name":"Platform Center","organization_status":"active"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let update_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/admin/master-data/organizations/platform")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_name":"Platform Group","organization_status":"active"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(update_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["organization_name"], "Platform Group");

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/admin/master-data/organizations/platform")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations?organization_status=deleted")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json["items"].as_array().unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["organization_code"], "platform");
    assert_eq!(items[0]["organization_status"], "deleted");
}

#[tokio::test]
async fn master_data_organization_rejects_hierarchy_cycle() {
    let app = build_router(AppState::new());

    for body in [
        r#"{"organization_code":"platform","organization_name":"Platform","organization_status":"active"}"#,
        r#"{"organization_code":"payments","organization_name":"Payments","organization_status":"active","parent_organization_code":"platform"}"#,
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/admin/master-data/organizations")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    let cycle_response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/admin/master-data/organizations/platform")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_name":"Platform","organization_status":"active","parent_organization_code":"payments"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(cycle_response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn master_data_workforce_can_be_upserted_and_listed() {
    let app = build_router(AppState::new());

    let organization_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_code":"delivery","organization_name":"Delivery Division","organization_status":"active"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(organization_response.status(), StatusCode::OK);

    let workforce_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/workforce")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"employee_number":"E1024","display_name":"김운영","employment_status":"active","primary_organization_code":"delivery","job_family":"platform_engineering","email":"ops@example.com"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(workforce_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/workforce?primary_organization_code=delivery")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json["items"].as_array().unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["employee_number"], "E1024");
    assert_eq!(items[0]["display_name"], "김운영");
    assert_eq!(items[0]["primary_organization_code"], "delivery");
}

#[tokio::test]
async fn organization_members_can_be_managed_via_nested_routes() {
    let app = build_router(AppState::new());

    for body in [
        r#"{"organization_code":"platform","organization_name":"Platform","organization_status":"active"}"#,
        r#"{"organization_code":"payments","organization_name":"Payments","organization_status":"active","parent_organization_code":"platform"}"#,
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/admin/master-data/organizations")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    let create_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations/platform/members")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"employee_number":"E1001","display_name":"홍관리","employment_status":"active","job_family":"operations","email":"ops@example.com"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_member.status(), StatusCode::OK);

    let update_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/api/v1/admin/master-data/organizations/platform/members/E1001")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"display_name":"홍관리자","employment_status":"active","primary_organization_code":"payments","job_family":"platform_ops"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_member.status(), StatusCode::OK);

    let list_members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations/payments/members")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_members.status(), StatusCode::OK);

    let body = axum::body::to_bytes(list_members.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json["items"].as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["display_name"], "홍관리자");
    assert_eq!(items[0]["primary_organization_code"], "payments");

    let delete_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/admin/master-data/organizations/payments/members/E1001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_member.status(), StatusCode::NO_CONTENT);

    let list_inactive = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/workforce?primary_organization_code=payments&employment_status=inactive")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_inactive.status(), StatusCode::OK);
}

#[tokio::test]
async fn master_data_history_endpoints_return_organization_and_member_logs() {
    let app = build_router(AppState::new());

    let create_org = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"organization_code":"platform","organization_name":"Platform Center","organization_status":"active"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_org.status(), StatusCode::OK);

    let create_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/admin/master-data/organizations/platform/members")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"employee_number":"E5001","display_name":"윤이력","employment_status":"active","job_family":"ops","email":"history@example.com"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_member.status(), StatusCode::OK);

    let organization_history = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations/platform/history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(organization_history.status(), StatusCode::OK);

    let organization_body = axum::body::to_bytes(organization_history.into_body(), usize::MAX)
        .await
        .unwrap();
    let organization_json: serde_json::Value = serde_json::from_slice(&organization_body).unwrap();
    assert!(!organization_json["items"].as_array().unwrap().is_empty());

    let member_history = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations/platform/member-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(member_history.status(), StatusCode::OK);

    let member_body = axum::body::to_bytes(member_history.into_body(), usize::MAX)
        .await
        .unwrap();
    let member_json: serde_json::Value = serde_json::from_slice(&member_body).unwrap();
    assert!(!member_json["items"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn master_data_structure_endpoint_returns_ancestor_child_and_member_counts() {
    let app = build_router(AppState::new());

    for payload in [
        r#"{"organization_code":"division","organization_name":"플랫폼사업부","organization_status":"active"}"#,
        r#"{"organization_code":"team","organization_name":"통합플랫폼팀","parent_organization_code":"division","organization_status":"active"}"#,
        r#"{"organization_code":"group","organization_name":"데이터허브그룹","parent_organization_code":"team","organization_status":"active"}"#,
        r#"{"organization_code":"part","organization_name":"수집연계파트","parent_organization_code":"group","organization_status":"active"}"#,
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/admin/master-data/organizations")
                    .header("content-type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    for payload in [
        r#"{"employee_number":"E7101","display_name":"한조직","employment_status":"active","job_family":"ops","email":"org1@example.com"}"#,
        r#"{"employee_number":"E7102","display_name":"두조직","employment_status":"active","job_family":"ops","email":"org2@example.com"}"#,
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/admin/master-data/organizations/part/members")
                    .header("content-type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/master-data/organizations/team/structure")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["item"]["organization_code"], "team");
    assert_eq!(json["item"]["ancestors"].as_array().unwrap().len(), 1);
    assert_eq!(json["item"]["children"].as_array().unwrap().len(), 1);
    assert_eq!(json["item"]["direct_member_count"], 0);
    assert_eq!(json["item"]["subtree_organization_count"], 3);
    assert_eq!(json["item"]["subtree_active_member_count"], 2);
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

#[tokio::test]
async fn domain_read_api_requires_database_pool() {
    let app = build_router(AppState::new());

    let projects_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(projects_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let work_items_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/work-items")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        work_items_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
}
