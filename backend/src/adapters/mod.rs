pub mod bamboo;
pub mod bitbucket;
pub mod config_loader;
pub mod confluence;
pub mod jira;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::services::pull_sync::PullRecordInput;
use crate::services::raw_ingestion::CreateRawIngestionEventInput;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PullAdapterRequest {
    pub mode: String,
    pub scope: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PushAdapterRequest {
    pub source_system: String,
    pub source_object_type: String,
    pub source_object_id: String,
    pub source_event_key: String,
    pub source_version: Option<String>,
    pub source_updated_at: Option<String>,
    pub payload: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AdapterHttpRequest {
    pub url: String,
    pub bearer_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterEndpointConfig {
    pub source_system: String,
    pub base_url: Option<String>,
    pub bearer_token: Option<String>,
    pub push_signing_secret: Option<String>,
    pub enable_pull: bool,
    pub enable_push: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AdapterError {
    UnsupportedSourceSystem(String),
    InvalidPayload(String),
    ExternalCall(String),
}

impl Display for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedSourceSystem(source_system) => {
                write!(f, "unsupported source system: {source_system}")
            }
            Self::InvalidPayload(message) => write!(f, "invalid payload: {message}"),
            Self::ExternalCall(message) => write!(f, "external call failed: {message}"),
        }
    }
}

impl std::error::Error for AdapterError {}

#[async_trait]
pub trait HttpTransport: Send + Sync {
    async fn get_json(
        &self,
        request: AdapterHttpRequest,
    ) -> Result<serde_json::Value, AdapterError>;
}

pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl HttpTransport for ReqwestTransport {
    async fn get_json(
        &self,
        request: AdapterHttpRequest,
    ) -> Result<serde_json::Value, AdapterError> {
        let mut headers = HeaderMap::new();
        if let Some(token) = request.bearer_token {
            let header_value = HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|error| AdapterError::ExternalCall(error.to_string()))?;
            headers.insert(AUTHORIZATION, header_value);
        }

        self.client
            .get(&request.url)
            .headers(headers)
            .send()
            .await
            .map_err(|error| AdapterError::ExternalCall(error.to_string()))?
            .error_for_status()
            .map_err(|error| AdapterError::ExternalCall(error.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|error| AdapterError::ExternalCall(error.to_string()))
    }
}

#[async_trait]
pub trait PullSourceAdapter: Send + Sync {
    fn source_system(&self) -> &'static str;

    async fn pull(&self, request: PullAdapterRequest)
    -> Result<Vec<PullRecordInput>, AdapterError>;
}

pub trait PushEventAdapter: Send + Sync {
    fn source_system(&self) -> &'static str;

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError>;
}

#[derive(Clone, Default)]
pub struct AdapterRegistry {
    pull_adapters: HashMap<String, Arc<dyn PullSourceAdapter>>,
    push_adapters: HashMap<String, Arc<dyn PushEventAdapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_pull_adapter(
        &mut self,
        adapter: Arc<dyn PullSourceAdapter>,
    ) -> Option<Arc<dyn PullSourceAdapter>> {
        self.pull_adapters
            .insert(adapter.source_system().to_string(), adapter)
    }

    pub fn register_push_adapter(
        &mut self,
        adapter: Arc<dyn PushEventAdapter>,
    ) -> Option<Arc<dyn PushEventAdapter>> {
        self.push_adapters
            .insert(adapter.source_system().to_string(), adapter)
    }

    pub fn get_pull_adapter(&self, source_system: &str) -> Option<Arc<dyn PullSourceAdapter>> {
        self.pull_adapters.get(source_system).cloned()
    }

    pub fn get_push_adapter(&self, source_system: &str) -> Option<Arc<dyn PushEventAdapter>> {
        self.push_adapters.get(source_system).cloned()
    }
}

pub fn build_default_registry() -> AdapterRegistry {
    let transport: Arc<dyn HttpTransport> = Arc::new(ReqwestTransport::new());
    build_registry_from_endpoint_configs(&load_default_endpoint_configs_from_env(), transport)
        .expect("default adapter endpoint configs should be supported")
}

pub fn build_registry_from_endpoint_configs(
    endpoint_configs: &[AdapterEndpointConfig],
    transport: Arc<dyn HttpTransport>,
) -> Result<AdapterRegistry, AdapterError> {
    let mut registry = AdapterRegistry::new();

    for config in endpoint_configs {
        if config.enable_pull {
            let pull_adapter = build_pull_adapter(config, transport.clone())?;
            registry.register_pull_adapter(pull_adapter);
        }

        if config.enable_push {
            let push_adapter = build_push_adapter(&config.source_system)?;
            registry.register_push_adapter(push_adapter);
        }
    }

    Ok(registry)
}

fn build_pull_adapter(
    config: &AdapterEndpointConfig,
    transport: Arc<dyn HttpTransport>,
) -> Result<Arc<dyn PullSourceAdapter>, AdapterError> {
    match config.source_system.as_str() {
        "jira" => Ok(Arc::new(jira::JiraPullAdapter::new(
            transport,
            config.base_url.clone(),
            config.bearer_token.clone(),
        ))),
        "bitbucket" => Ok(Arc::new(bitbucket::BitbucketPullAdapter::new(
            transport,
            config.base_url.clone(),
            config.bearer_token.clone(),
        ))),
        "bamboo" => Ok(Arc::new(bamboo::BambooPullAdapter::new(
            transport,
            config.base_url.clone(),
            config.bearer_token.clone(),
        ))),
        "confluence" => Ok(Arc::new(confluence::ConfluencePullAdapter::new(
            transport,
            config.base_url.clone(),
            config.bearer_token.clone(),
        ))),
        source_system => Err(AdapterError::UnsupportedSourceSystem(
            source_system.to_string(),
        )),
    }
}

fn build_push_adapter(source_system: &str) -> Result<Arc<dyn PushEventAdapter>, AdapterError> {
    match source_system {
        "jira" => Ok(Arc::new(jira::JiraPushAdapter)),
        "bitbucket" => Ok(Arc::new(bitbucket::BitbucketPushAdapter)),
        "bamboo" => Ok(Arc::new(bamboo::BambooPushAdapter)),
        "confluence" => Ok(Arc::new(confluence::ConfluencePushAdapter)),
        source_system => Err(AdapterError::UnsupportedSourceSystem(
            source_system.to_string(),
        )),
    }
}

pub fn load_default_endpoint_configs_from_env() -> Vec<AdapterEndpointConfig> {
    vec![
        AdapterEndpointConfig {
            source_system: "jira".to_string(),
            base_url: std::env::var("ALM_JIRA_BASE_URL").ok(),
            bearer_token: std::env::var("ALM_JIRA_TOKEN").ok(),
            push_signing_secret: std::env::var("ALM_JIRA_WEBHOOK_SECRET")
                .ok()
                .or_else(|| std::env::var("ALM_JIRA_TOKEN").ok()),
            enable_pull: true,
            enable_push: true,
        },
        AdapterEndpointConfig {
            source_system: "bitbucket".to_string(),
            base_url: std::env::var("ALM_BITBUCKET_BASE_URL").ok(),
            bearer_token: std::env::var("ALM_BITBUCKET_TOKEN").ok(),
            push_signing_secret: std::env::var("ALM_BITBUCKET_WEBHOOK_SECRET")
                .ok()
                .or_else(|| std::env::var("ALM_BITBUCKET_TOKEN").ok()),
            enable_pull: true,
            enable_push: true,
        },
        AdapterEndpointConfig {
            source_system: "bamboo".to_string(),
            base_url: std::env::var("ALM_BAMBOO_BASE_URL").ok(),
            bearer_token: std::env::var("ALM_BAMBOO_TOKEN").ok(),
            push_signing_secret: std::env::var("ALM_BAMBOO_WEBHOOK_SECRET")
                .ok()
                .or_else(|| std::env::var("ALM_BAMBOO_TOKEN").ok()),
            enable_pull: true,
            enable_push: true,
        },
        AdapterEndpointConfig {
            source_system: "confluence".to_string(),
            base_url: std::env::var("ALM_CONFLUENCE_BASE_URL").ok(),
            bearer_token: std::env::var("ALM_CONFLUENCE_TOKEN").ok(),
            push_signing_secret: std::env::var("ALM_CONFLUENCE_WEBHOOK_SECRET")
                .ok()
                .or_else(|| std::env::var("ALM_CONFLUENCE_TOKEN").ok()),
            enable_pull: true,
            enable_push: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::{
        AdapterEndpointConfig, AdapterError, AdapterHttpRequest, AdapterRegistry, HttpTransport,
        build_default_registry, build_registry_from_endpoint_configs,
    };

    struct NullTransport;

    #[async_trait]
    impl HttpTransport for NullTransport {
        async fn get_json(
            &self,
            _request: AdapterHttpRequest,
        ) -> Result<serde_json::Value, AdapterError> {
            Ok(serde_json::json!({}))
        }
    }

    #[test]
    fn registry_resolves_registered_pull_and_push_adapters() {
        let registry = build_default_registry();

        assert!(registry.get_pull_adapter("jira").is_some());
        assert!(registry.get_pull_adapter("bitbucket").is_some());
        assert!(registry.get_pull_adapter("bamboo").is_some());
        assert!(registry.get_pull_adapter("confluence").is_some());
        assert!(registry.get_push_adapter("jira").is_some());
        assert!(registry.get_push_adapter("bitbucket").is_some());
        assert!(registry.get_push_adapter("bamboo").is_some());
        assert!(registry.get_push_adapter("confluence").is_some());
        assert!(registry.get_pull_adapter("gitlab").is_none());
        assert!(registry.get_push_adapter("gitlab").is_none());
    }

    #[test]
    fn registry_can_be_created_empty() {
        let registry = AdapterRegistry::new();

        assert!(registry.get_pull_adapter("jira").is_none());
        assert!(registry.get_push_adapter("jira").is_none());
    }

    #[test]
    fn registry_builder_registers_enabled_capabilities_from_endpoint_configs() {
        let transport: Arc<dyn HttpTransport> = Arc::new(NullTransport);
        let configs = vec![
            AdapterEndpointConfig {
                source_system: "jira".to_string(),
                base_url: Some("https://jira.example.com".to_string()),
                bearer_token: Some("jira-token".to_string()),
                push_signing_secret: Some("jira-signing-secret".to_string()),
                enable_pull: true,
                enable_push: true,
            },
            AdapterEndpointConfig {
                source_system: "bamboo".to_string(),
                base_url: Some("https://bamboo.example.com".to_string()),
                bearer_token: None,
                push_signing_secret: None,
                enable_pull: true,
                enable_push: false,
            },
        ];

        let registry =
            build_registry_from_endpoint_configs(&configs, transport).expect("registry builds");

        assert!(registry.get_pull_adapter("jira").is_some());
        assert!(registry.get_push_adapter("jira").is_some());
        assert!(registry.get_pull_adapter("bamboo").is_some());
        assert!(registry.get_push_adapter("bamboo").is_none());
    }

    #[test]
    fn registry_builder_rejects_unsupported_source_system() {
        let transport: Arc<dyn HttpTransport> = Arc::new(NullTransport);
        let configs = vec![AdapterEndpointConfig {
            source_system: "gitlab".to_string(),
            base_url: Some("https://gitlab.example.com".to_string()),
            bearer_token: Some("gitlab-token".to_string()),
            push_signing_secret: Some("gitlab-signing-secret".to_string()),
            enable_pull: true,
            enable_push: true,
        }];

        let error = match build_registry_from_endpoint_configs(&configs, transport) {
            Ok(_) => panic!("unsupported source system should fail"),
            Err(error) => error,
        };

        match error {
            AdapterError::UnsupportedSourceSystem(source_system) => {
                assert_eq!(source_system, "gitlab")
            }
            other => panic!("unexpected error: {other}"),
        }
    }
}
