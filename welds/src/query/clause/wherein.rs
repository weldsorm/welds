use super::exists::ExistIn;
use super::ClauseAdder;
use crate::query::clause::OrderBy;
use crate::query::helpers::{build_tail, build_where, join_sql_parts};
use crate::table::HasSchema;
use crate::table::TableInfo;
use crate::table::UniqueIdentifier;
use crate::table::{Column, TableColumns};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::limit_skip::DbLimitSkipWriter;
use crate::{alias::TableAlias, query::builder::QueryBuilder};
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

/// Used to generated a SQL IN clause.
/// This is used when deleting and updating to be able to apply limit

pub struct WhereIn<'qb, 'schema, T, DB: sqlx::Database> {
    outer_tablealias: Option<String>,
    qb: &'qb QueryBuilder<'schema, T, DB>,
}

impl<'qb, 'schema, DB, T> WhereIn<'qb, 'schema, T, DB>
where
    DB: sqlx::Database + DbLimitSkipWriter + DbColumnWriter,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier<DB>,
{
    pub(crate) fn new(
        qb: &'qb QueryBuilder<'schema, T, DB>,
        outer_tablealias: Option<String>,
    ) -> Self {
        WhereIn {
            outer_tablealias,
            qb,
        }
    }

    fn outer_tablecolumn(&self) -> String {
        let column = T::Schema::id_column();
        match &self.outer_tablealias {
            Some(alias) => format!("{}.{}", alias, column.name()),
            None => {
                let tableparts = T::Schema::identifier();
                let tn = tableparts.join(".");
                format!("{}.{}", tn, column.name())
            }
        }
    }
}

impl<'qb, 'cd, 'args, 'schema, DB, T> ClauseAdder<'cd, DB> for WhereIn<'qb, 'schema, T, DB>
where
    'schema: 'args,
    'cd: 'schema,
    'schema: 'cd,
    <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
    DB: sqlx::Database + DbLimitSkipWriter + DbColumnWriter,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier<DB> + TableInfo + TableColumns<DB>,
{
    fn bind(&self, args: &mut <DB as sqlx::database::HasArguments<'cd>>::Arguments) {
        for w in &self.qb.wheres {
            w.bind(args);
        }
        for w in &self.qb.exist_ins {
            w.bind(args);
        }
    }

    fn clause(&self, alias: &TableAlias, next_params: &super::NextParam) -> Option<String> {
        // writes => ID IN ( SELECT ID FROM ... )

        let outcol = self.outer_tablecolumn();
        alias.next();
        let self_tablealias = alias.peek();
        let mut args = None;
        let inner_sql = join_sql_parts(&[
            build_head_select::<DB, <T as HasSchema>::Schema>(self_tablealias),
            build_where(
                next_params,
                alias,
                &mut args,
                &self.qb.wheres,
                &self.qb.exist_ins,
            ),
            build_tail(self.qb),
        ]);

        Some(format!(" {} IN ({}) ", outcol, inner_sql))
    }
}

fn build_head_select<DB, S>(tablealias: String) -> Option<String>
where
    DB: sqlx::Database + DbColumnWriter,
    S: TableInfo + UniqueIdentifier<DB>,
{
    let mut tablename = S::identifier().join(".");
    if tablename != tablealias {
        tablename = format!("{} {}", tablename, tablealias);
    }
    let writer = ColumnWriter::new::<DB>();
    let col_raw = S::id_column();
    let col = writer.write(&tablealias, &col_raw);
    Some(format!("SELECT {} FROM {}", col, tablename))
}
