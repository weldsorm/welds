use super::*;
use crate::Syntax;
use crate::migrations::MigrationWriter;

#[test]
fn should_make_basic_indexes() {
    let m = create_index().table("cars").column("make");

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).join("; ");
    let expected = r#"CREATE INDEX idx_cars_make ON cars ( make )"#;
    assert_eq!(sql, expected.trim(), "MYSQL");

    //postgres
    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    let expected = r#"CREATE INDEX idx_cars_make ON cars ( "make" )"#;
    assert_eq!(sql, expected.trim(), "POSTGRES");

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).join("; ");
    assert_eq!(sql, expected.trim(), "MSSQL");

    //sqlite
    let sql = MigrationWriter::up_sql(&m, Syntax::Sqlite).join("; ");
    assert_eq!(sql, expected.trim(), "SQLITE");
}

#[test]
fn should_make_unique_index_when_unique_is_set_before_column() {
    let m = create_index().table("cars").unique().column("make");

    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).join("; ");
    assert_eq!(
        sql, r#"CREATE UNIQUE INDEX idx_cars_make ON cars ( make )"#,
        "MYSQL"
    );

    let expected = r#"CREATE UNIQUE INDEX idx_cars_make ON cars ( "make" )"#;
    for syntax in [Syntax::Postgres, Syntax::Mssql, Syntax::Sqlite] {
        let sql = MigrationWriter::up_sql(&m, syntax).join("; ");
        assert_eq!(sql, expected, "{syntax:?}");
    }
}

#[test]
fn should_make_unique_index_when_unique_is_set_after_column() {
    let m = create_index().table("cars").column("make").unique();

    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(
        sql,
        r#"CREATE UNIQUE INDEX idx_cars_make ON cars ( "make" )"#
    );
}

#[test]
fn should_keep_unique_across_multiple_columns_and_custom_name() {
    let m = create_index()
        .table("cars")
        .unique()
        .with_name("ux_cars_make_model")
        .column("make")
        .column("model");

    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(
        sql,
        r#"CREATE UNIQUE INDEX ux_cars_make_model ON cars ( "make","model" )"#
    );
}

#[test]
fn should_not_make_unique_index_by_default() {
    let m = create_index().table("cars").column("make").column("model");

    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    assert!(!sql.contains("UNIQUE"), "{sql}");
}
