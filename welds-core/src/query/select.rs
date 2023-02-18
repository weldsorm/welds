use crate::database::Pool;
use crate::errors::Result;
use crate::query::clause::QueryBuilderAdder;
use crate::query::GenericQueryBuilder;
use crate::table::TableInfo;
use sqlx::mssql::MssqlRow;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;
use std::marker::PhantomData;

pub struct SelectBuilder<'args, T, S> {
    _t: PhantomData<T>,
    _s: PhantomData<S>,
    wheres: Vec<Box<dyn QueryBuilderAdder<'args>>>,
    qb: Option<GenericQueryBuilder<'args>>,
}

impl<'schema, 'args, T, S> SelectBuilder<'schema, T, S>
where
    S: Default + TableInfo,
    T: Send
        + Unpin
        + for<'r> FromRow<'r, SqliteRow>
        + for<'r> FromRow<'r, MySqlRow>
        + for<'r> FromRow<'r, MssqlRow>
        + for<'r> FromRow<'r, PgRow>,
{
    pub fn new() -> Self {
        Self {
            _t: Default::default(),
            _s: Default::default(),
            wheres: Vec::default(),
            qb: None,
        }
    }

    pub fn where_col(mut self, lam: impl Fn(S) -> Box<dyn QueryBuilderAdder<'schema>>) -> Self {
        let c = S::default();
        let qba = lam(c);
        self.wheres.push(qba);
        self
    }

    fn build(&mut self, conn: &Pool) {
        self.qb = Some(conn.querybuilder());
        let qb = self.qb.as_mut().unwrap();
        let wheres: &[_] = self.wheres.as_ref();
        qb.push("SELECT ");

        let cols = S::columns().join(", ");
        qb.push(cols);

        qb.push(" FROM ");
        qb.push(S::identifier());

        let where_len = wheres.len();
        if where_len > 0 {
            qb.push(" WHERE (");
            let mut i = 0;
            for clause in wheres {
                i += 1;
                clause.append_to(qb);
                if i < where_len {
                    qb.push(" AND ");
                }
            }
            qb.push(" )");
        }
    }

    pub fn to_sql(&mut self, conn: &Pool) -> String {
        self.build(conn);
        let mut qb: Option<GenericQueryBuilder> = None;
        std::mem::swap(&mut qb, &mut self.qb);
        let qb = qb.unwrap();
        qb.into_sql()
    }

    pub async fn run<'q>(&'q mut self, conn: &Pool) -> Result<Vec<T>>
    where
        'q: 'args,
    {
        use crate::query::generic_query_builder::run;
        self.build(conn);
        let qb = self.qb.as_mut().unwrap();
        let out = run::<T>(qb, conn).await?;
        Ok(out)
    }
}
