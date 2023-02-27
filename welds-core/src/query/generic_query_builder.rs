use crate::errors::Result;
use std::fmt::Display;

#[cfg(feature = "sqlite")]
type QB1<'args> = sqlx::QueryBuilder<'args, sqlx::Sqlite>;
#[cfg(feature = "mssql")]
type QB2<'args> = sqlx::QueryBuilder<'args, sqlx::Mssql>;
#[cfg(feature = "mysql")]
type QB3<'args> = sqlx::QueryBuilder<'args, sqlx::MySql>;
#[cfg(feature = "postgres")]
type QB4<'args> = sqlx::QueryBuilder<'args, sqlx::Postgres>;

/// This is a pdfwrapper around the sqlx QueryBuilder to allow talking with it without knowing the
/// underlying connection type
pub enum GenericQueryBuilder<'args> {
    #[cfg(feature = "sqlite")]
    Sqlite(QB1<'args>),
    #[cfg(feature = "mssql")]
    Mssql(QB2<'args>),
    #[cfg(feature = "mysql")]
    MySql(QB3<'args>),
    #[cfg(feature = "postgres")]
    Postgres(QB4<'args>),
}

impl<'args> GenericQueryBuilder<'args> {
    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        use GenericQueryBuilder::*;
        match self {
            #[cfg(feature = "sqlite")]
            Sqlite(qb) => {
                qb.push(sql);
            }
            #[cfg(feature = "mssql")]
            Mssql(qb) => {
                qb.push(sql);
            }
            #[cfg(feature = "mysql")]
            MySql(qb) => {
                qb.push(sql);
            }
            #[cfg(feature = "postgres")]
            Postgres(qb) => {
                qb.push(sql);
            }
        };
        self
    }

    pub fn push_bind<T>(&mut self, value: T) -> &mut Self
    where
        T: 'args + crate::row::ToRow<'args>,
    {
        use GenericQueryBuilder::*;
        match self {
            #[cfg(feature = "sqlite")]
            Sqlite(qb) => {
                qb.push_bind(value);
            }
            #[cfg(feature = "mssql")]
            Mssql(qb) => {
                qb.push_bind(value);
            }
            #[cfg(feature = "mysql")]
            MySql(qb) => {
                qb.push_bind(value);
            }
            #[cfg(feature = "postgres")]
            Postgres(qb) => {
                qb.push_bind(value);
            }
        }
        self
    }

    pub fn into_sql(self) -> String {
        use GenericQueryBuilder::*;
        match self {
            #[cfg(feature = "sqlite")]
            Sqlite(qb) => qb.into_sql(),
            #[cfg(feature = "mssql")]
            Mssql(qb) => qb.into_sql(),
            #[cfg(feature = "mysql")]
            MySql(qb) => qb.into_sql(),
            #[cfg(feature = "postgres")]
            Postgres(qb) => qb.into_sql(),
        }
    }
}

pub async fn run<'b, 'q, 'e, 'args, T>(
    gqb: &'b mut GenericQueryBuilder<'args>,
    conn: &crate::database::Pool,
) -> Result<Vec<T>>
where
    'b: 'q,
    T: Send,
    T: Unpin,
    T: crate::row::FromRow,
{
    use GenericQueryBuilder::*;
    match gqb {
        #[cfg(feature = "sqlite")]
        Sqlite(qb) => {
            let conn = conn.as_sqlite().unwrap();
            let query = qb.build_query_as::<T>();
            let data = query.fetch_all(conn).await?;
            Ok(data)
        }

        #[cfg(feature = "mysql")]
        MySql(qb) => {
            let conn = conn.as_mysql().unwrap();
            let query = qb.build_query_as::<T>();
            let data = query.fetch_all(conn).await?;
            Ok(data)
        }

        #[cfg(feature = "mssql")]
        Mssql(qb) => {
            let conn = conn.as_mssql().unwrap();
            let query = qb.build_query_as::<T>();
            let data = query.fetch_all(conn).await?;
            Ok(data)
        }

        #[cfg(feature = "postgres")]
        Postgres(qb) => {
            let conn = conn.as_postgres().unwrap();
            let query = qb.build_query_as::<T>();
            let data = query.fetch_all(conn).await?;
            Ok(data)
        }
    }
}
