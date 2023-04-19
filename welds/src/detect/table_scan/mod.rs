pub trait TableScan {
    /// returns the sql needed to get a list of table in the database
    /// a unique list is build from all the sql commands provided
    fn table_scan_sql() -> &'static str;
    fn single_table_scan_sql() -> &'static str;
    fn fk_scan_sql() -> &'static str;
}

#[cfg(feature = "postgres")]
impl TableScan for sqlx::Postgres {
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

#[cfg(feature = "mysql")]
impl TableScan for sqlx::MySql {
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

#[cfg(feature = "mssql")]
impl TableScan for sqlx::Mssql {
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

#[cfg(feature = "sqlite")]
impl TableScan for sqlx::Sqlite {
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
