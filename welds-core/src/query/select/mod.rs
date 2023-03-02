use super::clause::{DbParam, NextParam};
use crate::errors::Result;
use crate::query::clause::ClauseAdder;
use crate::table::TableInfo;
use sqlx::database::HasArguments;
use sqlx::query::{Query, QueryAs};
use sqlx::IntoArguments;
use sqlx::Row;
use std::marker::PhantomData;

pub struct SelectBuilder<'schema, T, S, DB: sqlx::Database> {
    _t: PhantomData<T>,
    _s: PhantomData<S>,
    wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    // This is needed for lifetime issues, remove if you can
    history: Vec<String>,
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
            history: Default::default(),
        }
    }

    pub fn where_col(mut self, lam: impl Fn(S) -> Box<dyn ClauseAdder<'schema, DB>>) -> Self {
        let c = S::default();
        let qba = lam(c);
        self.wheres.push(qba);
        self
    }

    pub fn to_sql(&mut self) -> String
    where
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = None;
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();

        join_sql_parts(&[
            build_head_select::<S>(),
            build_where(&next_params, &mut args, wheres),
        ])
    }

    pub async fn count<'q, 'ex, 'e, E>(&'q mut self, exec: &'ex E) -> Result<u64>
    where
        'q: 'args,
        &'ex E: sqlx::Executor<'e, Database = DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        i64: sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
        DB: DbParam,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();

        let sql = join_sql_parts(&[
            build_head_count::<S>(),
            build_where(&next_params, &mut args, wheres),
        ]);

        // lifetime hack
        self.history.push(sql);
        let sql = self.history.last().unwrap();

        // Run the query
        let query: Query<DB, <DB as HasArguments>::Arguments> =
            sqlx::query_with(sql, args.unwrap());
        let row = query.fetch_one(exec).await?;
        let count: i64 = row.try_get(0)?;
        Ok(count as u64)
    }

    pub async fn run<'q, 'ex, 'e, E>(&'q mut self, exec: &'ex E) -> Result<Vec<T>>
    where
        'q: 'args,
        &'ex E: sqlx::Executor<'e, Database = DB>,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
        DB: DbParam,
    {
        let mut args: Option<<DB as HasArguments>::Arguments> = Some(Default::default());
        let next_params = NextParam::new::<DB>();
        let wheres = self.wheres.as_slice();

        let sql = join_sql_parts(&[
            build_head_select::<S>(),
            build_where(&next_params, &mut args, wheres),
        ]);

        // lifetime hack
        self.history.push(sql);
        let sql = self.history.last().unwrap();

        // Run the query
        let q: QueryAs<DB, T, <DB as HasArguments>::Arguments> =
            sqlx::query_as_with(sql, args.unwrap());
        let data = q.fetch_all(exec).await?;
        Ok(data)
    }
}

fn join_sql_parts(parts: &[Option<String>]) -> String {
    // Join al the parts into
    let sql: Vec<&str> = parts
        .iter()
        .filter_map(|x| x.as_ref().map(|x| x.as_str()))
        .collect();
    let sql: String = sql.as_slice().join(" ");
    sql
}

fn build_where<'schema, 'args, DB>(
    next_params: &NextParam,
    args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    wheres: &[Box<dyn ClauseAdder<'schema, DB>>],
) -> Option<String>
where
    DB: sqlx::Database,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
{
    if wheres.len() == 0 {
        return None;
    }
    let mut where_sql: Vec<String> = Vec::default();
    where_sql.push("WHERE".to_owned());
    for clause in wheres {
        if let Some(args) = args {
            clause.bind(args);
        }
        if let Some(p) = clause.clause(&next_params) {
            where_sql.push(p);
        }
    }
    Some(where_sql.join(" "))
}

fn build_head_select<S: TableInfo>() -> Option<String> {
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");
    let cols = S::columns().join(", ");
    head.push(&cols);
    head.push("FROM");
    head.push(S::identifier());
    Some(head.join(" "))
}

fn build_head_count<S: TableInfo>() -> Option<String> {
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT cast(count(*) as bigint) FROM");
    head.push(S::identifier());
    Some(head.join(" "))
}
