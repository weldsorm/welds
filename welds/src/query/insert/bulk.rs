use crate::errors::Result;
use crate::model_traits::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::Client;
//use crate::Syntax;

/// Executes the query in the database Bulk Inserting values
pub async fn run<T, C>(conn: &C, data: &[T]) -> Result<()>
where
    C: Client,
    T: WriteToArgs + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    if data.is_empty() {
        return Ok(());
    }
    let syntax = conn.syntax();

    // // If postgres do the fast bulk insert
    // if let Syntax::Postgres = syntax {
    //     return run_fast(conn, data).await;
    // }

    let col_writer = ColumnWriter::new(syntax);
    let all_columns = <<T as HasSchema>::Schema as TableColumns>::columns();
    let pks = <<T as HasSchema>::Schema as TableColumns>::primary_keys();
    let columns: Vec<_> = all_columns.iter().filter(|c| !pks.contains(c)).collect();

    let identifier = <<T as HasSchema>::Schema>::identifier().join(".");

    let colnames: Vec<String> = columns
        .iter()
        .map(|c| col_writer.excape(c.name()))
        .collect();
    let colnames = colnames.join(", ");

    // number of to create per insert
    let max_params = NextParam::new(syntax).max_params();
    let chunk_size = max_params as usize / colnames.len();

    for chunk in data.chunks(chunk_size) {
        let next_params = NextParam::new(syntax);
        let mut args: ParamArgs = Vec::default();

        let mut rows: Vec<String> = Vec::default();
        for d in chunk {
            let mut row: Vec<String> = Vec::default();
            for col in &columns {
                d.bind(col.name(), &mut args)?;
                row.push(next_params.next());
            }
            rows.push(format!("({})", row.join(",")));
        }
        let rows = rows.join(",");
        let sql = format!("INSERT INTO {} ({}) VALUES {}", identifier, colnames, rows);
        conn.execute(&sql, &args).await?;
    }

    Ok(())
}

//  /// Executes the query in the database Bulk Inserting the values
//  /// This method of bulk inserting is faster, but is not available for all data structures.
//  /// This method is preferred if available
//  pub(crate) async fn run_fast<T, C>(conn: &C, data: &[T]) -> Result<()>
//  where
//      C: Client,
//      T: WriteToArgs + HasSchema,
//      <T as HasSchema>::Schema: TableInfo + TableColumns,
//      //'c: 'r,
//      //DB: Database,
//      //T: WriteBulkArrayToArgs<DB> + HasSchema,
//      //<T as HasSchema>::Schema: TableInfo + TableColumns<DB>,
//      //<DB as HasArguments<'r>>::Arguments: IntoArguments<'args, DB>,
//      //C: Connection<DB>,
//  {
//      if data.is_empty() {
//          return Ok(());
//      }
//      let syntax = conn.syntax();
//
//      let col_writer = ColumnWriter::new(syntax);
//
//      let all_columns = <<T as HasSchema>::Schema as TableColumns>::columns();
//      let pks = <<T as HasSchema>::Schema as TableColumns>::primary_keys();
//      let columns: Vec<_> = all_columns.iter().filter(|c| !pks.contains(c)).collect();
//
//      let identifier = <<T as HasSchema>::Schema>::identifier().join(".");
//      let next_params = NextParam::new(syntax);
//
//      //let mut args: <DB as HasArguments>::Arguments = Default::default();
//      let mut args: ParamArgs = Vec::default();
//
//      let colnames: Vec<String> = columns
//          .iter()
//          .map(|c| col_writer.excape(c.name()))
//          .collect();
//      let colnames = colnames.join(", ");
//
//      let mut nest_parts: Vec<String> = Vec::default();
//      let data: Vec<&T> = data.iter().collect();
//      for column in columns {
//          T::bind(&data, column, &mut args)?;
//          nest_parts.push(format!("{}::{}[]", next_params.next(), column.dbtype()));
//      }
//
//      //format!("INSERT INTO {} ({}) (select * from unnest($1::int[], $2::int[]))"
//      let nest = nest_parts.join(", ");
//      let sql = format!(
//          "INSERT INTO {} ({}) (select * from unnest({}))",
//          identifier, colnames, nest
//      );
//
//      // lifetime hacks - Remove if you can
//      // We know the use of sql and conn do not exceed the underlying call to fetch
//      // sqlx if wants to hold the borrow for much longer than what is needed.
//      // This hack prevents the borrow from exceeding the life of this call
//      let sql_len = sql.len();
//      let sqlp = sql.as_ptr();
//      let sql_hack: &[u8] = unsafe { std::slice::from_raw_parts(sqlp, sql_len) };
//      let sql: &str = std::str::from_utf8(sql_hack).unwrap();
//      conn.execute(sql, args).await?;
//
//      Ok(())
//  }
