import pytest

from alm_integration_backend.services.sync_runs import SyncRunStore, TERMINAL_STATUSES


@pytest.fixture
def store() -> SyncRunStore:
    return SyncRunStore()


def test_request_cancel_success(store: SyncRunStore) -> None:
    # Arrange
    run = store.create(source_system="test", scope=None, reason=None)

    # Act
    cancelled_run, error = store.request_cancel(
        run_id=run.run_id,
        requested_by="user_123",
        cancel_reason_code="test_cancel"
    )

    # Assert
    assert error is None
    assert cancelled_run is not None
    assert cancelled_run.run_id == run.run_id
    assert cancelled_run.cancel_requested_at is not None
    assert cancelled_run.cancel_requested_by == "user_123"
    assert cancelled_run.cancel_reason_code == "test_cancel"
    assert cancelled_run.status_reason_code == "cancel_requested"


def test_request_cancel_not_found(store: SyncRunStore) -> None:
    # Act
    run, error = store.request_cancel(
        run_id="nonexistent_id",
        requested_by="user_123",
        cancel_reason_code="test_cancel"
    )

    # Assert
    assert run is None
    assert error == "RESOURCE_NOT_FOUND"


def test_request_cancel_already_cancel_requested(store: SyncRunStore) -> None:
    # Arrange
    run = store.create(source_system="test", scope=None, reason=None)
    store.request_cancel(
        run_id=run.run_id,
        requested_by="user_123",
        cancel_reason_code="test_cancel"
    )

    # Act - Try to cancel again
    cancelled_run, error = store.request_cancel(
        run_id=run.run_id,
        requested_by="user_456",
        cancel_reason_code="another_cancel"
    )

    # Assert
    assert cancelled_run is not None
    assert error == "RUN_ALREADY_CANCEL_REQUESTED"
    # Ensure original cancellation details are kept
    assert cancelled_run.cancel_requested_by == "user_123"
    assert cancelled_run.cancel_reason_code == "test_cancel"


@pytest.mark.parametrize("terminal_status", TERMINAL_STATUSES)
def test_request_cancel_terminal_status(store: SyncRunStore, terminal_status: str) -> None:
    # Arrange
    run = store.create(source_system="test", scope=None, reason=None)
    run.run_status = terminal_status

    # Act
    cancelled_run, error = store.request_cancel(
        run_id=run.run_id,
        requested_by="user_123",
        cancel_reason_code="test_cancel"
    )

    # Assert
    assert cancelled_run is not None
    assert error == "RUN_ALREADY_FINISHED"
    assert cancelled_run.cancel_requested_at is None
