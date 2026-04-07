use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use axum::body::Bytes;
use axum::http::{HeaderMap, Method};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::adapters::AdapterEndpointConfig;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Default)]
pub struct IngestionAuthRegistry {
    configs: HashMap<String, IngestionAuthConfig>,
}

#[derive(Clone)]
pub struct IngestionAuthConfig {
    pub shared_secret: String,
    pub allowed_skew_seconds: i64,
}

#[derive(Debug)]
pub enum IngestionAuthError {
    MissingHeader(&'static str),
    SourceSystemMismatch,
    InvalidTimestamp,
    TimestampExpired,
    InvalidSignature,
}

impl Display for IngestionAuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHeader(name) => write!(f, "missing required header: {name}"),
            Self::SourceSystemMismatch => write!(f, "source system header does not match body"),
            Self::InvalidTimestamp => write!(f, "invalid signature timestamp"),
            Self::TimestampExpired => write!(f, "signature timestamp is outside the allowed skew"),
            Self::InvalidSignature => write!(f, "invalid signature"),
        }
    }
}

impl std::error::Error for IngestionAuthError {}

impl IngestionAuthRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, source_system: impl Into<String>, config: IngestionAuthConfig) {
        self.configs.insert(source_system.into(), config);
    }

    pub fn get(&self, source_system: &str) -> Option<&IngestionAuthConfig> {
        self.configs.get(source_system)
    }
}

pub fn build_ingestion_auth_registry_from_endpoint_configs(
    endpoint_configs: &[AdapterEndpointConfig],
) -> IngestionAuthRegistry {
    let mut registry = IngestionAuthRegistry::new();

    for config in endpoint_configs {
        if config.enable_push {
            if let Some(shared_secret) = config.push_signing_secret.clone() {
                registry.register(
                    config.source_system.clone(),
                    IngestionAuthConfig {
                        shared_secret,
                        allowed_skew_seconds: 300,
                    },
                );
            }
        }
    }

    registry
}

pub fn verify_ingestion_request(
    registry: &IngestionAuthRegistry,
    source_system: &str,
    headers: &HeaderMap,
    method: &Method,
    path: &str,
    body: &Bytes,
    now: DateTime<Utc>,
) -> Result<(), IngestionAuthError> {
    let Some(config) = registry.get(source_system) else {
        return Ok(());
    };

    let header_source_system = read_header(headers, "x-source-system")?;
    if header_source_system != source_system {
        return Err(IngestionAuthError::SourceSystemMismatch);
    }

    let timestamp = read_header(headers, "x-signature-timestamp")?;
    let parsed_timestamp = parse_signature_timestamp(&timestamp)?;
    let skew = (now.timestamp() - parsed_timestamp.timestamp()).abs();
    if skew > config.allowed_skew_seconds {
        return Err(IngestionAuthError::TimestampExpired);
    }

    let provided_signature = read_header(headers, "x-signature")?;
    let expected_signature = sign_ingestion_payload(
        &config.shared_secret,
        &timestamp,
        method.as_str(),
        path,
        body,
    );

    if provided_signature != expected_signature {
        return Err(IngestionAuthError::InvalidSignature);
    }

    let idempotency_key = read_header(headers, "x-idempotency-key")?;
    if idempotency_key.is_empty() {
        return Err(IngestionAuthError::MissingHeader("x-idempotency-key"));
    }

    Ok(())
}

pub fn sign_ingestion_payload(
    shared_secret: &str,
    timestamp: &str,
    method: &str,
    path: &str,
    body: &[u8],
) -> String {
    let body_hash = hash_body(body);
    let payload = format!("{timestamp}\n{method}\n{path}\n{body_hash}");
    let mut mac =
        HmacSha256::new_from_slice(shared_secret.as_bytes()).expect("hmac key should be valid");
    mac.update(payload.as_bytes());
    format!("sha256={}", hex::encode(mac.finalize().into_bytes()))
}

fn read_header(headers: &HeaderMap, name: &'static str) -> Result<String, IngestionAuthError> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string())
        .ok_or(IngestionAuthError::MissingHeader(name))
}

fn parse_signature_timestamp(value: &str) -> Result<DateTime<Utc>, IngestionAuthError> {
    if let Ok(seconds) = value.parse::<i64>() {
        return DateTime::<Utc>::from_timestamp(seconds, 0)
            .ok_or(IngestionAuthError::InvalidTimestamp);
    }

    DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&Utc))
        .map_err(|_| IngestionAuthError::InvalidTimestamp)
}

fn hash_body(body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use axum::body::Bytes;
    use axum::http::{HeaderMap, HeaderValue, Method};
    use chrono::{TimeZone, Utc};

    use super::{
        IngestionAuthConfig, IngestionAuthRegistry, sign_ingestion_payload,
        verify_ingestion_request,
    };

    #[test]
    fn verify_ingestion_request_accepts_valid_signature() {
        let mut registry = IngestionAuthRegistry::new();
        registry.register(
            "jira",
            IngestionAuthConfig {
                shared_secret: "jira-webhook-secret".to_string(),
                allowed_skew_seconds: 300,
            },
        );
        let body = Bytes::from_static(br#"{"source_system":"jira"}"#);
        let timestamp = "2026-04-07T12:00:00Z";
        let signature =
            sign_ingestion_payload("jira-webhook-secret", timestamp, "POST", "/api/v1/ingestion/events", &body);
        let mut headers = HeaderMap::new();
        headers.insert("x-source-system", HeaderValue::from_static("jira"));
        headers.insert(
            "x-signature-timestamp",
            HeaderValue::from_static("2026-04-07T12:00:00Z"),
        );
        headers.insert(
            "x-signature",
            HeaderValue::from_str(&signature).expect("header"),
        );
        headers.insert("x-idempotency-key", HeaderValue::from_static("event-1"));

        let result = verify_ingestion_request(
            &registry,
            "jira",
            &headers,
            &Method::POST,
            "/api/v1/ingestion/events",
            &body,
            Utc.with_ymd_and_hms(2026, 4, 7, 12, 1, 0).unwrap(),
        );

        assert!(result.is_ok());
    }
}
