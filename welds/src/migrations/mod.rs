use crate::Client;
use crate::Syntax;
use crate::TransactStart;
use crate::detect::{self, TableDef};
use crate::errors::Result;
use crate::errors::WeldsError;
use crate::state::DbState;
use std::collections::HashSet;

// All the things a dev needs to start writing migrations
pub mod prelude;

// object => sql strings
pub mod writers;

mod create_table;
pub mod types;
pub use create_table::create_table;
mod tablemod;
pub use tablemod::change_table;
mod utils;
use crate::connections::Transaction;
mod manual;
pub use manual::Manual;
mod indexes;
pub use indexes::create_index;

pub type MigrationFn = fn(state: &TableState) -> Result<MigrationStep>;

/// Migrate your database to the latest in the list of migrations
#[maybe_async::maybe_async]
pub async fn up(client: &dyn TransactStart, migrations: &[MigrationFn]) -> Result<()> {
    //make the migration table if needed
    {
        let setup_trans = client.begin().await?;
        let _r = setup_migration_table(setup_trans).await;
    }

    let trans = client.begin().await?;

    let mut seen = HashSet::new();

    for lambda in migrations {
        let state = get_state(&trans).await?;
        let step = lambda(&state)?;

        if seen.contains(step.name) {
            Err(WeldsError::DuplicateMigration)?;
        }

        seen.insert(step.name.to_string());

        let mut found = MigrationLog::where_col(|c| c.name.equal(step.name))
            .run(&trans)
            .await?;
        let found = found.pop();
        if found.is_none() {
            // run the migration step
            for part in step.writer.up_sql(trans.syntax()) {
                trans.execute(&part, &[]).await?;
            }

            let mut mlog = DbState::new_uncreated(MigrationLog {
                id: 0,
                name: step.name.to_string(),
                when_applied: unixtime() as i64,
                rollback_sql: step.writer.down_sql(trans.syntax()).join("; "),
            });
            mlog.save(&trans).await?;
        }
    }

    trans.commit().await?;
    Ok(())
}

#[maybe_async::maybe_async]
async fn setup_migration_table(trans: Transaction<'_>) -> Result<()> {
    // make sure the migration table exists
    let setup = migration_table().up_sql(trans.syntax());
    for sql in setup {
        let _ = trans.execute(&sql, &[]).await?;
    }
    trans.commit().await?;
    Ok(())
}

fn unixtime() -> u128 {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("System time is before the Unix epoch!"),
    }
}

/// Rolls back the last migration that ran.
/// return the name of the migration that rolled back
/// None, there were not more migrations to rollback
#[maybe_async::maybe_async]
pub async fn down_last(client: &dyn TransactStart) -> Result<Option<String>> {
    //make the migration table if needed
    {
        let setup_trans = client.begin().await?;
        let _r = setup_migration_table(setup_trans).await;
    }

    let trans = client.begin().await?;

    let mut last = MigrationLog::all()
        .order_by_desc(|x| x.id)
        .limit(1)
        .run(&trans)
        .await?;
    let last = last.pop();

    let mut last = match last {
        Some(x) => x,
        None => return Ok(None),
    };

    let down = last.rollback_sql.as_str();
    let parts = utils::split_sql_commands(down);

    for sql in parts {
        trans.execute(&sql, &[]).await?;
    }
    last.delete(&trans).await?;
    trans.commit().await?;

    Ok(Some(last.name.to_string()))
}

/// Rolls back the given migration.
/// return the name of the migration that rolled back
/// None, there were no matching migrations to rollback
#[maybe_async::maybe_async]
pub async fn down(client: &dyn TransactStart, name: impl Into<String>) -> Result<Option<String>> {
    //make the migration table if needed
    {
        let setup_trans = client.begin().await?;
        let _r = setup_migration_table(setup_trans).await;
    }

    let trans = client.begin().await?;
    let name: String = name.into();
    let name = name.as_str();

    let mut mlog = MigrationLog::all()
        .where_col(|c| c.name.equal(name))
        .order_by_desc(|x| x.id)
        .limit(1)
        .run(&trans)
        .await?;
    let mlog = mlog.pop();

    let mut mlog = match mlog {
        Some(x) => x,
        None => return Ok(None),
    };

    let down = mlog.rollback_sql.as_str();
    let parts = utils::split_sql_commands(down);

    for sql in parts {
        trans.execute(&sql, &[]).await?;
    }
    mlog.delete(&trans).await?;
    trans.commit().await?;
    Ok(Some(mlog.name.as_str().to_string()))
}

fn migration_table() -> Box<dyn MigrationWriter>
where
    create_table::TableBuilder: MigrationWriter,
{
    use types::Type;
    let m = create_table("_welds_migrations")
        .id(|c| c("id", Type::IntBig))
        .column(|c| c("name", Type::StringSized(255)).create_unique_index())
        .column(|c| c("when_applied", Type::IntBig))
        .column(|c| c("rollback_sql", Type::Text));
    Box::new(m)
}

// define a struct used to get info about what migrations have ran
use crate::WeldsModel;
#[derive(Debug, WeldsModel)]
#[welds(table = "_welds_migrations")]
#[welds_path(crate)] // needed only within the welds crate.
pub struct MigrationLog {
    #[welds(primary_key)]
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) when_applied: i64,
    pub(crate) rollback_sql: String,
}

#[maybe_async::maybe_async]
async fn get_state(client: &dyn Client) -> Result<TableState> {
    let state = detect::find_all_tables(client).await?;
    Ok(TableState(state))
}

/// The current state/schema of all tables/views in the database
pub struct TableState(pub(crate) Vec<TableDef>);

/// Implementers of this Trait are able to write the SQL for migrations
pub trait MigrationWriter {
    fn up_sql(&self, syntax: Syntax) -> Vec<String>;
    fn down_sql(&self, syntax: Syntax) -> Vec<String>;
}

/// Once migration step.
/// This is what is used by up/down
/// to make the changes to the database
pub struct MigrationStep {
    pub(crate) writer: Box<dyn MigrationWriter>,
    pub(crate) name: &'static str,
}

impl MigrationStep {
    pub fn new<T>(name: &'static str, writer: T) -> Self
    where
        T: MigrationWriter,
        T: 'static,
    {
        MigrationStep {
            name,
            writer: Box::new(writer),
        }
    }
}
