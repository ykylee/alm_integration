use std::sync::Arc;

use sqlx::{FromRow, PgPool};

use crate::adapters::AdapterEndpointConfig;
use crate::security::secrets::{EnvSecretCipher, SecretCipher};

pub struct DbAdapterConfigLoader {
    pool: PgPool,
    secret_cipher: Arc<dyn SecretCipher>,
}

impl DbAdapterConfigLoader {
    pub fn new(pool: PgPool) -> Self {
        Self::with_secret_cipher(pool, Arc::new(EnvSecretCipher::new()))
    }

    pub fn with_secret_cipher(pool: PgPool, secret_cipher: Arc<dyn SecretCipher>) -> Self {
        Self {
            pool,
            secret_cipher,
        }
    }

    pub async fn load_endpoint_configs(&self) -> anyhow::Result<Vec<AdapterEndpointConfig>> {
        let rows = sqlx::query_as::<_, AdapterConfigRow>(
            r#"
            select
              lower(s.system_type) as source_system,
              e.base_url,
              c.secret_ciphertext,
              c.secret_key_version,
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

        rows.into_iter()
            .map(|row| {
                let decrypted_secret = row
                    .secret_ciphertext
                    .as_deref()
                    .map(|ciphertext| {
                        self.secret_cipher
                            .decrypt(ciphertext, row.secret_key_version.as_deref())
                    })
                    .transpose()?;

                Ok(AdapterEndpointConfig {
                    source_system: row.source_system,
                    base_url: Some(row.base_url),
                    bearer_token: decrypted_secret.clone(),
                    push_signing_secret: decrypted_secret,
                    enable_pull: row.enable_pull,
                    enable_push: row.enable_push,
                })
            })
            .collect()
    }
}

#[derive(Debug, FromRow)]
struct AdapterConfigRow {
    source_system: String,
    base_url: String,
    secret_ciphertext: Option<String>,
    secret_key_version: Option<String>,
    enable_pull: bool,
    enable_push: bool,
}
