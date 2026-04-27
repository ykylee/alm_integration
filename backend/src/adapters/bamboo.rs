use std::sync::Arc;

use async_trait::async_trait;

use super::{
    AdapterError, AdapterHttpRequest, HttpTransport, PullAdapterRequest, PullSourceAdapter,
    PushAdapterRequest, PushEventAdapter,
};
use crate::services::pull_sync::PullRecordInput;
use crate::services::raw_ingestion::CreateRawIngestionEventInput;

pub struct BambooPullAdapter {
    transport: Arc<dyn HttpTransport>,
    base_url: Option<String>,
    bearer_token: Option<String>,
}

impl BambooPullAdapter {
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
            AdapterError::ExternalCall("bamboo base url is not configured".to_string())
        })?;
        let plan_key = request
            .scope
            .get("plan_key")
            .and_then(|value| value.as_str())
            .unwrap_or("ALM-PLAN");
        let mode = request.mode.as_str();

        Ok(format!(
            "{base_url}/rest/api/latest/result/{plan_key}.json?expand=results.result&mode={mode}"
        ))
    }

    fn parse_response(value: serde_json::Value) -> Result<Vec<PullRecordInput>, AdapterError> {
        let results = value
            .get("results")
            .and_then(|results| results.get("result"))
            .and_then(|result| result.as_array())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("bamboo results.result array is missing".to_string())
            })?;

        results
            .iter()
            .map(|result| {
                let build_result_key = result
                    .get("buildResultKey")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("bamboo buildResultKey is missing".to_string())
                    })?;
                let build_number = result
                    .get("buildNumber")
                    .and_then(|value| value.as_i64())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("bamboo buildNumber is missing".to_string())
                    })?;
                let completed = result
                    .get("buildCompletedTime")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload(
                            "bamboo buildCompletedTime is missing".to_string(),
                        )
                    })?;

                Ok(PullRecordInput {
                    source_object_type: "build_result".to_string(),
                    source_object_id: build_result_key.to_string(),
                    source_event_key: format!("bamboo-build-{build_result_key}-{build_number}"),
                    source_version: Some(build_number.to_string()),
                    source_updated_at: Some(completed.to_string()),
                    payload: result.clone(),
                })
            })
            .collect()
    }
}

#[async_trait]
impl PullSourceAdapter for BambooPullAdapter {
    fn source_system(&self) -> &'static str {
        "bamboo"
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

pub struct BambooPushAdapter;

impl PushEventAdapter for BambooPushAdapter {
    fn source_system(&self) -> &'static str {
        "bamboo"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        let build_result_key = request
            .payload
            .get("buildResultKey")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("bamboo buildResultKey is missing".to_string())
            })?;
        let build_number = request
            .payload
            .get("buildNumber")
            .and_then(|value| value.as_i64())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("bamboo buildNumber is missing".to_string())
            })?;
        let completed = request
            .payload
            .get("buildCompletedTime")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("bamboo buildCompletedTime is missing".to_string())
            })?;

        Ok(CreateRawIngestionEventInput {
            source_system: "bamboo".to_string(),
            source_object_type: "build_result".to_string(),
            source_object_id: build_result_key.to_string(),
            source_event_key: format!("bamboo-build-{build_result_key}-{build_number}"),
            source_version: Some(build_number.to_string()),
            source_updated_at: Some(completed.to_string()),
            payload: request.payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::{BambooPullAdapter, BambooPushAdapter};
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
    async fn bamboo_pull_adapter_parses_result_response() {
        let transport = Arc::new(StaticTransport {
            payload: serde_json::json!({
                "results": {
                    "result": [
                        {
                            "buildResultKey": "ALM-PLAN-101",
                            "buildNumber": 101,
                            "buildCompletedTime": "2026-04-07T09:30:00Z"
                        }
                    ]
                }
            }),
        });
        let adapter = BambooPullAdapter::new_for_test(
            transport,
            "https://bamboo.example.com".to_string(),
            None,
        );

        let records = adapter
            .pull(PullAdapterRequest {
                mode: "incremental".to_string(),
                scope: serde_json::json!({"plan_key": "ALM-PLAN"}),
            })
            .await
            .expect("bamboo pull should parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source_object_type, "build_result");
    }

    #[test]
    fn bamboo_push_adapter_maps_webhook_payload() {
        let adapter = BambooPushAdapter;

        let record = adapter
            .adapt(PushAdapterRequest {
                source_system: "bamboo".to_string(),
                source_object_type: "build_result".to_string(),
                source_object_id: "ALM-PLAN-101".to_string(),
                source_event_key: "ignored".to_string(),
                source_version: None,
                source_updated_at: None,
                payload: serde_json::json!({
                    "buildResultKey": "ALM-PLAN-101",
                    "buildNumber": 101,
                    "buildCompletedTime": "2026-04-07T09:30:00Z"
                }),
            })
            .expect("bamboo webhook should map");

        assert_eq!(record.source_system, "bamboo");
        assert_eq!(record.source_object_type, "build_result");
    }
}
