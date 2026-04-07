use anyhow::{Context, Result};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Settings;

use super::migrations::MIGRATOR;

pub async fn connect(settings: &Settings) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(settings.database_max_connections)
        .connect(&settings.database_url)
        .await
        .with_context(|| "failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .with_context(|| "failed to run database migrations")
}
