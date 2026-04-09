use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};

pub struct Settings {
    pub bind_address: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub auto_apply_migrations: bool,
    pub cors_allowed_origins: Vec<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        let values = std::env::vars().collect::<HashMap<_, _>>();
        Self::from_map(&values)
    }

    fn from_map(values: &HashMap<String, String>) -> Result<Self> {
        let bind_address = values
            .get("ALM_BACKEND_BIND_ADDRESS")
            .cloned()
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let database_url = values
            .get("ALM_BACKEND_DATABASE_URL")
            .cloned()
            .ok_or_else(|| anyhow!("ALM_BACKEND_DATABASE_URL is required"))?;
        let database_max_connections = values
            .get("ALM_BACKEND_DATABASE_MAX_CONNECTIONS")
            .map(|value| {
                value.parse::<u32>().with_context(|| {
                    format!("invalid ALM_BACKEND_DATABASE_MAX_CONNECTIONS: {value}")
                })
            })
            .transpose()?
            .unwrap_or(10);
        let auto_apply_migrations = values
            .get("ALM_BACKEND_AUTO_APPLY_MIGRATIONS")
            .map(|value| parse_bool_flag("ALM_BACKEND_AUTO_APPLY_MIGRATIONS", value))
            .transpose()?
            .unwrap_or(true);
        let cors_allowed_origins = values
            .get("ALM_BACKEND_CORS_ALLOWED_ORIGINS")
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|item| !item.is_empty())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|origins| !origins.is_empty())
            .unwrap_or_else(default_cors_allowed_origins);

        Ok(Self {
            bind_address,
            database_url,
            database_max_connections,
            auto_apply_migrations,
            cors_allowed_origins,
        })
    }
}

pub fn default_cors_allowed_origins() -> Vec<String> {
    vec![
        "http://127.0.0.1:8000".to_string(),
        "http://localhost:8000".to_string(),
        "http://127.0.0.1:8001".to_string(),
        "http://localhost:8001".to_string(),
    ]
}

fn parse_bool_flag(name: &str, value: &str) -> Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(anyhow!("{name} must be either 'true' or 'false'")),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Settings;

    #[test]
    fn from_map_requires_database_url() {
        let values = HashMap::from([(
            "ALM_BACKEND_BIND_ADDRESS".to_string(),
            "0.0.0.0:9000".to_string(),
        )]);

        let result = Settings::from_map(&values);

        assert!(result.is_err());
    }

    #[test]
    fn from_map_reads_database_settings_and_flags() {
        let values = HashMap::from([
            (
                "ALM_BACKEND_BIND_ADDRESS".to_string(),
                "0.0.0.0:9000".to_string(),
            ),
            (
                "ALM_BACKEND_DATABASE_URL".to_string(),
                "postgres://alm:secret@localhost:5432/alm".to_string(),
            ),
            (
                "ALM_BACKEND_DATABASE_MAX_CONNECTIONS".to_string(),
                "12".to_string(),
            ),
            (
                "ALM_BACKEND_AUTO_APPLY_MIGRATIONS".to_string(),
                "false".to_string(),
            ),
            (
                "ALM_BACKEND_CORS_ALLOWED_ORIGINS".to_string(),
                "http://127.0.0.1:8000, http://100.90.113.29:8000".to_string(),
            ),
        ]);

        let settings = Settings::from_map(&values).expect("settings should parse");

        assert_eq!(settings.bind_address, "0.0.0.0:9000");
        assert_eq!(
            settings.database_url,
            "postgres://alm:secret@localhost:5432/alm"
        );
        assert_eq!(settings.database_max_connections, 12);
        assert!(!settings.auto_apply_migrations);
        assert_eq!(
            settings.cors_allowed_origins,
            vec![
                "http://127.0.0.1:8000".to_string(),
                "http://100.90.113.29:8000".to_string()
            ]
        );
    }

    #[test]
    fn from_map_uses_defaults_for_optional_database_settings() {
        let values = HashMap::from([(
            "ALM_BACKEND_DATABASE_URL".to_string(),
            "postgres://alm:secret@localhost:5432/alm".to_string(),
        )]);

        let settings = Settings::from_map(&values).expect("settings should parse");

        assert_eq!(settings.bind_address, "127.0.0.1:8080");
        assert_eq!(settings.database_max_connections, 10);
        assert!(settings.auto_apply_migrations);
        assert_eq!(
            settings.cors_allowed_origins,
            vec![
                "http://127.0.0.1:8000".to_string(),
                "http://localhost:8000".to_string(),
                "http://127.0.0.1:8001".to_string(),
                "http://localhost:8001".to_string()
            ]
        );
    }
}
