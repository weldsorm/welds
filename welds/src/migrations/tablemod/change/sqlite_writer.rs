use super::*;
use crate::migrations::types::{ToDbType, Type};
use crate::migrations::MigrationWriter;

impl MigrationWriter<sqlx::Sqlite> for Change {
    fn down_sql(&self) -> Vec<String> {
        if rename_only(self) {
            let new_name = self.new_name.as_ref().unwrap();
            return vec![rename(&self.tabledef, new_name, &self.column_name)];
        }

        let temptable = format!("{}_weldstmp", self.tabledef.ident());
        let old_cols = old_columns(&self.tabledef);
        let new_cols = new_columns(self);
        let tablename = self.tabledef.ident().to_string();
        vec![
            build_table_create(&temptable, &old_cols),
            build_copy_data(&tablename, &new_cols, &temptable, &old_cols),
            build_drop(&tablename),
            build_table_rename(&temptable, &tablename),
        ]
    }

    fn up_sql(&self) -> Vec<String> {
        if rename_only(self) {
            let new_name = self.new_name.as_ref().unwrap();
            return vec![rename(&self.tabledef, &self.column_name, new_name)];
        }

        let temptable = format!("{}_weldstmp", self.tabledef.ident());
        let new_cols = new_columns(self);
        let old_cols = old_columns(&self.tabledef);
        let tablename = self.tabledef.ident().to_string();
        vec![
            build_table_create(&temptable, &new_cols),
            build_copy_data(&tablename, &old_cols, &temptable, &new_cols),
            build_drop(&tablename),
            build_table_rename(&temptable, &tablename),
        ]
    }
}

fn rename_only(change: &Change) -> bool {
    change.new_name.is_some() && change.new_ty.is_none() && change.set_null.is_none()
}

fn rename(tabledef: &TableDef, oldname: &str, newname: &str) -> String {
    let table = tabledef.ident().to_string();
    format!("ALTER TABLE {table} RENAME COLUMN {oldname} TO {newname}")
}

// writes the SQL to create a table
pub(crate) fn build_table_create(tablename: &str, cols: &[Col]) -> String {
    let cols_sql: Vec<String> = cols.iter().map(write_col).collect();

    // Join the columns parts together
    let cols_sql_joined = cols_sql.join(", ");
    // join all the parts together
    format!("CREATE TABLE {tablename} ( {cols_sql_joined} )")
}

/// write the column part for a create table
fn write_col(col: &Col) -> String {
    if col.primary_key {
        return write_pk(col);
    }
    let name = &col.name;
    let ty = &col.ty;
    let nullable = if col.nullable { "NULL" } else { "NOT NULL" };
    format!("{name} {ty} {nullable}")
}

/// write the primary key column part for a create table
fn write_pk(col: &Col) -> String {
    let name = &col.name;
    let mut ty = col.ty.as_str();
    let mut auto = "";
    if is_int(ty) {
        auto = " AUTOINCREMENT";
        ty = "INTEGER";
    };
    //else { "" };
    format!("{name} {ty} PRIMARY KEY{auto}")
}

pub(crate) struct Col {
    name: String,
    ty: String,
    nullable: bool,
    primary_key: bool,
}

// build a list of the new versions of the columns
fn new_columns(change: &Change) -> Vec<Col> {
    let mut list = old_columns(&change.tabledef);

    list.drain(..)
        .map(|c| {
            if c.name != change.column_name {
                return c;
            }

            // get the override fields
            let name = change.new_name.as_deref();
            let ty: Option<Type> = change.new_ty.clone();
            let ty: Option<String> = ty.map(|x| ToDbType::<sqlx::Sqlite>::dbtype(&x));
            let nullable: Option<bool> = change.set_null;

            // build the updated version of the column
            Col {
                name: name.unwrap_or(&c.name).to_string(),
                ty: ty.unwrap_or(c.ty.to_string()),
                nullable: nullable.unwrap_or(c.nullable),
                primary_key: c.primary_key,
            }
        })
        .collect()
}

// build a list of the old versions of the columns
pub(crate) fn old_columns(tabledef: &TableDef) -> Vec<Col> {
    let mut list = Vec::default();
    for def in tabledef.columns() {
        list.push(Col {
            name: def.name.to_string(),
            ty: def.ty.to_string(),
            nullable: def.null,
            primary_key: def.primary_key,
        });
    }
    list
}

fn build_copy_data(
    src_table: &str,
    src_cols: &[Col],
    dest_table: &str,
    dest_cols: &[Col],
) -> String {
    let dest_col_parts: Vec<_> = dest_cols.iter().map(|c| c.name.as_str()).collect();
    let dest_col_joined = dest_col_parts.join(", ");

    let mut src_col_parts = Vec::default();
    for (src, dest) in src_cols.iter().zip(dest_cols) {
        if src.ty == dest.ty {
            src_col_parts.push(src.name.to_string());
        } else {
            let convert = format!("CAST({} AS {})", src.name, dest.ty);
            src_col_parts.push(convert);
        }
    }
    let src_col_joined = src_col_parts.join(", ");

    format!(
        "INSERT INTO {dest_table} ( {dest_col_joined} ) SELECT {src_col_joined} FROM {src_table}"
    )
}

fn is_int(ty: &str) -> bool {
    ty == "INT"
        || ty == "INTEGER"
        || ty == "TINYINT"
        || ty == "SMALLINT"
        || ty == "MEDIUMINT"
        || ty == "BIGINT"
        || ty == "UNSIGNED BIG INT"
        || ty == "INT2"
        || ty == "INT8"
}

pub(crate) fn build_drop(tablename: &str) -> String {
    format!("DROP TABLE {tablename}")
}

fn build_table_rename(src_table: &str, dest_table: &str) -> String {
    format!("ALTER TABLE {src_table} RENAME TO {dest_table}")
}
