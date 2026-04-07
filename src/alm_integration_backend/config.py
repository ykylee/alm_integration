from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    app_name: str = "ALM Integration Backend"
    app_env: str = "local"
    api_prefix: str = "/api/v1"
    database_url: str = "postgresql+psycopg://alm:alm@localhost:5432/alm_integration"

    model_config = SettingsConfigDict(
        env_prefix="ALM_",
        env_file=".env",
        extra="ignore",
    )


settings = Settings()
