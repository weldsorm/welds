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
