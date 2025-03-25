use super::SelectBuilder;
use super::SelectColumn;
use crate::model_traits::{HasSchema, TableInfo};
use crate::query::clause::ClauseAdder;
use crate::query::clause::ParamArgs;
use crate::writers::alias::TableAlias;
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::Syntax;
use std::sync::Arc;

pub(crate) struct JoinBuilder {
    pub(crate) alias_asigner: Arc<TableAlias>,
    pub(crate) outer_key: String,
    pub(crate) inner_alias: String,
    pub(crate) inner_table: String,
    pub(crate) inner_key: String,
    pub(crate) wheres: Vec<Arc<Box<dyn ClauseAdder>>>,
    pub(crate) selects: Vec<SelectColumn>,
    pub(crate) ty: Join,
    pub(crate) subs: Vec<JoinBuilder>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Join {
    Inner,
    Left,
    Right,
    Cross,
}

impl Join {
    fn to_sql(self) -> &'static str {
        match self {
            Join::Inner => "JOIN",
            Join::Left => "LEFT JOIN",
            Join::Right => "RIGHT JOIN",
            Join::Cross => "CROSS JOIN",
        }
    }
}

impl JoinBuilder {
    pub(crate) fn set_aliases(&mut self, alias_asigner: &Arc<TableAlias>) {
        self.alias_asigner = alias_asigner.clone();
        self.inner_alias = self.alias_asigner.next();
    }

    pub(super) fn append_columns(&self, syntax: Syntax, list: &mut Vec<String>) {
        let alias = &self.inner_alias;
        // Add these columns
        for select in &self.selects {
            list.push(select.write(syntax, alias))
        }
        for sub in &self.subs {
            sub.append_columns(syntax, list);
        }
    }

    pub(super) fn append_jointable(
        &self,
        syntax: Syntax,
        list: &mut Vec<String>,
        outer_alias: &str,
    ) {
        let writer = ColumnWriter::new(syntax);
        let sql = format!(
            "{jointy} {itn} {ita} ON {ota}.{otk} = {ita}.{itk}",
            jointy = self.ty.to_sql(),
            itn = self.inner_table,
            ita = self.inner_alias,
            ota = outer_alias,
            otk = writer.excape(&self.outer_key),
            itk = writer.excape(&self.inner_key)
        );
        list.push(sql);
        for sub in &self.subs {
            sub.append_jointable(syntax, list, &self.inner_alias);
        }
    }

    pub(super) fn append_where<'s, 'args, 'p>(
        &'s self,
        syntax: Syntax,
        list: &mut Vec<String>,
        next_params: &NextParam,
        args: &'args mut Option<ParamArgs<'p>>,
    ) where
        's: 'p,
    {
        for clause in &self.wheres {
            if let Some(args) = args {
                clause.bind(args);
            }
            if let Some(p) = clause.clause(syntax, &self.inner_alias, next_params) {
                list.push(p);
            }
        }
        for sub in &self.subs {
            sub.append_where(syntax, list, next_params, args);
        }
    }

    pub(super) fn new<T>(sb: SelectBuilder<T>, outer_key: String, inner_key: String) -> JoinBuilder
    where
        T: Send + HasSchema,
        <T as HasSchema>::Schema: TableInfo,
    {
        let tn = <T as HasSchema>::Schema::identifier().join(".");
        JoinBuilder {
            alias_asigner: sb.qb.alias_asigner.clone(),
            inner_alias: sb.qb.alias.clone(),
            inner_table: tn,
            outer_key,
            inner_key,
            wheres: sb.qb.wheres,
            selects: sb.selects,
            ty: Join::Inner,
            subs: sb.joins,
        }
    }
}
