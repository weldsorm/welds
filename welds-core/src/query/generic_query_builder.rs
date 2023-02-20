use crate::database::Pool;
use crate::errors::Result;
use std::fmt::Display;

type QB1<'args> = sqlx::QueryBuilder<'args, sqlx::Sqlite>;
type QB2<'args> = sqlx::QueryBuilder<'args, sqlx::Mssql>;
type QB3<'args> = sqlx::QueryBuilder<'args, sqlx::MySql>;
type QB4<'args> = sqlx::QueryBuilder<'args, sqlx::Postgres>;

/// This is a pdfwrapper around the sqlx QueryBuilder to allow talking with it without knowing the
/// underlying connection type
pub enum GenericQueryBuilder<'args> {
    Sqlite(QB1<'args>),
    Mssql(QB2<'args>),
    MySql(QB3<'args>),
    Postgres(QB4<'args>),
}

impl<'args> GenericQueryBuilder<'args> {
    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        use GenericQueryBuilder::*;
        match self {
            Sqlite(qb) => {
                qb.push(sql);
            }
            Mssql(qb) => {
                qb.push(sql);
            }
            MySql(qb) => {
                qb.push(sql);
            }
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
            Sqlite(qb) => {
                qb.push_bind(value);
            }
            Mssql(qb) => {
                qb.push_bind(value);
            }
            MySql(qb) => {
                qb.push_bind(value);
            }
            Postgres(qb) => {
                qb.push_bind(value);
            }
        }
        self
    }

    pub fn into_sql(self) -> String {
        use GenericQueryBuilder::*;
        match self {
            Sqlite(qb) => qb.into_sql(),
            Mssql(qb) => qb.into_sql(),
            MySql(qb) => qb.into_sql(),
            Postgres(qb) => qb.into_sql(),
        }
    }
}

pub async fn run<'b, 'q, 'args, T>(
    gqb: &'b mut GenericQueryBuilder<'args>,
    conn: &Pool,
) -> Result<Vec<T>>
where
    'b: 'q,
    T: Send,
    T: Unpin,
    T: crate::row::FromRow,
{
    use GenericQueryBuilder::*;
    match gqb {
                Sqlite(qb) => {
                    let conn = conn.as_sqlite().unwrap();
                    let query = qb.build_query_as::<T>();
                    let data = query.fetch_all(conn).await?;
                    Ok(data)
            },
            Postgres(qb) => {
                    let conn = conn.as_postgres().unwrap();
                    let query = qb.build_query_as::<T>();
                    let data = query.fetch_all(conn).await?;
                    Ok(data)
            },
            _ => todo!()
            //Mssql(qb) => qb.into_sql(),
            //MySql(qb) => qb.into_sql(),
        }
}
