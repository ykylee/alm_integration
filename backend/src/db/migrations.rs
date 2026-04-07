use sqlx::migrate::Migrator;

#[allow(dead_code)]
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[cfg(test)]
mod tests {
    use super::MIGRATOR;

    #[test]
    fn includes_initial_runtime_migration() {
        let migrations: Vec<String> = MIGRATOR
            .iter()
            .map(|migration| migration.description.to_string())
            .collect();

        assert!(
            migrations
                .iter()
                .any(|description| description.contains("integration runtime tables"))
        );
    }
}
