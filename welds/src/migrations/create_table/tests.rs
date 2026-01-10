use super::*;
use crate::Syntax;
use crate::migrations::MigrationWriter;

#[test]
fn should_create_basic_table() {
    let m = create_table("s1.MyTable")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( id INT AUTO_INCREMENT PRIMARY KEY NOT NULL, name VARCHAR(255) NOT NULL )"#;
    assert_eq!(sql, expected.trim());

    //postgres
    let sql2 = MigrationWriter::up_sql(&m, Syntax::Postgres).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( "id" SERIAL PRIMARY KEY NOT NULL, "name" TEXT NOT NULL )"#;
    assert_eq!(sql2, expected.trim());

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( "id" INT IDENTITY(1,1) PRIMARY KEY NOT NULL, "name" NVARCHAR(MAX) NOT NULL )"#;
    assert_eq!(sql, expected.trim());

    //sqlite
    let sql2 = MigrationWriter::up_sql(&m, Syntax::Sqlite).join("; ");
    let expected = r#"
    CREATE TABLE s1.MyTable ( "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, "name" TEXT NOT NULL )"#;
    assert_eq!(sql2, expected.trim());
}

#[test]
fn should_be_able_to_make_a_fk() {
    let m = create_table("s1.MyTable")
        .id(|c| c("id", Type::Int))
        .column(|c| {
            c("other_id", Type::Int).create_foreign_key("others", "o_id", OnDelete::Cascade)
        });

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mysql).pop().unwrap();
    let expected = r#"ALTER TABLE s1.MyTable ADD CONSTRAINT fk_MyTable_other_id FOREIGN KEY (other_id) REFERENCES others (o_id) ON DELETE CASCADE"#;
    assert_eq!(sql, expected.trim());

    //postgres
    let sql = MigrationWriter::up_sql(&m, Syntax::Postgres).pop().unwrap();
    let expected = r#"ALTER TABLE s1.MyTable ADD CONSTRAINT fk_MyTable_other_id FOREIGN KEY ("other_id") REFERENCES others ("o_id") ON DELETE CASCADE"#;
    assert_eq!(sql, expected.trim());

    //mysql
    let sql = MigrationWriter::up_sql(&m, Syntax::Mssql).pop().unwrap();
    let expected = r#"ALTER TABLE s1.MyTable ADD CONSTRAINT fk_MyTable_other_id FOREIGN KEY ("other_id") REFERENCES others ("o_id") ON DELETE CASCADE"#;
    assert_eq!(sql, expected.trim());

    //sqlite
    let sql2 = MigrationWriter::up_sql(&m, Syntax::Sqlite).pop().unwrap();
    // NOTE sqlite FKs are made in CREATE TABLE no ALTER
    let expected = r#""#;
    assert_eq!(sql2, expected.trim());
}

#[test]
fn should_drop_basic_table() {
    let m = create_table("s1.MyTable")
        .id(|c| c("id", Type::Int))
        .column(|c| c("name", Type::String));
    let expected = r#"DROP TABLE s1.MyTable"#;

    //mysql
    let sql = MigrationWriter::down_sql(&m, Syntax::Mysql).join("; ");
    assert_eq!(sql, expected.trim());

    //postgres
    let sql2 = MigrationWriter::down_sql(&m, Syntax::Postgres).join("; ");
    assert_eq!(sql2, expected.trim());
}
