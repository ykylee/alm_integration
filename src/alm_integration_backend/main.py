from fastapi import FastAPI

from alm_integration_backend.api.router import api_router
from alm_integration_backend.config import settings


app = FastAPI(
    title=settings.app_name,
    version="0.1.0",
)
app.include_router(api_router, prefix=settings.api_prefix)
