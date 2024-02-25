use sqlx::{Connection, SqliteConnection};

use crate::{
    app_config::ConnectionParams,
    front_models::{self, DbObjects},
};

pub async fn tables(params: ConnectionParams) -> front_models::DbObjects {
    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", params.host))
        .await
        .unwrap();

    let rows: Vec<(String,)> = sqlx::query_as("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    let tables: Vec<front_models::Table> = rows
        .into_iter()
        .map(|(name,)| front_models::Table {
            schema: "".to_owned(),
            name,
        })
        .collect();

    DbObjects { tables }
}
