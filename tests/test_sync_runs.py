from alm_integration_backend.services.sync_runs import SyncRunStore

def test_sync_run_retry_success():
    store = SyncRunStore()
    run = store.create(source_system="jira", scope={"project_keys": ["ALM"]}, reason="initial")
    run.run_status = "failed"

    retried_run, error = store.retry(run.run_id, reason="retry")

    assert error is None
    assert retried_run is not None
    assert retried_run.run_status == "queued"
    assert retried_run.retry_of_run_id == run.run_id
    assert retried_run.reason == "retry"
    assert retried_run.status_reason_code == "retry_enqueued"
    assert retried_run.source_system == "jira"
    assert retried_run.scope == {"project_keys": ["ALM"]}

def test_sync_run_retry_not_found():
    store = SyncRunStore()

    retried_run, error = store.retry("nonexistent_run_id", reason="retry")

    assert retried_run is None
    assert error == "RESOURCE_NOT_FOUND"

def test_sync_run_retry_not_retriable():
    store = SyncRunStore()
    run = store.create(source_system="jira", scope={"project_keys": ["ALM"]}, reason="initial")
    # Status is 'queued' by default, which is not retriable

    retried_run, error = store.retry(run.run_id, reason="retry")

    assert retried_run is None
    assert error == "RUN_NOT_RETRIABLE"

def test_sync_run_retry_cancelled():
    store = SyncRunStore()
    run = store.create(source_system="jira", scope=None, reason="initial")
    run.run_status = "cancelled"

    retried_run, error = store.retry(run.run_id, reason="retry")

    assert error is None
    assert retried_run is not None
    assert retried_run.run_status == "queued"
    assert retried_run.retry_of_run_id == run.run_id

def test_sync_run_retry_partially_completed():
    store = SyncRunStore()
    run = store.create(source_system="jira", scope=None, reason="initial")
    run.run_status = "partially_completed"

    retried_run, error = store.retry(run.run_id, reason="retry")

    assert error is None
    assert retried_run is not None
    assert retried_run.run_status == "queued"
    assert retried_run.retry_of_run_id == run.run_id
