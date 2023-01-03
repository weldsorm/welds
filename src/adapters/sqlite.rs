use sqlx::{ Pool, SqlitePool};
use crate::schema::{Table, Column};
use std::env;
use anyhow::Result;

pub async fn schema() -> Result<Vec<Table>> {
    let uri = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&uri).await?;


    let mut tbls = tables(&pool).await?;
    // Insert the task, then obtain the ID of this row

    for tbl in tbls.iter_mut() {
        let mut cols = columns(&pool, tbl).await?;
        tbl.columns.append(&mut cols)
    }

    println!("Results: {:?}", tbls);

    Ok(tbls)
}

async fn tables(pool: &Pool<sqlx::Sqlite>) -> Result<Vec<Table>> {
    let rows = sqlx::query!(r#"PRAGMA table_list"#).fetch_all(pool).await?;


    let tables = rows.into_iter().map(|row|{
        Table::new(row.name, row.schema, Some(row.r#type) )
    }).collect();

    //"select * from pragma_table_info('tblName') as tblInfo;"
    Ok(tables)
}

#[derive(sqlx::FromRow, Debug)]
struct ColumnInfoRow {
    name: String,
    cid: i64,
    r#type: String,
    notnull: i64,
    pk: i64,
    dflt_value: Option<String>
}

async fn columns(pool: &Pool<sqlx::Sqlite>, table: &Table) -> Result<Vec<Column>> {

    let rows = sqlx::query_as::<_, ColumnInfoRow>("select * from pragma_table_info(?)")
        .bind(&table.name)
        .fetch_all(pool).await?;

    let columns = rows.into_iter().map(|row| {
        Column{
            name: row.name,
            r#type: row.r#type,
            null: row.notnull == 1
        }
    }).collect();


    Ok(columns)
}
