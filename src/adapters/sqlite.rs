use crate::errors::Result;
use crate::schema::{Column, Table};
use sqlx::{QueryBuilder, SqlitePool};

pub async fn schema(pool: &SqlitePool) -> Result<Vec<Table>> {
    let mut tbls = tables(pool).await?;
    // Insert the task, then obtain the ID of this row

    for tbl in tbls.iter_mut() {
        let mut cols = columns(&pool, tbl).await?;
        tbl.columns.append(&mut cols)
    }

    Ok(tbls)
}

async fn tables(conn: &SqlitePool) -> Result<Vec<Table>> {
    let mut qb: QueryBuilder<_> = QueryBuilder::new(r#"PRAGMA table_list"#);
    let q = qb.build_query_as::<TableRow>();
    let rows = q.fetch_all(conn).await?;

    let tables = rows
        .into_iter()
        .map(|row| Table::new(row.name, row.schema, row.r#type))
        .collect();

    Ok(tables)
}

async fn columns(pool: &SqlitePool, table: &Table) -> Result<Vec<Column>> {
    let rows = sqlx::query_as::<_, ColumnInfoRow>("select * from pragma_table_info(?)")
        .bind(&table.name)
        .fetch_all(pool)
        .await?;

    let columns = rows
        .into_iter()
        .map(|row| Column {
            name: row.name,
            r#type: row.r#type,
            null: row.notnull == 1,
        })
        .collect();

    Ok(columns)
}

#[derive(sqlx::FromRow, Debug)]
struct TableRow {
    name: String,
    schema: String,
    r#type: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
struct ColumnInfoRow {
    name: String,
    //cid: i64,
    r#type: String,
    notnull: i64,
    //pk: i64,
    //dflt_value: Option<String>,
}
