use super::*;

// Test Object that can be used to write SQL
use crate::WeldsModel;

#[derive(Debug, WeldsModel)]
#[welds(schema = "da_schemaname", table = "da_tablename")]
#[welds_path(crate)] // needed only within the welds crate.
struct Product {
    #[welds(rename = "dbname")]
    pub name: String,
}

// Tests

#[test]
fn should_write_basic_select() {
    let q = QueryBuilder::<Product>::new();
    assert_eq!(
        "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1",
        q.to_sql(Syntax::Postgres)
    );
}

#[test]
fn should_exec_basic_select() {
    let q = QueryBuilder::<Product>::new();
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Postgres);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1",
        &ran_sql
    );
}

#[test]
fn should_write_basic_count() {
    let q = QueryBuilder::<Product>::new();
    assert_eq!(
        "SELECT CAST( COUNT(t1.*) as BIGINT ) FROM da_schemaname.da_tablename t1",
        q.to_sql_count(Syntax::Postgres)
    );
}

#[test]
fn should_exec_basic_count() {
    let q = QueryBuilder::<Product>::new();
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Postgres);
        let _ = q.count(&client).await;
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT CAST( COUNT(t1.*) as BIGINT ) FROM da_schemaname.da_tablename t1",
        &ran_sql
    );
}

#[test]
fn should_be_able_to_add_basic_where() {
    //making sure it compiles, and doesn't barf when running
    let _q = QueryBuilder::<Product>::new().where_col(|c| c.name.equal("bla"));
}

#[test]
fn should_be_able_to_write_sql_with_basic_where() {
    let q = QueryBuilder::<Product>::new().where_col(|c| c.name.equal("bla"));
    let sql = q.to_sql(Syntax::Sqlite);
    assert_eq!(
        "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname = ? )",
        &sql
    );
}

#[test]
fn should_be_able_to_write_sql_count_with_basic_where() {
    let q = QueryBuilder::<Product>::new().where_col(|c| c.name.equal("bla"));
    let sql = q.to_sql_count(Syntax::Sqlite);
    assert_eq!(
        "SELECT CAST( COUNT(*) as BIGINT ) FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname = ? )",
        &sql
    );
}

#[test]
fn should_exec_basic_with_where() {
    let q = QueryBuilder::<Product>::new().where_col(|c| c.name.equal("bla"));
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Mssql);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname = @p1 )",
        &ran_sql
    );
}

#[test]
fn sqlite_syntax_should_be_translated_to_like() {
    let q = QueryBuilder::<Product>::new().where_col(|c| c.name.ilike("bla"));
    let ran_sql = futures::executor::block_on(async move {
        let client = welds_connections::noop::build(Syntax::Sqlite);
        q.run(&client).await.unwrap();
        client.last_sql()
    })
    .unwrap();
    assert_eq!(
        "SELECT t1.\"dbname\" FROM da_schemaname.da_tablename t1 WHERE ( t1.dbname like ? )",
        &ran_sql
    );
}
