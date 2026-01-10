use super::ClauseAdder;
use crate::Syntax;
use crate::model_traits::HasSchema;
use crate::model_traits::TableColumns;
use crate::model_traits::TableInfo;
use crate::model_traits::UniqueIdentifier;
use crate::query::builder::QueryBuilder;
use crate::query::clause::ParamArgs;
use crate::query::helpers::{build_tail, build_where, join_sql_parts};
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;

/// Used to generated a SQL IN clause.
/// This is used when deleting and updating to be able to apply limit
pub struct WhereIn<'qb, T> {
    qb: &'qb QueryBuilder<T>,
}

impl<'qb, T> WhereIn<'qb, T>
where
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier,
{
    pub(crate) fn new(qb: &'qb QueryBuilder<T>) -> Self {
        WhereIn { qb }
    }

    fn outer_tablecolumn(&self, syntax: Syntax, outer_tablealias: &str) -> String {
        let column = T::Schema::id_column();
        let col_writer = ColumnWriter::new(syntax);
        format!("{}.{}", outer_tablealias, col_writer.excape(column.name()))
    }
}

impl<T> ClauseAdder for WhereIn<'_, T>
where
    T: HasSchema + Sync + Send,
    <T as HasSchema>::Schema: UniqueIdentifier + TableInfo + TableColumns,
{
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for w in &self.qb.wheres {
            w.bind(args);
        }
        for w in &self.qb.exist_ins {
            w.bind(args);
        }
    }

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        // writes => ID IN ( SELECT ID FROM ... )

        let outcol = self.outer_tablecolumn(syntax, alias);
        let inner_alias = &self.qb.alias;
        let mut args = None;
        let inner_sql = join_sql_parts(&[
            build_head_select::<<T as HasSchema>::Schema>(syntax, inner_alias),
            build_where(
                syntax,
                next_params,
                inner_alias,
                &self.qb.wheres,
                &mut args,
                &self.qb.exist_ins,
            ),
            build_tail(syntax, self.qb),
        ]);

        Some(format!(" {} IN ({}) ", outcol, inner_sql))
    }
}

fn build_head_select<S>(syntax: Syntax, tablealias: &str) -> Option<String>
where
    S: TableInfo + UniqueIdentifier,
{
    let mut tablename = TableWriter::new(syntax).write2(S::identifier());

    if tablename != tablealias {
        tablename = format!("{} {}", tablename, tablealias);
    }
    let writer = ColumnWriter::new(syntax);
    let col_raw = S::id_column();
    let col = writer.write(tablealias, &col_raw);
    Some(format!("SELECT {} FROM {}", col, tablename))
}
