use crate::Client;
use crate::errors::{Result, WeldsError};
use crate::model_traits::{HasSchema, TableColumns, TableInfo, WriteToArgs};
use crate::query::clause::ParamArgs;
use crate::relations::HasJoinTableForeignkey;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;
use std::any::type_name;

/// Create a many-to-many join table link.
/// Creates an instance of a <Link> model in the data that joins model_a and model_b
/// Success if link already exists
/// Fails only if there is a database issue.
///
/// Warning: This function DOES NOT check that model_a or model_b is in the database.
/// This is the responsibility of the calling code.
#[maybe_async::maybe_async]
pub async fn create<Link, A, B>(conn: &dyn Client, model_a: &A, model_b: &B) -> Result<()>
where
    Link: WriteToArgs + HasSchema,
    <Link as HasSchema>::Schema: TableInfo + TableColumns,
    Link: HasJoinTableForeignkey<A> + HasJoinTableForeignkey<B>,
    A: WriteToArgs + HasSchema,
    <A as HasSchema>::Schema: TableInfo + TableColumns,
    B: WriteToArgs + HasSchema,
    <B as HasSchema>::Schema: TableInfo + TableColumns,
{
    let syntax = conn.syntax();
    let tablename_parts = <<Link as HasSchema>::Schema>::identifier();
    let tablename: String = TableWriter::new(syntax).write2(tablename_parts);

    // Make sure each model has exactly one PK
    let a_pks = <A as HasSchema>::Schema::primary_keys();
    if a_pks.len() != 1 {
        let key_error = format!(
            "Error: the struct \"{}\" can't be used in a join table. It must have exactly one PK.",
            type_name::<A>()
        );
        return Err(WeldsError::InsertFailed(key_error));
    }
    let a_pk = &a_pks[0];
    let b_pks = <B as HasSchema>::Schema::primary_keys();
    if b_pks.len() != 1 {
        let key_error = format!(
            "Error: the struct \"{}\" can't be used in a join table. It must have exactly one PK.",
            type_name::<B>()
        );
        return Err(WeldsError::InsertFailed(key_error));
    }
    let b_pk = &b_pks[0];

    // Add the PKs to the args to send the the DB
    let mut args: ParamArgs = Vec::with_capacity(2);
    model_a.bind(a_pk.name(), &mut args)?;
    model_b.bind(b_pk.name(), &mut args)?;
    model_a.bind(a_pk.name(), &mut args)?;
    model_b.bind(b_pk.name(), &mut args)?;

    let a_fk = <Link as HasJoinTableForeignkey<A>>::fk_column();
    let b_fk = <Link as HasJoinTableForeignkey<B>>::fk_column();
    let col_write = ColumnWriter::new(syntax);
    let a_fk = col_write.excape(a_fk);
    let b_fk = col_write.excape(b_fk);
    let next_param = NextParam::new(syntax);

    let sql = format!(
        "INSERT INTO {tablename} ({a_fk}, {b_fk}) SELECT {}, {} WHERE NOT EXISTS ( SELECT 1 FROM {tablename} WHERE {a_fk} = {} AND {b_fk} = {} )",
        next_param.next(),
        next_param.next(),
        next_param.next(),
        next_param.next()
    );

    conn.execute(&sql, &args).await?;

    Ok(())
}

/// Removes a many-to-many join table link.
/// deletes an instance of a <Link> model in the database linking model_a to model_b
/// Success if link was not in database
/// Fails only if there is a database issue.
#[maybe_async::maybe_async]
pub async fn delete<Link, A, B>(conn: &dyn Client, model_a: &A, model_b: &B) -> Result<()>
where
    Link: WriteToArgs + HasSchema,
    <Link as HasSchema>::Schema: TableInfo + TableColumns,
    Link: HasJoinTableForeignkey<A> + HasJoinTableForeignkey<B>,
    A: WriteToArgs + HasSchema,
    <A as HasSchema>::Schema: TableInfo + TableColumns,
    B: WriteToArgs + HasSchema,
    <B as HasSchema>::Schema: TableInfo + TableColumns,
{
    let syntax = conn.syntax();
    let tablename_parts = <<Link as HasSchema>::Schema>::identifier();
    let tablename: String = TableWriter::new(syntax).write2(tablename_parts);

    // Make sure each model has exactly one PK
    let a_pks = <A as HasSchema>::Schema::primary_keys();
    if a_pks.len() != 1 {
        let key_error = format!(
            "Error: the struct \"{}\" can't be used in a join table. It must have exactly one PK.",
            type_name::<A>()
        );
        return Err(WeldsError::InsertFailed(key_error));
    }
    let a_pk = &a_pks[0];
    let b_pks = <B as HasSchema>::Schema::primary_keys();
    if b_pks.len() != 1 {
        let key_error = format!(
            "Error: the struct \"{}\" can't be used in a join table. It must have exactly one PK.",
            type_name::<B>()
        );
        return Err(WeldsError::InsertFailed(key_error));
    }
    let b_pk = &b_pks[0];

    // Add the PKs to the args to send the the DB
    let mut args: ParamArgs = Vec::with_capacity(2);
    model_a.bind(a_pk.name(), &mut args)?;
    model_b.bind(b_pk.name(), &mut args)?;

    let a_fk = <Link as HasJoinTableForeignkey<A>>::fk_column();
    let b_fk = <Link as HasJoinTableForeignkey<B>>::fk_column();
    let col_write = ColumnWriter::new(syntax);
    let a_fk = col_write.excape(a_fk);
    let b_fk = col_write.excape(b_fk);
    let next_param = NextParam::new(syntax);

    let sql = format!(
        "DELETE FROM {tablename} WHERE {a_fk} = {} AND {b_fk} = {}",
        next_param.next(),
        next_param.next(),
    );

    conn.execute(&sql, &args).await?;

    Ok(())
}
