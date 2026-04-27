use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::{
    AdapterError, AdapterHttpRequest, HttpTransport, PullAdapterRequest, PullSourceAdapter,
    PushAdapterRequest, PushEventAdapter,
};
use crate::services::pull_sync::PullRecordInput;
use crate::services::raw_ingestion::CreateRawIngestionEventInput;

pub struct JiraPullAdapter {
    transport: Arc<dyn HttpTransport>,
    base_url: Option<String>,
    bearer_token: Option<String>,
}

impl JiraPullAdapter {
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

    fn build_search_url(&self, request: &PullAdapterRequest) -> Result<String, AdapterError> {
        let base_url = self.base_url.as_ref().ok_or_else(|| {
            AdapterError::ExternalCall("jira base url is not configured".to_string())
        })?;
        let project_keys = request
            .scope
            .get("project_keys")
            .and_then(|value| value.as_array())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default();

        let jql = if project_keys.is_empty() {
            "order by updated desc".to_string()
        } else {
            format!("project in ({project_keys}) order by updated desc")
        };
        let encoded_jql = jql.replace(' ', "%20");
        let mode = request.mode.as_str();

        Ok(format!(
            "{base_url}/rest/api/2/search?jql={encoded_jql}&fields=summary,status,updated&mode={mode}"
        ))
    }

    fn parse_search_response(
        value: serde_json::Value,
    ) -> Result<Vec<PullRecordInput>, AdapterError> {
        let issues = value
            .get("issues")
            .and_then(|items| items.as_array())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("jira issues array is missing".to_string())
            })?;

        issues
            .iter()
            .map(|issue| {
                let issue_id = issue
                    .get("id")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("jira issue.id is missing".to_string())
                    })?;
                let issue_key = issue
                    .get("key")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload("jira issue.key is missing".to_string())
                    })?;
                let updated = issue
                    .get("fields")
                    .and_then(|fields| fields.get("updated"))
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| {
                        AdapterError::InvalidPayload(
                            "jira issue.fields.updated is missing".to_string(),
                        )
                    })?;

                Ok(PullRecordInput {
                    source_object_type: "issue".to_string(),
                    source_object_id: issue_key.to_string(),
                    source_event_key: format!("jira-issue-{issue_id}-{updated}"),
                    source_version: None,
                    source_updated_at: parse_jira_datetime(updated)?,
                    payload: issue.clone(),
                })
            })
            .collect()
    }
}

#[async_trait]
impl PullSourceAdapter for JiraPullAdapter {
    fn source_system(&self) -> &'static str {
        "jira"
    }

    async fn pull(
        &self,
        request: PullAdapterRequest,
    ) -> Result<Vec<PullRecordInput>, AdapterError> {
        let url = self.build_search_url(&request)?;
        let response = self
            .transport
            .get_json(AdapterHttpRequest {
                url,
                bearer_token: self.bearer_token.clone(),
            })
            .await?;

        Self::parse_search_response(response)
    }
}

pub struct JiraPushAdapter;

impl PushEventAdapter for JiraPushAdapter {
    fn source_system(&self) -> &'static str {
        "jira"
    }

    fn adapt(
        &self,
        request: PushAdapterRequest,
    ) -> Result<CreateRawIngestionEventInput, AdapterError> {
        let issue = request.payload.get("issue").ok_or_else(|| {
            AdapterError::InvalidPayload("jira webhook issue is missing".to_string())
        })?;
        let issue_key = issue
            .get("key")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("jira webhook issue.key is missing".to_string())
            })?;
        let issue_id = issue
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload("jira webhook issue.id is missing".to_string())
            })?;
        let updated = issue
            .get("fields")
            .and_then(|fields| fields.get("updated"))
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                AdapterError::InvalidPayload(
                    "jira webhook issue.fields.updated is missing".to_string(),
                )
            })?;

        Ok(CreateRawIngestionEventInput {
            source_system: "jira".to_string(),
            source_object_type: "issue".to_string(),
            source_object_id: issue_key.to_string(),
            source_event_key: format!("jira-issue-{issue_id}-{updated}"),
            source_version: None,
            source_updated_at: parse_jira_datetime(updated)?,
            payload: request.payload,
        })
    }
}

fn parse_jira_datetime(value: &str) -> Result<Option<String>, AdapterError> {
    DateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.3f%z")
        .map(|datetime| datetime.with_timezone(&Utc).to_rfc3339())
        .map(Some)
        .map_err(|_| AdapterError::InvalidPayload(format!("invalid jira datetime: {value}")))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::{JiraPullAdapter, JiraPushAdapter};
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
    async fn jira_pull_adapter_parses_issue_search_response() {
        let transport = Arc::new(StaticTransport {
            payload: serde_json::json!({
                "issues": [
                    {
                        "id": "10001",
                        "key": "ALM-123",
                        "fields": {
                            "updated": "2026-04-07T08:15:00.000+0000",
                            "summary": "Sync process update"
                        }
                    }
                ]
            }),
        });
        let adapter =
            JiraPullAdapter::new_for_test(transport, "https://jira.example.com".to_string(), None);

        let records = adapter
            .pull(PullAdapterRequest {
                mode: "incremental".to_string(),
                scope: serde_json::json!({"project_keys": ["ALM"]}),
            })
            .await
            .expect("jira pull should parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source_object_id, "ALM-123");
    }

    #[test]
    fn jira_push_adapter_maps_webhook_payload() {
        let adapter = JiraPushAdapter;

        let record = adapter
            .adapt(PushAdapterRequest {
                source_system: "jira".to_string(),
                source_object_type: "issue".to_string(),
                source_object_id: "ALM-123".to_string(),
                source_event_key: "ignored".to_string(),
                source_version: None,
                source_updated_at: None,
                payload: serde_json::json!({
                    "issue": {
                        "id": "10001",
                        "key": "ALM-123",
                        "fields": {
                            "updated": "2026-04-07T08:15:00.000+0000"
                        }
                    }
                }),
            })
            .expect("jira webhook should map");

        assert_eq!(record.source_system, "jira");
        assert_eq!(record.source_object_id, "ALM-123");
    }
}
