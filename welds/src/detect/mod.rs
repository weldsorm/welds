use crate::connection::Connection;
use crate::connection::DbProvider;
use crate::table::TableIdent;
use anyhow::Result;
use log::debug;
use sqlx::database::HasArguments;
use sqlx::Arguments;
use sqlx::IntoArguments;
use std::collections::HashMap;

mod table_scan;
pub(crate) use table_scan::TableScan;
mod table_scan_row;
use table_scan_row::TableScanRow;
mod fk_scan_row;
use fk_scan_row::{FkScanRow, FkScanTableCol};

pub(crate) mod table_def;
pub use table_def::{ColumnDef, DataType, RelationDef, TableDef, TableDefSingle};

/// Returns a list of all user defined tables in the database
/// requires feature `detect`
pub async fn find_tables<'c, 'args, DB, C>(conn: &'c C) -> Result<Vec<TableDef>>
where
    'c: 'args,
    C: Connection<DB>,
    <DB as HasArguments<'args>>::Arguments: IntoArguments<'args, DB>,
    DB: sqlx::Database + TableScan,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
{
    let sql = DB::table_scan_sql();
    let args: <DB as HasArguments>::Arguments = Default::default();

    debug!("i haz sql");
    let mut raw_rows = conn.fetch_rows(sql, args).await?;
    debug!("i haz rows {:?}", raw_rows.len());

    let rows: Vec<TableScanRow> = raw_rows.drain(..).map(|r| r.into()).collect();
    let mut tables = build_table_defs(rows);

    debug!("i gets fkeys");
    // Build a list of all the FKs
    let sql = DB::fk_scan_sql();
    let args: <DB as HasArguments>::Arguments = Default::default();
    let mut fks_raw = conn.fetch_rows(sql, args).await?;
    let fks: Vec<FkScanRow> = fks_raw.drain(..).map(|r| r.into()).collect();

    link_fks_into_tables(&fks, &mut tables);

    Ok(tables)
}

/// Returns the schema info for a given table in the database
/// NOTE: does not include relationship info. use find_tables for that
pub async fn find_table<'a, 'c, 'args1, 'args2, DB, C>(
    namespace: Option<&'a str>,
    tablename: &'a str,
    conn: &'c C,
) -> Result<Option<TableDefSingle>>
where
    'a: 'args1,
    'c: 'args1,
    C: Connection<DB>,
    <DB as HasArguments<'args1>>::Arguments: IntoArguments<'args2, DB>,
    DB: sqlx::Database + TableScan,
    usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    i32: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    String: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    Option<String>: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
    for<'x> &'x str: sqlx::Type<DB> + sqlx::Encode<'x, DB>,
    for<'x> Option<&'x str>: sqlx::Type<DB> + sqlx::Encode<'x, DB>,
{
    let sql = DB::single_table_scan_sql();
    let mut args: <DB as HasArguments>::Arguments = Default::default();
    args.add(namespace);
    // Mysql query needs the namespace param twice
    if conn.provider() == DbProvider::MySql {
        args.add(namespace);
    }
    args.add(tablename);

    let mut raw_rows = conn.fetch_rows(sql, args).await?;

    let rows: Vec<TableScanRow> = raw_rows.drain(..).map(|r| r.into()).collect();
    let table = build_table_defs(rows).pop().map(|x| x.into());

    Ok(table)
}

fn link_fks_into_tables(fks: &[FkScanRow], tables: &mut [TableDef]) {
    // Build lookup to the FKs
    let mut belongs_to = build_lookup(fks, |x| &x.me);
    let mut has_many = build_lookup(fks, |x| &x.other);

    // Add all the FKs to their appropriate tables
    for table in tables {
        let ident = table.ident.clone();
        // build the belongs_to
        if let Some(bt) = belongs_to.remove(&ident) {
            bt.iter().for_each(|&x| {
                let other_table = x.other.ident.clone();
                let fk = x.me.column.as_str();
                let pk = x.other.column.as_str();
                let ref_def = RelationDef::new(other_table, fk, pk);
                table.belongs_to.push(ref_def);
            });
        }
        // has_many
        if let Some(hm) = has_many.remove(&ident) {
            hm.iter().for_each(|&x| {
                let other_table = x.me.ident.clone();
                let fk = x.me.column.as_str();
                let pk = x.other.column.as_str();
                let ref_def = RelationDef::new(other_table, fk, pk);
                table.has_many.push(ref_def);
            });
        }
    }
}

/// Groups the Table Scan Rows into TableDefs
fn build_table_defs(rows: Vec<TableScanRow>) -> Vec<TableDef> {
    //group the rows into vecs for each table
    let mut buckets = HashMap::new();
    for row in rows {
        let key = row.ident();
        let bucket = buckets.entry(key).or_insert_with(Vec::default);
        bucket.push(row);
    }
    // build a table for each bucket
    let mut tables = Vec::default();
    for (ident, bucket) in buckets.drain() {
        let ty = bucket[0].kind();
        let columns = build_cols(bucket);
        tables.push(TableDef {
            ident,
            ty,
            columns,
            has_many: Vec::default(),
            belongs_to: Vec::default(),
        });
    }
    tables
}

fn build_lookup(
    fks: &[FkScanRow],
    src: impl Fn(&FkScanRow) -> &FkScanTableCol,
) -> HashMap<&TableIdent, Vec<&FkScanRow>> {
    let mut map = HashMap::new();
    for fk in fks {
        let key = &src(fk).ident;
        let values = map.entry(key).or_insert_with(Vec::default);
        values.push(fk);
    }
    map
}

fn build_cols(mut rows: Vec<TableScanRow>) -> Vec<ColumnDef> {
    rows.drain(..)
        .map(|r| ColumnDef {
            name: r.column_name,
            ty: r.column_type.to_uppercase(),
            null: r.is_nullable > 0,
            primary_key: r.is_primary_key > 0,
            updatable: r.is_updatable > 0,
        })
        .collect()
}
