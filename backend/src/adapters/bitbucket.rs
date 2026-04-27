use std::sync::Arc;

use async_trait::async_trait;
use chrono::{TimeZone, Utc};

use super::{
    AdapterError, AdapterHttpRequest, HttpTransport, PullAdapterRequest, PullSourceAdapter,
    PushAdapterRequest, PushEventAdapter,
};
use crate::services::pull_sync::PullRecordInput;
use crate::services::raw_ingestion::CreateRawIngestionEventInput;

pub struct BitbucketPullAdapter {
    transport: Arc<dyn HttpTransport>,
    base_url: Option<String>,
    bearer_token: Option<String>,
}

impl BitbucketPullAdapter {
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
            AdapterError::ExternalCall("bitbucket base url is not configured".to_string())
        })?;
        let project_key = request
            .scope
            .get("project_key")
            .and_then(|value| value.as_str())
            .unwrap_or("ALM");
        let repository = request
            .scope
            .get("repository_slug")
            .and_then(|value| value.as_str())
            .unwrap_or("platform");

        Ok(format!(
            "{base_url}/rest/api/1.0/projects/{project_key}/repos/{repository}/pull-requests?state=ALL"
        ))
    }

    fn parse_response(value: serde_json::Value) -> Result<Vec<PullRecordInput>, AdapterError> {
        let values = value
            .get("values")
            .and_then(|items| items.as_array())
            .ok_or_else(|| {
                AdapterError::InvalidPayload(
                    "bitbucket pull request values are missing".to_string(),
                )
            })?;

        values
            .iter()
            .map(|item| {
                let repository = item
                    .get("fromRef")
                    .and_then(|from_ref| from_ref.get("repository"))
                    .and_then(|repo| repo.get("slug"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                let id = item
                    .get("id")
                    .and_then(|value| value.as_i64())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload(
                            "bitbucket pull request id is missing".to_string(),
                        )
                    })?;
                let updated = item
                    .get("updatedDate")
                    .and_then(|value| value.as_i64())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload(
                            "bitbucket pull request updatedDate is missing".to_string(),
                        )
                    })?;

                Ok(PullRecordInput {
                    source_object_type: "pull_request".to_string(),
                    source_object_id: format!("{repository}#{id}"),
                    source_event_key: format!("bitbucket-pr-{id}-{updated}"),
                    source_version: Some(updated.to_string()),
                    source_updated_at: Some(epoch_millis_to_rfc3339(updated)?),
                    payload: item.clone(),
                })
            })
            .collect()
    }
}

#[async_trait]
impl PullSourceAdapter for BitbucketPullAdapter {
    fn source_system(&self) -> &'static str {
        "bitbucket"
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

pub struct BitbucketPushAdapter;

impl PushEventAdapter for BitbucketPushAdapter {
    fn source_system(&self) -> &'static str {
        "bitbucket"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        let pull_request = request.payload.get("pullrequest").ok_or_else(|| {
            AdapterError::InvalidPayload("bitbucket webhook pullrequest is missing".to_string())
        })?;
        let id = pull_request
            .get("id")
            .and_then(|value| value.as_i64())
            .ok_or_else(|| {
                AdapterError::InvalidPayload(
                    "bitbucket webhook pullrequest.id is missing".to_string(),
                )
            })?;
        let updated = pull_request
            .get("updated_on")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload(
                    "bitbucket webhook pullrequest.updated_on is missing".to_string(),
                )
            })?;
        let repository = request
            .payload
            .get("repository")
            .and_then(|repo| repo.get("slug"))
            .and_then(|value| value.as_str())
            .unwrap_or("unknown");

        Ok(CreateRawIngestionEventInput {
            source_system: "bitbucket".to_string(),
            source_object_type: "pull_request".to_string(),
            source_object_id: format!("{repository}#{id}"),
            source_event_key: format!("bitbucket-pr-{id}-{updated}"),
            source_version: None,
            source_updated_at: Some(updated.to_string()),
            payload: request.payload,
        })
    }
}

fn epoch_millis_to_rfc3339(value: i64) -> Result<String, AdapterError> {
    Utc.timestamp_millis_opt(value)
        .single()
        .map(|datetime| datetime.to_rfc3339())
        .ok_or_else(|| {
            AdapterError::InvalidPayload(format!("invalid bitbucket timestamp: {value}"))
        })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::{BitbucketPullAdapter, BitbucketPushAdapter};
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
    async fn bitbucket_pull_adapter_parses_pull_request_response() {
        let transport = Arc::new(StaticTransport {
            payload: serde_json::json!({
                "values": [
                    {
                        "id": 17,
                        "updatedDate": 1775559600000i64,
                        "fromRef": {
                            "repository": {
                                "slug": "platform"
                            }
                        }
                    }
                ]
            }),
        });
        let adapter = BitbucketPullAdapter::new_for_test(
            transport,
            "https://bitbucket.example.com".to_string(),
            None,
        );

        let records = adapter
            .pull(PullAdapterRequest {
                mode: "incremental".to_string(),
                scope: serde_json::json!({"project_key": "ALM", "repository_slug": "platform"}),
            })
            .await
            .expect("bitbucket pull should parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source_object_type, "pull_request");
    }

    #[test]
    fn bitbucket_push_adapter_maps_webhook_payload() {
        let adapter = BitbucketPushAdapter;

        let record = adapter
            .adapt(PushAdapterRequest {
                source_system: "bitbucket".to_string(),
                source_object_type: "pull_request".to_string(),
                source_object_id: "platform#17".to_string(),
                source_event_key: "ignored".to_string(),
                source_version: None,
                source_updated_at: None,
                payload: serde_json::json!({
                    "pullrequest": {
                        "id": 17,
                        "updated_on": "2026-04-07T09:00:00+00:00"
                    },
                    "repository": {
                        "slug": "platform"
                    }
                }),
            })
            .expect("bitbucket webhook should map");

        assert_eq!(record.source_system, "bitbucket");
        assert_eq!(record.source_object_type, "pull_request");
    }
}
