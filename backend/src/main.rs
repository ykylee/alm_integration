mod adapters;
mod app_state;
mod config;
mod db;
mod http;
mod services;

use std::net::SocketAddr;

use adapters::config_loader::DbAdapterConfigLoader;
use app_state::AppState;
use axum::Router;
use config::Settings;
use db::pool::{connect, run_migrations};
use http::router::build_router;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let settings = Settings::from_env()?;
    let db_pool = connect(&settings).await?;
    if settings.auto_apply_migrations {
        run_migrations(&db_pool).await?;
    }

    let state = build_state(db_pool).await;
    let app: Router = build_router(state);
    let address: SocketAddr = settings.bind_address.parse()?;
    let listener = TcpListener::bind(address).await?;

    info!("backend listening on {}", address);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn build_state(db_pool: sqlx::PgPool) -> AppState {
    let loader = DbAdapterConfigLoader::new(db_pool.clone());

    match loader.load_registry().await {
        Ok(Some(adapter_registry)) => AppState::with_pool_and_adapters(db_pool, adapter_registry),
        Ok(None) => AppState::with_pool(db_pool),
        Err(error) => {
            warn!(
                "failed to load adapter registry from database, fallback to default registry: {}",
                error
            );
            AppState::with_pool(db_pool)
        }
    }
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with_target(false)
        .compact()
        .init();
}
