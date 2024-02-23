use sqlx::Pool;
use sqlx::{sqlite::SqliteConnectOptions, Sqlite};
use std::env;
use std::fs;
use std::sync::Arc;

use crate::db_config::DatabaseConfig;

pub struct App {
    config_pool: Arc<sqlx::Pool<Sqlite>>,
}

impl App {
    pub async fn new() -> Self {
        let home_path = env::var("HOME").unwrap();
        let config_path = format!("{}/.config/table_trail", home_path);
        let config_pool = Self::init_config_db(&config_path).await;

        App {
            config_pool: Arc::new(config_pool),
        }
    }

    pub async fn init_config_db(config_path: &str) -> Pool<Sqlite> {
        fs::create_dir_all(&config_path).unwrap();

        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(format!("{}/config.db", config_path));

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .unwrap();

        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS database_configs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                database_type INTEGER NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await;

        pool
    }

    pub async fn add_db_config(&self, config: DatabaseConfig) {
        let pool = Arc::clone(&self.config_pool);

        let r = sqlx::query(
            r#"
            INSERT INTO database_configs (name, host, port, username, password, database_type)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(config.name)
        .bind(config.host)
        .bind(config.port)
        .bind(config.username)
        .bind(config.password)
        .bind(config.database_type as i8)
        .execute(&*pool)
        .await;

        r.unwrap();
    }

    pub async fn db_configs(&self) -> Vec<DatabaseConfig> {
        let pool = Arc::clone(&self.config_pool);

        let rows: Vec<(i64, String, String, i64, String, String, i8)> = sqlx::query_as("SELECT * FROM database_configs")
            .fetch_all(&*pool)
            .await
            .unwrap();

        let configs: Vec<DatabaseConfig> = rows
            .into_iter()
            .map(|row| {
                let (id, name, host, port, username, password, database_type) = row;
                DatabaseConfig::new(
                    name,
                    host,
                    port,
                    username,
                    password,
                    crate::db_config::DatabaseType::from_i8(database_type),
                )
            })
            .collect();

        configs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn adding_connection() {
        let app = App::new().await;
        app.add_db_config(DatabaseConfig::new(
            "test".to_string(),
            "localhost".to_string(),
            5432,
            "test".to_string(),
            "test".to_string(),
            crate::db_config::DatabaseType::Postgres,
        )).await;

    }
}
