use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::adapters::{
    AdapterRegistry, build_default_registry, load_default_endpoint_configs_from_env,
};
use crate::security::ingestion_auth::{
    IngestionAuthRegistry, build_ingestion_auth_registry_from_endpoint_configs,
};
use crate::services::raw_ingestion::RawIngestionStore;
use crate::services::sync_runs::SyncRunStore;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db_pool: Option<PgPool>,
    pub adapter_registry: AdapterRegistry,
    pub ingestion_auth_registry: IngestionAuthRegistry,
    pub raw_ingestion_store: Arc<RwLock<RawIngestionStore>>,
    pub sync_run_store: Arc<RwLock<SyncRunStore>>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let endpoint_configs = load_default_endpoint_configs_from_env();
        Self {
            db_pool: None,
            adapter_registry: build_default_registry(),
            ingestion_auth_registry: build_ingestion_auth_registry_from_endpoint_configs(
                &endpoint_configs,
            ),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    pub fn with_pool(db_pool: PgPool) -> Self {
        let endpoint_configs = load_default_endpoint_configs_from_env();
        Self {
            db_pool: Some(db_pool),
            adapter_registry: build_default_registry(),
            ingestion_auth_registry: build_ingestion_auth_registry_from_endpoint_configs(
                &endpoint_configs,
            ),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_adapters(adapter_registry: AdapterRegistry) -> Self {
        Self {
            db_pool: None,
            adapter_registry,
            ingestion_auth_registry: IngestionAuthRegistry::new(),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_pool_and_adapters(db_pool: PgPool, adapter_registry: AdapterRegistry) -> Self {
        Self {
            db_pool: Some(db_pool),
            adapter_registry,
            ingestion_auth_registry: IngestionAuthRegistry::new(),
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_ingestion_auth(ingestion_auth_registry: IngestionAuthRegistry) -> Self {
        Self {
            db_pool: None,
            adapter_registry: build_default_registry(),
            ingestion_auth_registry,
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_adapters_and_ingestion_auth(
        adapter_registry: AdapterRegistry,
        ingestion_auth_registry: IngestionAuthRegistry,
    ) -> Self {
        Self {
            db_pool: None,
            adapter_registry,
            ingestion_auth_registry,
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }

    #[allow(dead_code)]
    pub fn with_pool_adapters_and_ingestion_auth(
        db_pool: PgPool,
        adapter_registry: AdapterRegistry,
        ingestion_auth_registry: IngestionAuthRegistry,
    ) -> Self {
        Self {
            db_pool: Some(db_pool),
            adapter_registry,
            ingestion_auth_registry,
            raw_ingestion_store: Arc::new(RwLock::new(RawIngestionStore::default())),
            sync_run_store: Arc::new(RwLock::new(SyncRunStore::default())),
        }
    }
}
