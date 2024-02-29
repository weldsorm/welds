use super::*;
use crate::detect::table_def::mock::MockTableDef;
use crate::migrations::types::Type;
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

#[test]
fn should_be_able_to_rename_column() {
    let table = mock_table(Syntax::Postgres);
    let m = table.change("name").rename("name2");
    // up sql
    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    let expected_up = r#"ALTER TABLE s2.cars RENAME name TO name2"#;
    assert_eq!(sql, expected_up);
    // down sql
    let sql = MigrationWriter::down_sql(&m, Syntax::Postgres).join("; ");
    let expected_down = r#"ALTER TABLE s2.cars RENAME name2 TO name"#;
    assert_eq!(sql, expected_down);
}

#[test]
fn should_be_able_to_rename_column2() {
    let table = mock_table(Syntax::Mssql);
    let m = table.change("name").rename("name2");
    // up sql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).join("; ");
    let expected_up = r#"EXEC sp_rename 's2.cars', 'name', 'name2'"#;
    assert_eq!(sql, expected_up);
    // down sql
    let sql = MigrationWriter::down_sql(&m, Syntax::Mssql).join("; ");
    let expected_down = r#"EXEC sp_rename 's2.cars', 'name2', 'name'"#;
    assert_eq!(sql, expected_down);
}

#[test]
fn should_be_able_to_rename_and_change_type() {
    let table = mock_table(Syntax::Postgres);
    let t = Type::Raw("APPLE".to_string());
    let m = table.change("name").rename("name2").to_type(t).null();
    // up sql
    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    let expected_up = [
        r#"ALTER TABLE s2.cars RENAME name TO name2"#,
        r#"ALTER TABLE s2.cars ALTER COLUMN name2 TYPE APPLE NULL"#,
    ]
    .join("; ");
    assert_eq!(sql, expected_up);
    // down sql
    let sql = MigrationWriter::down_sql(&m, Syntax::Postgres).join("; ");
    let expected_up = [
        r#"ALTER TABLE s2.cars ALTER COLUMN name2 TYPE TEXT NOT NULL"#,
        r#"ALTER TABLE s2.cars RENAME name2 TO name"#,
    ]
    .join("; ");
    assert_eq!(sql, expected_up);
}
