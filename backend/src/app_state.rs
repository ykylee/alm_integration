use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::adapters::{AdapterRegistry, build_default_registry};
use crate::services::raw_ingestion::RawIngestionStore;
use crate::services::sync_runs::SyncRunStore;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db_pool: Option<PgPool>,
    pub adapter_registry: AdapterRegistry,
    pub raw_ingestion_store: Arc<RwLock<RawIngestionStore>>,
    pub sync_run_store: Arc<RwLock<SyncRunStore>>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            db_pool: None,
            adapter_registry: build_default_registry(),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    pub fn with_pool(db_pool: PgPool) -> Self {
        Self {
            db_pool: Some(db_pool),
            adapter_registry: build_default_registry(),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_adapters(adapter_registry: AdapterRegistry) -> Self {
        Self {
            db_pool: None,
            adapter_registry,
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_pool_and_adapters(db_pool: PgPool, adapter_registry: AdapterRegistry) -> Self {
        Self {
            db_pool: Some(db_pool),
            adapter_registry,
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }
}
