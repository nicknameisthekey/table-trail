use sqlx::Pool;
use sqlx::{sqlite::SqliteConnectOptions, Sqlite};
use std::env;
use std::fs;
use std::sync::Arc;

pub struct App {
    app_config: Arc<sqlx::Pool<Sqlite>>,
    connected_profiles: Vec<i64>,
}

impl App {
    pub async fn new() -> Self {
        let home_path = env::var("HOME").unwrap();
        let config_path = format!("{}/.config/table_trail", home_path);
        let config_pool = Self::init_config_db(&config_path).await;

        App {
            app_config: Arc::new(config_pool),
            connected_profiles: vec![],
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
            CREATE TABLE IF NOT EXISTS connection_params (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                host TEXT NOT NULL,
                port TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                database_type TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await;

        pool
    }

    pub async fn add_profile(&self, profile: crate::front_models::ConnectionProfile) {
        let pool = Arc::clone(&self.app_config);

        let r = sqlx::query(
            r#"
            INSERT INTO connection_params (name, host, port, username, password, database_type)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(profile.params.name)
        .bind(profile.params.host)
        .bind(profile.params.port)
        .bind(profile.params.username)
        .bind(profile.params.password)
        .bind(profile.params.database_type)
        .execute(&*pool)
        .await;

        r.unwrap();
    }

    pub async fn connection_profiles(&self) -> Vec<crate::front_models::ConnectionProfile> {
        let pool = Arc::clone(&self.app_config);

        let rows: Vec<(i64, String, String, String, String, String, String)> =
            sqlx::query_as("SELECT * FROM connection_params")
                .fetch_all(&*pool)
                .await
                .unwrap();

        let conn_params: Vec<crate::app_config::ConnectionParams> = rows
            .into_iter()
            .map(|row| {
                let (id, name, host, port, username, password, database_type) = row;
                crate::app_config::ConnectionParams {
                    id,
                    name,
                    host,
                    port,
                    username,
                    password,
                    database_type,
                }
            })
            .collect();

        let result = conn_params
            .into_iter()
            .map(move |c| {
                let connected = self.connected_profiles.contains(&c.id);
                crate::front_models::ConnectionProfile::from(c, connected)
            })
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn adding_connection() {
        let app = App::new().await;
    }
}
