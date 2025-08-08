use super::MssqlParam;
use super::ToSql;
use super::pool::PooledConnection;
use crate::Row;
use crate::errors::Result;
use crate::{Param, StreamClient};
use futures::Stream;
use futures::StreamExt;
use futures_core::stream::BoxStream;
use std::pin::Pin;

pub(crate) struct MssqlClientStream<'e> {
    conn: PooledConnection,
    // Points at `self.guard` which we know is itself non-null.
    // Using an Option so we can pin in place then fill in the value.
    stream: Option<BoxStream<'e, Result<Row>>>,
}

impl<'e> MssqlClientStream<'e> {
    pub async fn new<'params>(
        conn: PooledConnection,
        sql: &str,
        params: &[&'params (dyn Param + Sync)],
    ) -> Pin<Box<Self>>
    where
        'params: 'e,
    {
        // pin the new stream so we known it's conn isn't going to move.
        let mut gs = Box::new(MssqlClientStream { conn, stream: None });

        let mut args: Vec<&dyn ToSql> = Vec::new();
        for &p in params {
            args = MssqlParam::add_param(p, args);
        }

        // WARNING: self reference so we can run the QueryStream while we hold
        // the connection until we are finished using it.
        let stream = unsafe {
            let conn: *mut PooledConnection = &mut gs.conn;
            (&*conn).stream(sql, params).await
        };

        //save off the stream so we can read it when called
        gs.stream = Some(stream);

        // return the box, pinned without moving it
        Box::into_pin(gs)
    }
}

impl<'e> Stream for MssqlClientStream<'e> {
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
