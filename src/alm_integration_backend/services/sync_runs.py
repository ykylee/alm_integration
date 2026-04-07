from __future__ import annotations

from dataclasses import asdict, dataclass, field
from datetime import UTC, datetime
from uuid import uuid4


TERMINAL_STATUSES = {"completed", "failed", "cancelled", "partially_completed"}
RETRIABLE_STATUSES = {"failed", "partially_completed", "cancelled"}


def _now() -> datetime:
    return datetime.now(UTC)


@dataclass
class SyncRunRecord:
    run_id: str
    source_system: str
    run_status: str
    queued_at: str
    reason: str | None = None
    status_reason_code: str | None = None
    started_at: str | None = None
    ended_at: str | None = None
    cancel_requested_at: str | None = None
    cancel_requested_by: str | None = None
    cancel_reason_code: str | None = None
    processed_count: int = 0
    success_count: int = 0
    failure_count: int = 0
    pending_count: int = 0
    retry_of_run_id: str | None = None
    scope: dict[str, list[str]] = field(default_factory=dict)

    def to_dict(self) -> dict[str, object]:
        return asdict(self)


class SyncRunStore:
    def __init__(self) -> None:
        self._runs: dict[str, SyncRunRecord] = {}

    def create(self, source_system: str, scope: dict[str, list[str]] | None, reason: str | None) -> SyncRunRecord:
        run_id = f"sync_{uuid4().hex[:12]}"
        record = SyncRunRecord(
            run_id=run_id,
            source_system=source_system,
            run_status="queued",
            queued_at=_now().isoformat(),
            reason=reason,
            status_reason_code="manual_run_requested",
            scope=scope or {},
        )
        self._runs[run_id] = record
        return record

    def list(self) -> list[SyncRunRecord]:
        return list(self._runs.values())

    def get(self, run_id: str) -> SyncRunRecord | None:
        return self._runs.get(run_id)

    def retry(self, run_id: str, reason: str | None) -> tuple[SyncRunRecord | None, str | None]:
        original = self.get(run_id)
        if original is None:
            return None, "RESOURCE_NOT_FOUND"
        if original.run_status not in RETRIABLE_STATUSES:
            return None, "RUN_NOT_RETRIABLE"
        retried = SyncRunRecord(
            run_id=f"sync_{uuid4().hex[:12]}",
            source_system=original.source_system,
            run_status="queued",
            queued_at=_now().isoformat(),
            reason=reason,
            status_reason_code="retry_enqueued",
            retry_of_run_id=run_id,
            scope=original.scope,
        )
        self._runs[retried.run_id] = retried
        return retried, None

    def request_cancel(
        self,
        run_id: str,
        requested_by: str,
        cancel_reason_code: str | None,
    ) -> tuple[SyncRunRecord | None, str | None]:
        run = self.get(run_id)
        if run is None:
            return None, "RESOURCE_NOT_FOUND"
        if run.cancel_requested_at is not None:
            return run, "RUN_ALREADY_CANCEL_REQUESTED"
        if run.run_status in TERMINAL_STATUSES:
            return run, "RUN_ALREADY_FINISHED"
        run.cancel_requested_at = _now().isoformat()
        run.cancel_requested_by = requested_by
        run.cancel_reason_code = cancel_reason_code
        run.status_reason_code = "cancel_requested"
        return run, None


sync_run_store = SyncRunStore()
