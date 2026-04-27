import pytest
from alm_integration_backend.services.sync_runs import SyncRunStore, SyncRunRecord

@pytest.fixture
def store() -> SyncRunStore:
    return SyncRunStore()

def test_create(store: SyncRunStore) -> None:
    record = store.create(source_system="jira", scope={"projects": ["PROJ1"]}, reason="manual_sync")

    assert record.run_id.startswith("sync_")
    assert record.source_system == "jira"
    assert record.run_status == "queued"
    assert record.reason == "manual_sync"
    assert record.status_reason_code == "manual_run_requested"
    assert record.scope == {"projects": ["PROJ1"]}
    assert record.queued_at is not None

def test_create_empty_scope(store: SyncRunStore) -> None:
    record = store.create(source_system="jira", scope=None, reason=None)
    assert record.scope == {}
    assert record.reason is None

def test_list(store: SyncRunStore) -> None:
    record1 = store.create("jira", None, None)
    record2 = store.create("confluence", None, None)

    runs = store.list()
    assert len(runs) == 2
    assert record1 in runs
    assert record2 in runs

def test_get(store: SyncRunStore) -> None:
    record = store.create("jira", None, None)

    fetched = store.get(record.run_id)
    assert fetched == record

    not_found = store.get("nonexistent")
    assert not_found is None

def test_retry_success(store: SyncRunStore) -> None:
    original = store.create("jira", {"projects": ["A"]}, "initial")
    # Manually modify status to make it retriable
    original.run_status = "failed"

    retried, err = store.retry(original.run_id, reason="retry_because_failed")

    assert err is None
    assert retried is not None
    assert retried.run_id != original.run_id
    assert retried.run_id.startswith("sync_")
    assert retried.retry_of_run_id == original.run_id
    assert retried.source_system == "jira"
    assert retried.run_status == "queued"
    assert retried.reason == "retry_because_failed"
    assert retried.status_reason_code == "retry_enqueued"
    assert retried.scope == {"projects": ["A"]}

def test_retry_not_found(store: SyncRunStore) -> None:
    retried, err = store.retry("nonexistent", reason=None)

    assert retried is None
    assert err == "RESOURCE_NOT_FOUND"

def test_retry_not_retriable(store: SyncRunStore) -> None:
    original = store.create("jira", None, None)
    # Status is 'queued', not in RETRIABLE_STATUSES

    retried, err = store.retry(original.run_id, reason=None)

    assert retried is None
    assert err == "RUN_NOT_RETRIABLE"

def test_request_cancel_success(store: SyncRunStore) -> None:
    original = store.create("jira", None, None)

    canceled, err = store.request_cancel(original.run_id, requested_by="user1", cancel_reason_code="user_aborted")

    assert err is None
    assert canceled is not None
    assert canceled.cancel_requested_at is not None
    assert canceled.cancel_requested_by == "user1"
    assert canceled.cancel_reason_code == "user_aborted"
    assert canceled.status_reason_code == "cancel_requested"

def test_request_cancel_not_found(store: SyncRunStore) -> None:
    canceled, err = store.request_cancel("nonexistent", "user1", None)

    assert canceled is None
    assert err == "RESOURCE_NOT_FOUND"

def test_request_cancel_already_requested(store: SyncRunStore) -> None:
    original = store.create("jira", None, None)
    store.request_cancel(original.run_id, "user1", None)

    # Try again
    canceled, err = store.request_cancel(original.run_id, "user2", None)

    assert err == "RUN_ALREADY_CANCEL_REQUESTED"
    assert canceled == original

def test_request_cancel_already_finished(store: SyncRunStore) -> None:
    original = store.create("jira", None, None)
    # Manually put in terminal state
    original.run_status = "completed"

    canceled, err = store.request_cancel(original.run_id, "user1", None)

    assert err == "RUN_ALREADY_FINISHED"
    assert canceled == original
