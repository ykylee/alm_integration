from fastapi import APIRouter

from alm_integration_backend.api.routes.health import router as health_router
from alm_integration_backend.api.routes.sync_runs import router as sync_runs_router

api_router = APIRouter()
api_router.include_router(health_router, tags=["health"])
api_router.include_router(sync_runs_router, prefix="/admin/sync-runs", tags=["sync-runs"])
