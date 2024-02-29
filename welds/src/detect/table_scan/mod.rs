use crate::Syntax;

pub(crate) struct TableScan {
    table_scan_sql: fn() -> &'static str,
    single_table_scan_sql: fn() -> &'static str,
    fk_scan_sql: fn() -> &'static str,
}

impl TableScan {
    pub(crate) fn new(syntax: Syntax) -> TableScan {
        match syntax {
            Syntax::Mysql => TableScan {
                table_scan_sql: MySql::table_scan_sql,
                single_table_scan_sql: MySql::single_table_scan_sql,
                fk_scan_sql: MySql::fk_scan_sql,
            },
            Syntax::Postgres => TableScan {
                table_scan_sql: Postgres::table_scan_sql,
                single_table_scan_sql: Postgres::single_table_scan_sql,
                fk_scan_sql: Postgres::fk_scan_sql,
            },
            Syntax::Sqlite => TableScan {
                table_scan_sql: Sqlite::table_scan_sql,
                single_table_scan_sql: Sqlite::single_table_scan_sql,
                fk_scan_sql: Sqlite::fk_scan_sql,
            },
            Syntax::Mssql => TableScan {
                table_scan_sql: Mssql::table_scan_sql,
                single_table_scan_sql: Mssql::single_table_scan_sql,
                fk_scan_sql: Mssql::fk_scan_sql,
            },
        }
    }

    pub(crate) fn table_scan_sql(&self) -> &'static str {
        (self.table_scan_sql)()
    }

    pub(crate) fn single_table_scan_sql(&self) -> &'static str {
        (self.single_table_scan_sql)()
    }

    pub(crate) fn fk_scan_sql(&self) -> &'static str {
        (self.fk_scan_sql)()
    }
}

struct Postgres;
impl Postgres {
    fn table_scan_sql() -> &'static str {
        include_str!("./postgres.sql")
    }
    fn single_table_scan_sql() -> &'static str {
        include_str!("./postgres_single.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./postgres_fk.sql")
    }
}

struct MySql;
impl MySql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mysql.sql")
    }
    fn single_table_scan_sql() -> &'static str {
        include_str!("./mysql_single.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mysql_fk.sql")
    }
}

struct Mssql;
impl Mssql {
    fn table_scan_sql() -> &'static str {
        include_str!("./mssql.sql")
    }
    fn single_table_scan_sql() -> &'static str {
        include_str!("./mssql_single.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./mssql_fk.sql")
    }
}

struct Sqlite;
impl Sqlite {
    fn table_scan_sql() -> &'static str {
        include_str!("./sqlite.sql")
    }
    fn single_table_scan_sql() -> &'static str {
        include_str!("./sqlite_single.sql")
    }
    fn fk_scan_sql() -> &'static str {
        include_str!("./sqlite_fk.sql")
    }
}
