use sqlx::{Column, Connection, Row, SqliteConnection, TypeInfo};

use crate::{
    app_config::ConnectionParams,
    front_models::{self, DbObjects},
};

pub async fn query(params: ConnectionParams, query: String) -> front_models::QueryResult {
    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", params.host))
        .await
        .unwrap();

    let query_result = sqlx::query(&query).fetch_all(&mut conn).await.unwrap();

    let mut columns = vec![];
    if query_result.len() == 0 {
        return front_models::QueryResult {
            columns: vec![],
            rows: vec![],
        };
    }

    query_result[0].columns().iter().for_each(|col| {
        columns.push(col.name().to_owned());
    });

    let rows: Vec<Vec<String>> = query_result
        .iter()
        .map(|row| {
            let row_data: Vec<String> = row
                .columns()
                .iter()
                .map(|col| {
                    let column_name = col.name();

                    let data_type = col.type_info();
                    let type_name = data_type.name();
                    println!("name: {}, type: {}", column_name, type_name);

                    /*
                     DataType::Float => "REAL",
                     DataType::Blob => "BLOB",
                     DataType::Numeric => "NUMERIC",

                     // non-standard extensions
                     DataType::Bool => "BOOLEAN",
                     DataType::Datetime => "DATETIME",
                    */

                    let value = match type_name {
                        "TEXT" | "DATE" | "TIME" => {
                            let column_value: String =
                                row.try_get(column_name).unwrap_or("".to_owned());
                            Some(column_value)
                        }
                        "NULL" => Some("NULL".to_owned()),
                        "INTEGER" => {
                            let column_value: i64 = row.try_get(column_name).unwrap();
                            Some(column_value.to_string())
                        }
                        _ => None,
                    };

                    value.unwrap_or("".to_owned())
                })
                .collect();

            row_data
        })
        .collect();

    return front_models::QueryResult { columns, rows };
}

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
