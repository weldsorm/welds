use super::*;
use crate::detect::table_def::mock::MockTableDef;
use crate::migrations::MigrationWriter;
use crate::Syntax;

fn mock_table(syntax: Syntax) -> Table {
    // Note: syntax shouldn't matter here
    Table::mock(
        MockTableDef::new(syntax, "s2.cars")
            .with_pk("id", "INT")
            .with_column("name", "TEXT"),
    )
}

#[test]
fn should_be_able_to_drop_a_table() {
    let table = mock_table(Syntax::Mysql);
    let m = table.drop();

    let expected = r#"DROP TABLE s2.cars"#;
    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).join("; ");
    assert_eq!(sql, expected.trim());

    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(sql, expected.trim());

    let sql = MigrationWriter::up_sql(&m, Syntax::Sqlite).join("; ");
    assert_eq!(sql, expected.trim());

    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).join("; ");
    assert_eq!(sql, expected.trim());
}

#[test]
fn down_should_recreate_the_table() {
    let table = mock_table(Syntax::Postgres);
    let m = table.drop();
    let expected = r#"
    CREATE TABLE s2.cars ( id SERIAL PRIMARY KEY, name TEXT NOT NULL )"#;

    let sql = MigrationWriter::down_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(sql, expected.trim());
}