use crate::detect::{self, TableDef};
use crate::errors::Result;
use crate::Client;
use crate::Syntax;
use crate::TransactStart;

// object => strings
pub mod writers;

mod create_table;
pub mod types;
pub use create_table::create_table;
mod tablemod;
pub use tablemod::alter_table;

type MigrationFn = fn(state: &TableState) -> Result<(&'static str, Box<dyn MigrationWriter>)>;

/// Migrate your database to the latest in the list of migrations
pub async fn up(client: &(dyn TransactStart), migrations: &[MigrationFn]) -> Result<()> {
    let trans = client.begin().await?;

    // make sure the migration table exists
    let setup = migration_table().up_sql(trans.syntax());
    for sql in setup {
        let _ = trans.execute(&sql, &[]).await;
    }

    for lambda in migrations {
        let state = get_state(&trans).await?;
        let (name, m) = lambda(&state)?;
        //MigrationStep::find_by_id(&client, name).await;
        //
    }

    trans.commit().await?;
    Ok(())
}

/// Migrate your migrations down one migrations
pub async fn down(client: &(dyn TransactStart), migrations: &[MigrationFn]) -> Result<()> {
    //let trans = client.begin().await?;

    //// make sure the migration table exists
    //let setup = migration_table().up_sql(trans.syntax());
    //for sql in setup {
    //    let _ = trans.execute(&sql, &[]).await;
    //}

    //for lambda in migrations {
    //    let state = get_state(&trans).await?;
    //    let (name, m) = lambda(&state)?;
    //    //MigrationStep::find_by_id(&client, name).await;
    //    //
    //}

    //trans.commit().await?;
    //Ok(())
    todo!()
}

fn migration_table() -> Box<dyn MigrationWriter>
where
    create_table::TableBuilder: MigrationWriter,
{
    use types::Type;
    let m = create_table("_welds_migrations")
        .column(|c| c("name", Type::StringSized(255)).create_unique_index())
        .column(|c| c("rollback", Type::Text));
    Box::new(m)
}

/// define a struct used to get info about what migrations have ran
// use crate::WeldsModel;
// #[derive(Debug, WeldsModel)]
// #[welds(table = "_welds_migrations")]
// #[welds_path(crate)] // needed only within the welds crate.
// pub struct MigrationStep {
//     #[welds(primary_key)]
//     pub name: String,
//     pub rollback: String,
// }

async fn get_state(client: &dyn Client) -> Result<TableState> {
    let state = detect::find_tables(client).await?;
    Ok(TableState(state))
}

// The current state/schema of all tables/views in the database
pub struct TableState(pub(crate) Vec<TableDef>);

pub trait MigrationWriter {
    fn up_sql(&self, syntax: Syntax) -> Vec<String>;
    fn down_sql(&self, syntax: Syntax) -> Vec<String>;
}
