use std::{any::Any, collections::HashMap, ops::Deref};

use futures::TryStreamExt;
use sqlx::{
    postgres::{PgPoolOptions, PgTypeKind, PgValueRef},
    Column, Row, TypeInfo,
};

#[derive(Debug)]
pub struct PGTable {
    pub name: String,
    pub schema: String,
    pub user_defined: bool,
}

pub async fn tables<'a>() -> Result<Vec<PGTable>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:admin@localhost")
        .await?;

    let rows = sqlx::query(
        "SELECT
            nspname AS schema_name,
            relname AS table_name
         FROM
            pg_class
         JOIN
            pg_namespace ON pg_namespace.oid = pg_class.relnamespace
         WHERE
            relkind = 'r' -- 'r' denotes tables; use 'v' for views, etc.
         ORDER BY
            schema_name,
            table_name;",
    )
    .fetch(&pool);

    let result: Vec<_> = rows
        .map_ok(|r| {
            let schema: &str = r.try_get("schema_name").unwrap();
            let table: &str = r.try_get("table_name").unwrap();
            PGTable {
                name: table.to_string(),
                schema: schema.to_string(),
                user_defined: schema != "information_schema" && schema != "pg_catalog", //todo: find real disctinction
            }
        })
        .try_collect()
        .await
        .unwrap();

    Ok(result)
}

pub async fn send_query<'a>(query: String) -> Result<Vec<HashMap<String, String>>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:admin@localhost")
        .await?;

    let rows = sqlx::query(&query).fetch(&pool);

    let result: Vec<HashMap<String, String>> = rows
        .map_ok(|row| {
            let mut map = HashMap::new();
            
            for column in row.columns() {
                
                let column_name = column.name();

                let data_type = column.type_info();
                let type_name = data_type.name();
                println!("name: {}, type: {}",column_name, type_name);

                let value = match type_name {
                    "VARCHAR" | "TEXT" | "NAME" => {
                        let column_value: String = row.try_get(column_name).unwrap_or("".to_owned());
                        Some(column_value)
                    },
                    "INT8" => {
                        let column_value: i64 = row.try_get(column_name).unwrap();
                        Some(column_value.to_string())
                    }
                    _ => None,
                };

                map.insert(column_name.to_owned(), value.unwrap_or("".to_owned()));
            }

            map
        })
        .try_collect()
        .await
        .unwrap();

    print!("{:?}", result);

    Ok(result)
}
