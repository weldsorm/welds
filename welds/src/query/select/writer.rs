use crate::model_traits::Column;
use crate::model_traits::TableIdent;
use crate::query::clause::exists::ExistIn;
use crate::query::clause::ClauseAdder;
use crate::query::clause::OrderBy;
use crate::query::clause::ParamArgs;
use crate::query::helpers::{build_where, join_sql_parts};
use crate::query::tail;
use crate::writers::alias::TableAlias;
use crate::writers::ColumnWriter;
use crate::writers::CountWriter;
use crate::writers::NextParam;
use crate::Syntax;
use std::sync::Arc;

/// take all info from a query and translates it into SQL
/// can build the params Vec as needed
pub struct SelectWriter {
    syntax: Syntax,
    table_ident: TableIdent,
    tablealias: String,
}

impl SelectWriter {
    /// creates a new writer build building SQL select statements
    pub fn new(syntax: Syntax, table_ident: &TableIdent) -> Self {
        let ta = TableAlias::new();
        Self {
            syntax,
            table_ident: table_ident.clone(),
            tablealias: ta.next(),
        }
    }

    /// creates a new writer build building SQL select statements
    pub fn new_with_alias(
        syntax: Syntax,
        table_ident: &TableIdent,
        alias: impl Into<String>,
    ) -> Self {
        Self {
            syntax,
            table_ident: table_ident.clone(),
            tablealias: alias.into(),
        }
    }

    /// Write a `Select count` SQL String from its parts
    /// Will fill in the args to be sent to the database if provided
    pub fn sql_count<'lam, 'exist, 'args, 'p>(
        &self,
        wheres: &'lam [Arc<Box<dyn ClauseAdder>>],
        exist_ins: &'exist [ExistIn],
        limit: &Option<i64>,
        offset: &Option<i64>,
        orders: &[OrderBy],
        args: &'args mut Option<ParamArgs<'p>>,
    ) -> String
    where
        'lam: 'p,
        'exist: 'p,
    {
        let next_params = NextParam::new(self.syntax);
        join_sql_parts(&[
            build_head_count(&self.table_ident, &self.tablealias, self.syntax),
            build_where(
                self.syntax,
                &next_params,
                &self.tablealias,
                wheres,
                args,
                exist_ins,
            ),
            tail::write(self.syntax, limit, offset, orders, &self.tablealias),
        ])
    }

    /// Write a `Select ... FROM ...` SQL String from its parts
    /// Will fill in the args to be sent to the database if provided
    pub fn sql<'col, 'lam, 'exist, 'args, 'p>(
        &self,
        columns: &'col [Column],
        wheres: &'lam [Arc<Box<dyn ClauseAdder>>],
        exist_ins: &'exist [ExistIn],
        limit: &Option<i64>,
        offset: &Option<i64>,
        orders: &[OrderBy],
        args: &'args mut Option<ParamArgs<'p>>,
    ) -> String
    where
        'lam: 'p,
        'exist: 'p,
    {
        let next_params = NextParam::new(self.syntax);
        join_sql_parts(&[
            build_head_select(self.syntax, &self.table_ident, &self.tablealias, columns),
            build_where(
                self.syntax,
                &next_params,
                &self.tablealias,
                wheres,
                args,
                exist_ins,
            ),
            tail::write(self.syntax, limit, offset, orders, &self.tablealias),
        ])
    }
}

fn build_head_count(table: &TableIdent, tablealias: &str, syntax: Syntax) -> Option<String> {
    let tn = table.to_string();
    let identifier = format!("{} {}", tn, &tablealias);
    let cw = CountWriter::new(syntax);
    let count_star = cw.count(Some(tablealias), Some("*"));
    Some(format!("SELECT {} FROM {}", count_star, identifier))
}

fn build_head_select(
    syntax: Syntax,
    table: &TableIdent,
    tablealias: &str,
    cols_info: &[Column],
) -> Option<String> {
    let writer = ColumnWriter::new(syntax);
    let mut head: Vec<&str> = Vec::default();
    head.push("SELECT");
    //let cols_info = S::columns();
    let cols: Vec<_> = cols_info
        .iter()
        .map(|col| writer.write(tablealias, col))
        .collect();
    let cols = cols.join(", ");
    head.push(&cols);
    head.push("FROM");
    let tn = table.to_string();
    let identifier = format!("{} {}", tn, tablealias);
    head.push(&identifier);
    Some(head.join(" "))
}
