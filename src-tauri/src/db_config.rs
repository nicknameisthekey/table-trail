use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum DatabaseType {
    Postgres = 0,
    Sqlite = 1,
}

impl DatabaseType {
    pub(crate) fn from_i8(database_type: i8) -> DatabaseType{
        match database_type {
            0 => DatabaseType::Postgres,
            1 => DatabaseType::Sqlite,
            _ => panic!("Unknown database type")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub host: String,
    pub port: i64,
    pub username: String,
    pub password: String,
    pub database_type: DatabaseType,
}

impl DatabaseConfig {
    pub fn new(
        name: String,
        host: String,
        port: i64,
        username: String,
        password: String,
        database_type: DatabaseType,
    ) -> Self {
        Self {
            name,
            host,
            port,
            username,
            password,
            database_type,
        }
    }
}
