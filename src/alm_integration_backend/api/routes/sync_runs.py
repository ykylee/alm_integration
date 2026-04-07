from __future__ import annotations

from dataclasses import asdict

from fastapi import APIRouter, Header, HTTPException, Response, status
from pydantic import BaseModel, Field

from alm_integration_backend.services.sync_runs import sync_run_store

router = APIRouter()


class SyncRunCreateRequest(BaseModel):
    source_system: str
    mode: str
    scope: dict[str, list[str]] = Field(default_factory=dict)
    reason: str | None = None


class SyncRunRetryRequest(BaseModel):
    reason: str | None = None


class SyncRunCancelRequest(BaseModel):
    reason: str | None = None
    cancel_reason_code: str | None = None


@router.post("", status_code=status.HTTP_202_ACCEPTED)
def create_sync_run(
    request: SyncRunCreateRequest,
    response: Response,
    x_idempotency_key: str | None = Header(default=None, alias="X-Idempotency-Key"),
) -> dict[str, object]:
    record = sync_run_store.create(
        source_system=request.source_system,
        scope=request.scope,
        reason=request.reason,
    )
    response.headers["X-Idempotency-Key"] = x_idempotency_key or record.run_id
    return {
        "request_id": record.run_id,
        "accepted": True,
        "run_id": record.run_id,
        "status": "queued",
        "status_reason_code": record.status_reason_code,
    }


@router.get("")
def list_sync_runs() -> dict[str, list[dict[str, object]]]:
    return {"items": [record.to_dict() for record in sync_run_store.list()]}


@router.get("/{run_id}")
def get_sync_run(run_id: str) -> dict[str, object]:
    record = sync_run_store.get(run_id)
    if record is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="RESOURCE_NOT_FOUND")
    return record.to_dict()


@router.post("/{run_id}/retry", status_code=status.HTTP_202_ACCEPTED)
def retry_sync_run(run_id: str, request: SyncRunRetryRequest, response: Response) -> dict[str, object]:
    record, error_code = sync_run_store.retry(run_id, request.reason)
    if error_code == "RESOURCE_NOT_FOUND":
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail=error_code)
    if error_code == "RUN_NOT_RETRIABLE":
        raise HTTPException(status_code=status.HTTP_409_CONFLICT, detail=error_code)
    assert record is not None
    response.headers["Location"] = f"/api/v1/admin/sync-runs/{record.run_id}"
    return {
        "request_id": record.run_id,
        "accepted": True,
        "run_id": record.run_id,
        "status": record.run_status,
        "status_reason_code": record.status_reason_code,
        "message": "retry run created",
    }


@router.post("/{run_id}/cancel")
def cancel_sync_run(
    run_id: str,
    request: SyncRunCancelRequest,
    response: Response,
    x_user_id: str | None = Header(default="system.admin", alias="X-User-Id"),
) -> dict[str, object]:
    record, error_code = sync_run_store.request_cancel(
        run_id=run_id,
        requested_by=x_user_id or "system.admin",
        cancel_reason_code=request.cancel_reason_code,
    )
    if error_code == "RESOURCE_NOT_FOUND":
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail=error_code)
    if error_code == "RUN_NOT_CANCELLABLE":
        raise HTTPException(status_code=status.HTTP_409_CONFLICT, detail=error_code)
    assert record is not None
    if error_code == "RUN_ALREADY_CANCEL_REQUESTED":
        response.status_code = status.HTTP_200_OK
        accepted = False
        message = "cancellation request already registered"
    elif error_code == "RUN_ALREADY_FINISHED":
        response.status_code = status.HTTP_200_OK
        accepted = False
        message = "run already finished; cancellation not applied"
    else:
        response.status_code = status.HTTP_202_ACCEPTED
        accepted = True
        message = "cancellation request registered"
    return {
        "request_id": run_id,
        "accepted": accepted,
        "run_id": record.run_id,
        "status": record.run_status,
        "status_reason_code": error_code.lower() if error_code else "cancel_requested",
        "cancel_requested_at": record.cancel_requested_at,
        "message": message,
    }
