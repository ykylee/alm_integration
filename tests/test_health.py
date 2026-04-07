from fastapi.testclient import TestClient

from alm_integration_backend.main import app


client = TestClient(app)


def test_health_returns_ok() -> None:
    response = client.get("/api/v1/health")

    assert response.status_code == 200
    assert response.json() == {"status": "ok"}


def test_sync_run_lifecycle_stub() -> None:
    create_response = client.post(
        "/api/v1/admin/sync-runs",
        json={
            "source_system": "jira",
            "mode": "incremental",
            "scope": {"project_keys": ["ALM"]},
            "reason": "initial scaffold test",
        },
    )

    assert create_response.status_code == 202
    run_id = create_response.json()["run_id"]

    detail_response = client.get(f"/api/v1/admin/sync-runs/{run_id}")
    assert detail_response.status_code == 200
    assert detail_response.json()["run_status"] == "queued"

    cancel_response = client.post(
        f"/api/v1/admin/sync-runs/{run_id}/cancel",
        json={"cancel_reason_code": "operator_manual_stop"},
    )
    assert cancel_response.status_code == 202
    assert cancel_response.json()["status_reason_code"] == "cancel_requested"
