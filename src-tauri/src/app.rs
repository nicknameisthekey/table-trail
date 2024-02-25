use futures::lock::Mutex;
use sqlx::Pool;
use sqlx::{sqlite::SqliteConnectOptions, Sqlite};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Arc;

pub struct App {
    app_config: Arc<sqlx::Pool<Sqlite>>,
    connected_profiles: Vec<i64>, //todo delete maybe?
    cached_db_objects: Mutex<HashMap<i64, crate::front_models::DbObjects>>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn db_objects(
    app: tauri::State<'_, App>,
    profile_id: i64,
) -> Result<crate::front_models::DbObjects, ()> {
    let mut cache = app.cached_db_objects.lock().await;
    let result = if cache.contains_key(&profile_id) {
        let objects = cache.get(&profile_id).unwrap();
        objects.clone()
    } else {
        let conn_params = app.connection_params(profile_id).await;
        let objects = match conn_params.database_type.as_str() {
            "sqlite" => crate::sqlite::tables(conn_params).await,
            _ => todo!(),
        };
        objects
    };

    cache.insert(profile_id, result.clone());
    Ok(result)
}

impl App {
    pub async fn new() -> Self {
        let home_path = env::var("HOME").unwrap();
        let config_path = format!("{}/.config/table_trail", home_path);
        let config_pool = Self::init_config_db(&config_path).await;

        App {
            app_config: Arc::new(config_pool),
            connected_profiles: vec![],
            cached_db_objects: Mutex::new(HashMap::new()),
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

    async fn connection_params(&self, conn_id: i64) -> crate::app_config::ConnectionParams {
        let pool = Arc::clone(&self.app_config);

        let row: (i64, String, String, String, String, String, String) =
            sqlx::query_as("SELECT * FROM connection_params WHERE id = ?")
                .bind(&conn_id)
                .fetch_one(&*pool)
                .await
                .unwrap();

        crate::app_config::ConnectionParams {
            id: row.0,
            name: row.1,
            host: row.2,
            port: row.3,
            username: row.4,
            password: row.5,
            database_type: row.6,
        }
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
