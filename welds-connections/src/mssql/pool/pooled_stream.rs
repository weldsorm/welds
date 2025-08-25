use super::super::MssqlParam;
use super::Param;
use super::TiberiusConn;
use crate::errors::Result;
use crate::row::Row;
use async_mutex::MutexGuard;
use futures::Stream;
use futures::StreamExt;
use futures_core::stream::BoxStream;
use std::pin::Pin;
use tiberius::ToSql;

pub(crate) struct PooledConnectionStream<'e> {
    // hold the guard for the lifetime of this stream
    guard: MutexGuard<'e, Option<TiberiusConn>>,
    // Points at `self.guard` which we know is itself non-null.
    // Using an Option so we can pin in place then fill in the value.
    stream: Option<BoxStream<'e, Result<Row>>>,
}

impl<'e> PooledConnectionStream<'e> {
    pub async fn new<'params>(
        guard: MutexGuard<'e, Option<TiberiusConn>>,
        sql: &str,
        params: &[&'params (dyn Param + Sync)],
    ) -> Pin<Box<Self>>
    where
        'params: 'e,
    {
        // pin the new stream so we known it's conn isn't going to move.
        let mut gs = Box::new(PooledConnectionStream {
            stream: None,
            guard,
        });

        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }

        // WARNING: self reference so we can run the QueryStream while we hold
        // the connection until we are finished using it.
        let stream_result = unsafe {
            let conn: *mut TiberiusConn = gs.guard.as_mut().unwrap();
            (&mut *conn).query(sql, &args).await
        };

        // stream the stream or a stream of its error
        let stream: BoxStream<'e, Result<Row>> = match stream_result {
            Err(err) => futures::stream::iter([Err(crate::errors::Error::Tiberius(err))]).boxed(),
            Ok(s) => s
                .into_row_stream()
                .map(|x| x.map_err(crate::Error::Tiberius).map(Row::from))
                .boxed(),
        };

        // save off the self ref
        gs.stream = Some(stream);

        // return the box, pinned without moving it
        Box::into_pin(gs)
    }
}

impl<'e> Stream for PooledConnectionStream<'e> {
    type Item = Result<Row>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let stream = &mut self.get_mut().stream.as_mut().unwrap();
        stream.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let stream = self.stream.as_ref().unwrap();
        Stream::size_hint(stream)
    }
}
