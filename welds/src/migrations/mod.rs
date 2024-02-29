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

//pub use tablemod::{alter_table, Table};

type MigrationFn = fn(state: &TableState) -> Result<Box<dyn MigrationWriter>>;

//  /// Call to migrate your database to the latest schema you have defined
//  pub async fn migrate_up<'c, 'args, C, DB>(conn: &C, migrations: &[MigrationFn<DB>]) -> Result<()>
//  where
//      C: Connection<DB> + IntoTrans<DB>,
//      Transaction<'args, DB>: Connection<DB>,
//      <DB as HasArguments<'args>>::Arguments: IntoArguments<'args, DB>,
//      DB: Database + TableScan,
//      create_table::TableBuilder: MigrationWriter<DB>,
//  {
//      // get a list of all the migrations that have already been ran
//      let done = get_do_migrations(conn).await?;
//      println!("DONE: {:?}", done);
//
//      for migration_builder in migrations {
//          let trans = conn.begin().await?;
//
//          // Get the current state of the database
//          // This way to migrations can know what is there when writing their changes
//          let state = unsafe {
//              let t_ptr: *const Transaction<DB> = &trans;
//              let t: &Transaction<DB> = &*t_ptr;
//              get_state(t).await?
//          };
//
//          let m = migration_builder(&state)?;
//          let statements = m.up_sql();
//
//          for sql in &statements {
//              log::debug!("MIGRATE UP: {}", sql);
//              // ignoring the lifetimes
//              unsafe {
//                  let s_ptr: *const String = sql;
//                  let s: &String = &*s_ptr;
//                  let t_ptr: *const Transaction<DB> = &trans;
//                  let t: &Transaction<DB> = &*t_ptr;
//                  t.execute(s, Default::default()).await?;
//              };
//          }
//
//          trans.commit().await?;
//      }
//
//      Ok(())
//  }
//
//  //  async fn mark_as_done<'a, 'c, 't, 'args1, 'args2, DB, C>(
//  //      name: &str,
//  //      down: &str,
//  //      trans: &Transaction<'t, DB>,
//  //  ) -> Result<()>
//  //  where
//  //      'a: 'args1,
//  //      'c: 'args1,
//  //      C: Connection<DB>,
//  //      Transaction<'args, DB>: Connection<DB>,
//  //      <DB as HasArguments<'args1>>::Arguments: IntoArguments<'args2, DB>,
//  //      DB: Database,
//  //  {
//  //      let p = NextParam::new::<DB>();
//  //
//  //      let sql = format!(
//  //          "INSERT INTO _welds_migrations (name, down) VALUES ({}, {})",
//  //          p.next(),
//  //          p.next()
//  //      );
//  //
//  //      let mut args: <DB as HasArguments>::Arguments = Default::default();
//  //      args.add(name);
//  //      args.add(down);
//  //
//  //
//  //      //// make sure the migration table exists
//  //      //let make_table_sql = migration_table::<DB>().up_sql().join(";");
//  //      //// if table already exists ignore the error
//  //      //let _ = conn.execute(&make_table_sql, Default::default()).await;
//  //      //// fetch the list of migrations
//  //      //let sql = "SELECT name FROM _welds_migrations";
//  //      //let rows = conn.fetch_rows(sql, Default::default()).await?;
//  //      //// Build a list of all the migrations that have ran
//  //      //let mut list: Vec<String> = Vec::default();
//  //      //for row in rows {
//  //      //    list.push(row.get(0));
//  //      //}
//  //
//  //      Ok(())
//  //  }
//
//  async fn get_do_migrations<C, DB>(conn: &C) -> Result<Vec<String>>
//  where
//      C: Connection<DB>,
//      DB: Database,
//      create_table::TableBuilder: MigrationWriter<DB>,
//  {
//      // make sure the migration table exists
//      let make_table_sql = migration_table::<DB>().up_sql().join(";");
//      // if table already exists ignore the error
//      let _ = conn.execute(&make_table_sql, Default::default()).await;
//      // fetch the list of migrations
//      let sql = "SELECT name FROM _welds_migrations";
//      let rows = conn.fetch_rows(sql, Default::default()).await?;
//      // Build a list of all the migrations that have ran
//      let mut list: Vec<String> = Vec::default();
//      for row in rows {
//          list.push(row.get(0));
//      }
//      Ok(list)
//  }
//
//  fn migration_table<DB>() -> Box<dyn MigrationWriter<DB>>
//  where
//      create_table::TableBuilder: MigrationWriter<DB>,
//  {
//      use types::Type;
//      let m = create_table("_welds_migrations")
//          .column(|c| c("name", Type::String(255)).create_unique_index())
//          .column(|c| c("rollback", Type::Text));
//      Box::new(m)
//  }

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
