use super::ClauseAdder;

use crate::query::builder::QueryBuilder;
use crate::query::helpers::{build_tail, build_where, join_sql_parts};
use crate::table::HasSchema;
use crate::table::TableColumns;
use crate::table::TableInfo;
use crate::table::UniqueIdentifier;
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::limit_skip::DbLimitSkipWriter;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;

/// Used to generated a SQL IN clause.
/// This is used when deleting and updating to be able to apply limit

pub struct WhereIn<'qb, 'schema, T, DB: sqlx::Database> {
    qb: &'qb QueryBuilder<'schema, T, DB>,
}

impl<'qb, 'schema, DB, T> WhereIn<'qb, 'schema, T, DB>
where
    DB: sqlx::Database + DbLimitSkipWriter + DbColumnWriter,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier<DB>,
{
    pub(crate) fn new(qb: &'qb QueryBuilder<'schema, T, DB>) -> Self {
        WhereIn { qb }
    }

    fn outer_tablecolumn(&self, outer_tablealias: &str) -> String {
        let column = T::Schema::id_column();
        format!("{}.{}", outer_tablealias, column.name())
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

    fn clause(&self, alias: &str, next_params: &super::NextParam) -> Option<String> {
        // writes => ID IN ( SELECT ID FROM ... )

        let outcol = self.outer_tablecolumn(alias);
        let inner_alias = &self.qb.alias;
        let mut args = None;
        let inner_sql = join_sql_parts(&[
            build_head_select::<DB, <T as HasSchema>::Schema>(inner_alias),
            build_where(
                next_params,
                inner_alias,
                &mut args,
                &self.qb.wheres,
                &self.qb.exist_ins,
            ),
            build_tail(self.qb),
        ]);

        Some(format!(" {} IN ({}) ", outcol, inner_sql))
    }
}

fn build_head_select<DB, S>(tablealias: &str) -> Option<String>
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
    let col = writer.write(tablealias, &col_raw);
    Some(format!("SELECT {} FROM {}", col, tablename))
}
