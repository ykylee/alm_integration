import pytest
from unittest.mock import patch
from fastapi.testclient import TestClient

from alm_integration_backend.main import app
from alm_integration_backend.services.sync_runs import SyncRunRecord

client = TestClient(app)


def test_retry_sync_run_success():
    with patch("alm_integration_backend.api.routes.sync_runs.sync_run_store.retry") as mock_retry:
        mock_record = SyncRunRecord(
            run_id="sync_1234567890ab",
            source_system="jira",
            run_status="queued",
            queued_at="2023-10-01T00:00:00+00:00",
            status_reason_code="retry_enqueued",
            retry_of_run_id="sync_0987654321ba",
        )
        mock_retry.return_value = (mock_record, None)

        response = client.post("/api/v1/admin/sync-runs/sync_0987654321ba/retry", json={"reason": "test retry"})

        assert response.status_code == 202
        assert response.headers["Location"] == "/api/v1/admin/sync-runs/sync_1234567890ab"
        mock_retry.assert_called_once_with("sync_0987654321ba", "test retry")


def test_retry_sync_run_not_found():
    with patch("alm_integration_backend.api.routes.sync_runs.sync_run_store.retry") as mock_retry:
        mock_retry.return_value = (None, "RESOURCE_NOT_FOUND")

        response = client.post("/api/v1/admin/sync-runs/nonexistent/retry", json={"reason": "test retry"})

        assert response.status_code == 404
        assert response.json() == {"detail": "RESOURCE_NOT_FOUND"}
        mock_retry.assert_called_once_with("nonexistent", "test retry")


def test_retry_sync_run_not_retriable():
    with patch("alm_integration_backend.api.routes.sync_runs.sync_run_store.retry") as mock_retry:
        mock_retry.return_value = (None, "RUN_NOT_RETRIABLE")

        response = client.post("/api/v1/admin/sync-runs/sync_0987654321ba/retry", json={"reason": "test retry"})

        assert response.status_code == 409
        assert response.json() == {"detail": "RUN_NOT_RETRIABLE"}
        mock_retry.assert_called_once_with("sync_0987654321ba", "test retry")
