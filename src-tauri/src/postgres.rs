use futures::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, Row};

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
