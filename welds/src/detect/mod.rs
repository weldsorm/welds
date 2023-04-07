use crate::errors::Result;
use crate::table::{ColumnDef, TableDef};
use sqlx::database::HasArguments;
use sqlx::{IntoArguments, Row};
use std::collections::HashMap;

mod table_scan;
use table_scan::{TableScan, TableScanRow};

pub async fn find_tables<'i, 's, 'e, 'ee, 'args, DB, E>(exec: &'ee E) -> Result<Vec<TableDef>>
where
    &'ee E: sqlx::Executor<'e, Database = DB>,
    <DB as HasArguments<'args>>::Arguments: IntoArguments<'args, DB>,
    DB: sqlx::Database + TableScan,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
{
    let sql = DB::table_scan_sql();
    let q = sqlx::query(sql);
    let rows: Vec<_> = q
        .fetch_all(exec)
        .await?
        .iter()
        .map(|r| TableScanRow {
            schema: r.get(0),
            table_name: r.get(1),
            ty: r.get(2),
            column_name: r.get(3),
            column_type: r.get(4),
            is_nullable: r.get(5),
            is_primary_key: r.get(6),
            is_updatable: r.get(7),
        })
        .collect();

    //group the rows into vecs for each table
    let mut buckets = HashMap::new();
    for row in rows {
        let key = row.ident();
        let bucket = buckets.entry(key).or_insert_with(|| Vec::default());
        bucket.push(row);
    }

    // build a table for each bucket
    let mut tables = Vec::default();
    for (ident, bucket) in buckets.drain() {
        let ty = bucket[0].kind();
        let columns = build_cols(bucket);
        tables.push(TableDef { ident, ty, columns });
    }

    Ok(tables)
}

fn build_cols(mut rows: Vec<TableScanRow>) -> Vec<ColumnDef> {
    rows.drain(..)
        .map(|r| ColumnDef {
            name: r.column_name,
            ty: r.column_type,
            null: r.is_nullable > 0,
            primary_key: r.is_primary_key > 0,
            updatable: r.is_updatable > 0,
        })
        .collect()
}
