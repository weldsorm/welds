use super::PostgresClient;
use super::PostgresParam;
use crate::Row;
use crate::errors::Result;
use crate::params::Param;
use futures::Stream;
use futures::StreamExt;
use futures_core::stream::BoxStream;
use sqlx::Postgres;
use std::pin::Pin;

pub(crate) struct PgRowStream<'e> {
    sql: String,
    inner_stream: Option<BoxStream<'e, Result<Row>>>,
}

impl<'e> PgRowStream<'e> {
    pub(crate) fn new<'params>(
        conn: &PostgresClient,
        sql: &str,
        params: &[&'params (dyn Param + Sync)],
    ) -> Pin<Box<Self>>
    where
        'params: 'e,
    {
        let mut row_stream = Box::new(Self {
            sql: sql.to_string(),
            inner_stream: None,
        });

        // WARNING: self ref to access SQL string while running
        let sql_str_self_ref: &str = unsafe {
            let sql_ptr: *const String = &row_stream.sql;
            &*sql_ptr
        };

        let mut query = sqlx::query::<Postgres>(sql_str_self_ref);
        for param in params {
            query = PostgresParam::add_param(*param, query);
        }
        let inner_stream = query
            .fetch(&*conn.pool)
            .map(|x| x.map_err(|x| crate::Error::Sqlx(x)).map(Row::from))
            .boxed();

        row_stream.inner_stream = Some(inner_stream);

        // return the box, pinned without moving it
        Box::into_pin(row_stream)
    }
}

impl<'e> Stream for PgRowStream<'e> {
    type Item = Result<Row>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let stream = &mut self.get_mut().inner_stream.as_mut().unwrap();
        stream.poll_next_unpin(cx)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let stream = self.inner_stream.as_ref().unwrap();
        Stream::size_hint(stream)
    }
}
