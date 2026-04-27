use std::sync::Arc;

use async_trait::async_trait;

use super::{
    AdapterError, AdapterHttpRequest, HttpTransport, PullAdapterRequest, PullSourceAdapter,
    PushAdapterRequest, PushEventAdapter,
};
use crate::services::pull_sync::PullRecordInput;
use crate::services::raw_ingestion::CreateRawIngestionEventInput;

pub struct ConfluencePullAdapter {
    transport: Arc<dyn HttpTransport>,
    base_url: Option<String>,
    bearer_token: Option<String>,
}

impl ConfluencePullAdapter {
    pub fn new(
        transport: Arc<dyn HttpTransport>,
        base_url: Option<String>,
        bearer_token: Option<String>,
    ) -> Self {
        Self {
            transport,
            base_url,
            bearer_token,
        }
    }

    #[cfg(test)]
    pub fn new_for_test(
        transport: Arc<dyn HttpTransport>,
        base_url: String,
        bearer_token: Option<String>,
    ) -> Self {
        Self::new(transport, Some(base_url), bearer_token)
    }

    fn build_pull_url(&self, request: &PullAdapterRequest) -> Result<String, AdapterError> {
        let base_url = self.base_url.as_ref().ok_or_else(|| {
            AdapterError::ExternalCall("confluence base url is not configured".to_string())
        })?;
        let space_key = request
            .scope
            .get("space_key")
            .and_then(|value| value.as_str())
            .unwrap_or("ALM");
        let mode = request.mode.as_str();

        Ok(format!(
            "{base_url}/rest/api/content/search?cql=space={space_key}&expand=version&mode={mode}"
        ))
    }

    fn parse_response(value: serde_json::Value) -> Result<Vec<PullRecordInput>, AdapterError> {
        let results = value
            .get("results")
            .and_then(|items| items.as_array())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("confluence results array is missing".to_string())
            })?;

        results
            .iter()
            .map(|item| {
                let id = item
                    .get("id")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("confluence id is missing".to_string())
                    })?;
                let title = item
                    .get("title")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("confluence title is missing".to_string())
                    })?;
                let updated = item
                    .get("version")
                    .and_then(|version| version.get("when"))
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload(
                            "confluence version.when is missing".to_string(),
                        )
                    })?;

                Ok(PullRecordInput {
                    source_object_type: "page".to_string(),
                    source_object_id: id.to_string(),
                    source_event_key: format!("confluence-page-{id}-{updated}"),
                    source_version: None,
                    source_updated_at: Some(updated.to_string()),
                    payload: serde_json::json!({
                        "id": id,
                        "title": title,
                        "version": item.get("version").cloned().unwrap_or_default()
                    }),
                })
            })
            .collect()
    }
}

#[async_trait]
impl PullSourceAdapter for ConfluencePullAdapter {
    fn source_system(&self) -> &'static str {
        "confluence"
    }

    async fn pull(
        &self,
        request: PullAdapterRequest,
    ) -> Result<Vec<PullRecordInput>, AdapterError> {
        let url = self.build_pull_url(&request)?;
        let response = self
            .transport
            .get_json(AdapterHttpRequest {
                url,
                bearer_token: self.bearer_token.clone(),
            })
            .await?;

        Self::parse_response(response)
    }
}

pub struct ConfluencePushAdapter;

impl PushEventAdapter for ConfluencePushAdapter {
    fn source_system(&self) -> &'static str {
        "confluence"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        let page = request.payload.get("page").ok_or_else(|| {
            AdapterError::InvalidPayload("confluence webhook page is missing".to_string())
        })?;
        let id = page
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("confluence webhook page.id is missing".to_string())
            })?;
        let updated = page
            .get("version")
            .and_then(|version| version.get("when"))
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload(
                    "confluence webhook page.version.when is missing".to_string(),
                )
            })?;

        Ok(CreateRawIngestionEventInput {
            source_system: "confluence".to_string(),
            source_object_type: "page".to_string(),
            source_object_id: id.to_string(),
            source_event_key: format!("confluence-page-{id}-{updated}"),
            source_version: None,
            source_updated_at: Some(updated.to_string()),
            payload: request.payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::{ConfluencePullAdapter, ConfluencePushAdapter};
    use crate::adapters::{
        AdapterError, AdapterHttpRequest, HttpTransport, PullAdapterRequest, PullSourceAdapter,
        PushAdapterRequest, PushEventAdapter,
    };

    struct StaticTransport {
        payload: serde_json::Value,
    }

    #[async_trait]
    impl HttpTransport for StaticTransport {
        async fn get_json(
            &self,
            _request: AdapterHttpRequest,
        ) -> Result<serde_json::Value, AdapterError> {
            Ok(self.payload.clone())
        }
    }

    #[tokio::test]
    async fn confluence_pull_adapter_parses_page_response() {
        let transport = Arc::new(StaticTransport {
            payload: serde_json::json!({
                "results": [
                    {
                        "id": "12345",
                        "title": "Release Plan",
                        "version": {
                            "when": "2026-04-07T10:00:00Z"
                        }
                    }
                ]
            }),
        });
        let adapter = ConfluencePullAdapter::new_for_test(
            transport,
            "https://confluence.example.com".to_string(),
            None,
        );

        let records = adapter
            .pull(PullAdapterRequest {
                mode: "incremental".to_string(),
                scope: serde_json::json!({"space_key": "ALM"}),
            })
            .await
            .expect("confluence pull should parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source_object_type, "page");
    }

    #[test]
    fn confluence_push_adapter_maps_webhook_payload() {
        let adapter = ConfluencePushAdapter;

        let record = adapter
            .adapt(PushAdapterRequest {
                source_system: "confluence".to_string(),
                source_object_type: "page".to_string(),
                source_object_id: "12345".to_string(),
                source_event_key: "ignored".to_string(),
                source_version: None,
                source_updated_at: None,
                payload: serde_json::json!({
                    "page": {
                        "id": "12345",
                        "version": {
                            "when": "2026-04-07T10:00:00Z"
                        }
                    }
                }),
            })
            .expect("confluence webhook should map");

        assert_eq!(record.source_system, "confluence");
        assert_eq!(record.source_object_type, "page");
    }
}
