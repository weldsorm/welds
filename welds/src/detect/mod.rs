use crate::errors::Result;
use crate::model_traits::TableIdent;
use crate::query::clause::ParamArgs;
use crate::Client;
use crate::Syntax;
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
pub async fn find_tables<C>(client: &C) -> Result<Vec<TableDef>>
where
    C: Client,
{
    let syntax = client.syntax();
    let ts = TableScan::new(syntax);
    let sql = ts.table_scan_sql();

    let args: ParamArgs = Vec::default();
    let mut raw_rows = client.fetch_rows(sql, &args).await?;

    let rows: Result<Vec<TableScanRow>> = raw_rows.drain(..).map(|r| r.try_into()).collect();
    let rows = rows?;
    let mut tables = build_table_defs(syntax, rows);

    // Build a list of all the FKs
    let sql = ts.fk_scan_sql();
    let args: ParamArgs = Vec::default();
    let mut fks_raw = client.fetch_rows(sql, &args).await?;
    let fks: Result<Vec<FkScanRow>> = fks_raw.drain(..).map(|r| r.try_into()).collect();
    let fks = fks?;

    link_fks_into_tables(&fks, &mut tables);

    Ok(tables)
}

/// Returns the schema info for a given table in the database
/// NOTE: does not include relationship info. use find_tables for that
pub async fn find_table<C>(
    namespace: Option<impl Into<String>>,
    tablename: impl Into<String>,
    client: &C,
) -> Result<Option<TableDefSingle>>
where
    C: Client,
{
    let syntax = client.syntax();
    let ts = TableScan::new(syntax);
    let sql = ts.single_table_scan_sql();
    let mut args: ParamArgs = Vec::default();
    let namespace: Option<String> = namespace.map(|x| x.into());
    let tablename: String = tablename.into();
    args.push(&namespace);

    // Mysql query needs the namespace param twice
    if let Syntax::Mysql = syntax {
        args.push(&namespace);
    }

    args.push(&tablename);

    let mut raw_rows = client.fetch_rows(sql, &args).await?;

    let rows: Result<Vec<TableScanRow>> = raw_rows.drain(..).map(|r| r.try_into()).collect();
    let rows = rows?;

    let table = build_table_defs(syntax, rows).pop().map(|x| x.into());

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
fn build_table_defs(syntax: Syntax, rows: Vec<TableScanRow>) -> Vec<TableDef> {
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
            syntax,
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
        .map(|r| {
            // NOTE: we get _TYPE back for types that are TYPE[]. doing this type swap back to normal here.
            let mut ty: String = r.column_type.to_uppercase();
            if ty.starts_with('_') {
                ty = format!("{}[]", &ty[1..]);
            }

            ColumnDef {
                name: r.column_name,
                ty,
                null: r.is_nullable > 0,
                primary_key: r.is_primary_key > 0,
                updatable: r.is_updatable > 0,
            }
        })
        .collect()
}
