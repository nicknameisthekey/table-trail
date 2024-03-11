use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionProfile {
    pub params: ConnectionParams,
    pub connected: bool,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionParams {
    pub name: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database_type: String,
}

impl ConnectionProfile {
    pub fn from(conn_params: crate::app_config::ConnectionParams, connected: bool) -> Self {
        Self {
            params: ConnectionParams {
                database_type: conn_params.database_type,
                name: conn_params.name,
                host: conn_params.host,
                port: conn_params.port,
                username: conn_params.username,
                password: conn_params.password,
            },
            id: conn_params.id,
            connected: connected,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DbObjects {
    pub tables: Vec<Table>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub schema: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
