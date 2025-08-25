use super::{Client, Param};
use crate::errors::Error::PoolError;
use crate::errors::Result;
use async_mutex::Mutex as AsyncMutex;
use bb8::ManageConnection;
use bb8_tiberius::ConnectionManager;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::yield_now;

#[cfg(feature = "unstable-api")]
pub(crate) mod pooled_stream;

// ******************************************************************************
// Why in the world are we making our own connection pool?
// I known this seams crazy and unnecessary
//
// Long story short, rust doesn't have "async drop"
// we need a way to cleanup a connection before it returns to the pool for others to use
// The need here is if a user drops a connection that has an open transaction.
// This is very common if an Err() is returned from with there code.
// Ideally we would like to rollback that transaction for them.
//
// This pool allows us to "cleanup" a connection before returning it back to the pool
// that is check if it has open transactions that needs a rollback
// ******************************************************************************

pub(crate) type TiberiusConn = tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>;

mod pooledconnection;
pub(crate) use pooledconnection::PooledConnection;

pub(crate) struct Pool {
    mgr: ConnectionManager,
    slots: Vec<Mutex<Slot>>,
    round_robin_next: Mutex<usize>,
    tx: Sender<(TiberiusConn, ConnectionStatus)>,
}

impl Pool {
    /// Create a new connection pool
    pub fn new(mgr: ConnectionManager) -> Arc<Self> {
        let size = 10;
        let mut slots = Vec::with_capacity(size);
        for _ in 0..size {
            slots.push(Mutex::new(Slot::Empty));
        }

        let (tx, rx) = channel();

        let me = Arc::new(Self {
            mgr,
            slots,
            round_robin_next: Mutex::new(0),
            tx,
        });
        let return_ref = me.clone();
        tokio::spawn(async move { pool_return(return_ref, rx).await });
        me
    }

    /// Returns a connection from the connection pool.
    /// useful if you want to do several operations in the same connection
    /// The connection is automatically returned to the pool when dropped
    pub async fn get(&self) -> Result<PooledConnection> {
        // Note: the round_robin_next doesn't have to be perfect.
        // Its is just a good starting point to start looking for the next available connection
        let mut slot_index: usize = {
            let guard = self.round_robin_next.lock().map_err(|_| PoolError)?;
            *guard
        };

        loop {
            let checked_out = try_checkout(slot_index, &self.slots, &self.mgr).await?;

            if let Some(tiberius_conn) = checked_out {
                // save off the next slot to use
                slot_index += 1;
                slot_index %= self.slots.len();
                {
                    let mut guard = self.round_robin_next.lock().unwrap();
                    *guard = slot_index;
                }
                let tiberius_conn = AsyncMutex::new(Some(tiberius_conn));
                return Ok(PooledConnection {
                    status: ConnectionStatus::Clean,
                    tiberius_conn,
                    conn_return: self.tx.clone(),
                });
            }

            slot_index += 1;
            slot_index %= self.slots.len();
            yield_now().await;
        }
    }

    /// returns a string displaying the status of each Slot in the pool
    pub async fn status(&self) -> String {
        let mut display = Vec::with_capacity(self.slots.len() + 2);
        display.push('[');
        for slot in &self.slots {
            let guard = slot.lock().unwrap();
            let slot_guard: &Slot = &guard;
            match slot_guard {
                Slot::Avalable(_) => display.push('A'),
                Slot::Empty => display.push('.'),
                Slot::Checkedout => display.push('C'),
            }
        }
        display.push(']');
        display.iter().collect()
    }
}

/// tries to checkout the connection at a given slot.
/// returns None if the connection in that slot is not available
async fn try_checkout(
    index: usize,
    slots: &[Mutex<Slot>],
    mgr: &ConnectionManager,
) -> Result<Option<TiberiusConn>> {
    // lock the mutex and do the checking out part.
    let slot: Slot = {
        let mut slot_guard = slots[index].lock().unwrap();
        let slot_guard: &mut Slot = slot_guard.deref_mut();

        if let Slot::Checkedout = slot_guard {
            return Ok(None);
        }

        // steel the Slot out of the mutex.
        // This and checkout the slot
        let mut slot = Slot::Checkedout;
        std::mem::swap(&mut slot, slot_guard);
        slot
    };

    // get the connection out of the slot
    let conn: Option<TiberiusConn> = match slot {
        Slot::Avalable(c) => Some(c),
        Slot::Empty => None,
        // this should never happen
        Slot::Checkedout => panic!("double checkout"),
    };

    match conn {
        None => {
            // build a new connection
            log::debug!("MSSQL POOL adding Connection");
            let new_conn = mgr.connect().await?;
            Ok(Some(new_conn))
        }
        Some(mut conn) => {
            // make sure the connection isn't dead
            if conn.execute("SELECT 1", &[]).await.is_err() {
                log::debug!("MSSQL POOL rebuild Connection");
                let new_conn = mgr.connect().await?;
                Ok(Some(new_conn))
            } else {
                Ok(Some(conn))
            }
        }
    }
}

pub(crate) enum Slot {
    Avalable(TiberiusConn),
    Checkedout,
    Empty,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum ConnectionStatus {
    Clean,
    NeedsRollback(String),
}

async fn pool_return(pool: Arc<Pool>, mut rx: Receiver<(TiberiusConn, ConnectionStatus)>) {
    loop {
        let _ = pool_return_inner(pool.clone(), &mut rx).await;
        if Arc::strong_count(&pool) == 1 {
            return;
        }
        yield_now().await;
    }
}

async fn pool_return_inner(
    pool: Arc<Pool>,
    rx: &mut Receiver<(TiberiusConn, ConnectionStatus)>,
) -> Result<()> {
    // wait for a connection to be returned
    let tuple = match rx.try_recv().ok() {
        None => return Ok(()),
        Some(conn) => conn,
    };

    tokio::spawn(async move {
        let (mut conn, status) = tuple;

        // before putting the connection back in the pool, rollback the transaction if needed
        if let ConnectionStatus::NeedsRollback(_trans_name) = status {
            let sql = "WHILE @@TRANCOUNT > 0 BEGIN ROLLBACK TRANSACTION; END";
            let _ = conn.simple_query(sql).await;
        }

        // best guess where to start returning into the pool, doesn't need to be perfect
        let slot_index: usize = {
            let guard = pool.round_robin_next.lock().unwrap();
            *guard
        };
        let mut index = slot_index;

        let itor = pool
            .slots
            .iter()
            .cycle()
            .skip(slot_index)
            .take(pool.slots.len());

        // find a slot to put the conn back into
        for slot in itor {
            let mut slot_guard = slot.lock().unwrap();
            let slot_guard: &mut Slot = slot_guard.deref_mut();

            let mut is_checkout = false;
            if let Slot::Checkedout = slot_guard {
                is_checkout = true;
            }

            if is_checkout {
                // return the connection to the pool
                let mut returning = Slot::Avalable(conn);
                std::mem::swap(&mut returning, slot_guard);
                {
                    // update the next slot to use to point to this slot.
                    let mut guard = pool.round_robin_next.lock().unwrap();
                    *guard = index;
                }
                return;
            }
            index += 1;
            index %= pool.slots.len();
        }

        panic!("unable to return a connection to the connection pool, pool is full");
        //
    });
    Ok(())
}
