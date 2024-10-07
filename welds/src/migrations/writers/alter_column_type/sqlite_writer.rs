use crate::detect::TableDef;

pub(crate) fn down_sql(
    table: &TableDef,
    colname_match: impl Into<String>,
    colname_new: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> Vec<String> {
    let temptable = format!("{}_weldstmp", table.ident());
    let col_match: String = colname_match.into();
    let col_new: String = colname_new.into();
    let ty: String = ty.into();
    let old_cols = old_columns(table, &col_match, &col_new);
    let new_cols = new_columns(table, &col_match, &col_new, &ty, nullable);
    let tablename = table.ident().to_string();
    vec![
        build_table_create(&temptable, &old_cols),
        build_copy_data(&tablename, &new_cols, &temptable, &old_cols),
        build_drop(&tablename),
        build_table_rename(&temptable, &tablename),
    ]
}

pub(crate) fn up_sql(
    table: &TableDef,
    colname_match: impl Into<String>,
    colname_new: impl Into<String>,
    ty: impl Into<String>,
    nullable: bool,
) -> Vec<String> {
    let temptable = format!("{}_weldstmp", table.ident());

    let col_match: String = colname_match.into();
    let col_new: String = colname_new.into();
    let ty: String = ty.into();

    let old_cols = old_columns(table, &col_match, &col_new);
    let new_cols = new_columns(table, &col_match, &col_new, &ty, nullable);

    let tablename = table.ident().to_string();
    vec![
        build_table_create(&temptable, &new_cols),
        build_copy_data(&tablename, &old_cols, &temptable, &new_cols),
        build_drop(&tablename),
        build_table_rename(&temptable, &tablename),
    ]
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

#[derive(Debug)]
pub(crate) struct Col {
    name: String,
    ty: String,
    nullable: bool,
    primary_key: bool,
}

// build a list of the new versions of the columns
fn new_columns(
    table: &TableDef,
    col: &str,
    new_name: &str,
    new_ty: &str,
    nullable: bool,
) -> Vec<Col> {
    let mut list = old_columns(table, col, new_name);

    list.drain(..)
        .map(|c| {
            if c.name != col {
                return c;
            }
            // build the updated version of the column
            Col {
                name: col.to_string(),
                ty: new_ty.to_string(),
                nullable,
                primary_key: c.primary_key,
            }
        })
        .collect()
}

// build a list of the old versions of the columns
pub(crate) fn old_columns(tabledef: &TableDef, col_match: &str, col_name: &str) -> Vec<Col> {
    let mut list = Vec::default();
    for def in tabledef.columns() {
        // The column name could have changed AFTER the table was scanned. This happens then the
        // column has been renamed. and THEN a type change occurs. make sure we are using the
        // correct name
        let name = if def.name == col_match {
            col_name
        } else {
            def.name()
        };

        list.push(Col {
            name: name.to_string(),
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
