use sqlx::mssql::MssqlRow;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;

pub trait FromRow:
    Send
    + Unpin
    + for<'r> sqlx::FromRow<'r, SqliteRow>
    + for<'r> sqlx::FromRow<'r, MySqlRow>
    + for<'r> sqlx::FromRow<'r, MssqlRow>
    + for<'r> sqlx::FromRow<'r, PgRow>
{
}

impl<T> FromRow for T where
    T: Send
        + Unpin
        + for<'r> sqlx::FromRow<'r, SqliteRow>
        + for<'r> sqlx::FromRow<'r, MySqlRow>
        + for<'r> sqlx::FromRow<'r, MssqlRow>
        + for<'r> sqlx::FromRow<'r, PgRow>
{
}

pub trait ToRow<'args>:
    Send
    + sqlx::Type<sqlx::Sqlite>
    + sqlx::Type<sqlx::MySql>
    + sqlx::Type<sqlx::Postgres>
    + sqlx::Type<sqlx::Mssql>
    + sqlx::Encode<'args, sqlx::Sqlite>
    + sqlx::Encode<'args, sqlx::MySql>
    + sqlx::Encode<'args, sqlx::Postgres>
    + sqlx::Encode<'args, sqlx::Mssql>
{
}

impl<'args, T> ToRow<'args> for T where
    T: Send
        + sqlx::Type<sqlx::Sqlite>
        + sqlx::Type<sqlx::MySql>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::Type<sqlx::Mssql>
        + sqlx::Encode<'args, sqlx::Sqlite>
        + sqlx::Encode<'args, sqlx::MySql>
        + sqlx::Encode<'args, sqlx::Postgres>
        + sqlx::Encode<'args, sqlx::Mssql>
{
}
