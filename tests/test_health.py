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

def test_sync_run_cancel_already_requested() -> None:
    create_response = client.post(
        "/api/v1/admin/sync-runs",
        json={
            "source_system": "jira",
            "mode": "incremental",
            "scope": {"project_keys": ["ALM"]},
            "reason": "already requested cancel test",
        },
    )
    run_id = create_response.json()["run_id"]

    # First cancel
    first_cancel_response = client.post(
        f"/api/v1/admin/sync-runs/{run_id}/cancel",
        json={"cancel_reason_code": "operator_manual_stop"},
    )
    assert first_cancel_response.status_code == 202

    # Second cancel
    cancel_response = client.post(
        f"/api/v1/admin/sync-runs/{run_id}/cancel",
        json={"cancel_reason_code": "operator_manual_stop"},
    )

    assert cancel_response.status_code == 200
    json_data = cancel_response.json()
    assert json_data["accepted"] is False
    assert json_data["message"] == "cancellation request already registered"


def test_sync_run_cancel_already_finished() -> None:
    from alm_integration_backend.services.sync_runs import sync_run_store

    create_response = client.post(
        "/api/v1/admin/sync-runs",
        json={
            "source_system": "jira",
            "mode": "incremental",
            "scope": {"project_keys": ["ALM"]},
            "reason": "already finished cancel test",
        },
    )
    run_id = create_response.json()["run_id"]

    # Modify store directly to make it finished
    run = sync_run_store.get(run_id)
    assert run is not None
    run.run_status = "completed"

    cancel_response = client.post(
        f"/api/v1/admin/sync-runs/{run_id}/cancel",
        json={"cancel_reason_code": "operator_manual_stop"},
    )

    assert cancel_response.status_code == 200
    json_data = cancel_response.json()
    assert json_data["accepted"] is False
    assert json_data["message"] == "run already finished; cancellation not applied"
