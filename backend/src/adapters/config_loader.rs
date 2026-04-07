use std::sync::Arc;

use sqlx::{FromRow, PgPool};

use crate::adapters::{
    AdapterEndpointConfig, AdapterRegistry, HttpTransport, ReqwestTransport,
    build_registry_from_endpoint_configs,
};

pub struct DbAdapterConfigLoader {
    pool: PgPool,
}

impl DbAdapterConfigLoader {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn load_endpoint_configs(&self) -> Result<Vec<AdapterEndpointConfig>, sqlx::Error> {
        let rows = sqlx::query_as::<_, AdapterConfigRow>(
            r#"
            select
              lower(s.system_type) as source_system,
              e.base_url,
              c.secret_ciphertext as bearer_token,
              case
                when e.endpoint_type in ('pull', 'both') then true
                else false
              end as enable_pull,
              case
                when e.endpoint_type in ('push', 'both') then true
                else false
              end as enable_push
            from integration_endpoint e
            join integration_system s
              on s.integration_system_id = e.integration_system_id
            left join integration_credential c
              on c.integration_endpoint_id = e.integration_endpoint_id
             and c.effective_to is null
            where e.is_active = true
            order by s.system_type, e.endpoint_name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| AdapterEndpointConfig {
                source_system: row.source_system,
                base_url: Some(row.base_url),
                bearer_token: row.bearer_token,
                enable_pull: row.enable_pull,
                enable_push: row.enable_push,
            })
            .collect())
    }

    pub async fn load_registry(&self) -> anyhow::Result<Option<AdapterRegistry>> {
        let endpoint_configs = self.load_endpoint_configs().await?;
        if endpoint_configs.is_empty() {
            return Ok(None);
        }

        let transport: Arc<dyn HttpTransport> = Arc::new(ReqwestTransport::new());
        let registry = build_registry_from_endpoint_configs(&endpoint_configs, transport)?;
        Ok(Some(registry))
    }
}

#[derive(Debug, FromRow)]
struct AdapterConfigRow {
    source_system: String,
    base_url: String,
    bearer_token: Option<String>,
    enable_pull: bool,
    enable_push: bool,
}
