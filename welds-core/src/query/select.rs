use crate::errors::Result;
use crate::query::clause::QueryBuilderAdder;
use crate::table::TableInfo;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use sqlx::QueryBuilder;
use std::marker::PhantomData;

pub struct SelectBuilder<'schema, T, S, DB: sqlx::Database> {
    _t: PhantomData<T>,
    _s: PhantomData<S>,
    wheres: Vec<Box<dyn QueryBuilderAdder<'schema, DB>>>,
    qb: Option<QueryBuilder<'schema, DB>>,
}

impl<'schema, 'args, T, S, DB> SelectBuilder<'schema, T, S, DB>
where
    S: Default + TableInfo,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
    DB: sqlx::Database,
{
    pub fn new() -> Self {
        Self {
            _t: Default::default(),
            _s: Default::default(),
            wheres: Vec::default(),
            qb: None,
        }
    }

    pub fn where_col(mut self, lam: impl Fn(S) -> Box<dyn QueryBuilderAdder<'schema, DB>>) -> Self {
        let c = S::default();
        let qba = lam(c);
        self.wheres.push(qba);
        self
    }

    fn build(&mut self) {
        self.qb = Some(sqlx::QueryBuilder::<DB>::new(""));
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

    pub fn to_sql(&mut self) -> String {
        self.build();
        let mut qb: Option<QueryBuilder<DB>> = None;
        std::mem::swap(&mut qb, &mut self.qb);
        let qb = qb.unwrap();
        qb.into_sql()
    }

    pub async fn run<'q, 'ex, 'e, E>(&'q mut self, exec: &'ex E) -> Result<Vec<T>>
    where
        'q: 'args,
        &'ex E: sqlx::Executor<'e, Database = DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
    {
        self.build();
        let qb = self.qb.as_mut().unwrap();
        let query = qb.build_query_as::<T>();
        let data = query.fetch_all(exec).await?;
        Ok(data)
    }
}
