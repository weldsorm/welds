use super::SelectBuilder;
use super::SelectColumn;
use super::select_column::SelectRender;
use crate::Syntax;
use crate::model_traits::{HasSchema, TableInfo};
use crate::query::clause::ClauseAdder;
use crate::query::clause::ParamArgs;
use crate::relations::RelationshipWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;
use crate::writers::alias::TableAlias;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct JoinBuilder {
    pub(crate) alias_asigner: Arc<TableAlias>,
    // how the outer table is linked to the joined (inner) table
    pub(crate) ship: RelationshipWriter,
    pub(crate) inner_alias: String,
    pub(crate) inner_table: &'static [&'static str],
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

    pub(super) fn append_select_renders(&self, list: &mut Vec<SelectRender>) {
        let alias = &self.inner_alias;
        // Add these columns
        for select in &self.selects {
            list.push(SelectRender::new(select, alias));
        }
        for sub in &self.subs {
            sub.append_select_renders(list);
        }
    }

    pub(super) fn append_jointable<'s, 'args, 'p>(
        &'s self,
        syntax: Syntax,
        list: &mut Vec<String>,
        outer_alias: &str,
        next_params: &NextParam,
        args: &'args mut Option<ParamArgs<'p>>,
    ) where
        's: 'p,
    {
        let inner_table = TableWriter::new(syntax).write2(self.inner_table);

        // bind in the same order the clauses are written
        if let Some(args) = args {
            self.ship.bind(args);
        }
        let on_parts = self
            .ship
            .to_sql(syntax, outer_alias, &self.inner_alias, next_params);

        let mut sql = format!(
            "{jointy} {itn} {ita}",
            jointy = self.ty.to_sql(),
            itn = inner_table,
            ita = self.inner_alias,
        );
        // A relationship without any clauses has nothing to join ON
        if !on_parts.is_empty() {
            sql.push_str(" ON ");
            sql.push_str(&on_parts.join(" AND "));
        }
        list.push(sql);
        for sub in &self.subs {
            sub.append_jointable(syntax, list, &self.inner_alias, next_params, args);
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

    pub(super) fn new<T>(sb: SelectBuilder<T>, ship: RelationshipWriter) -> JoinBuilder
    where
        T: Send + HasSchema,
        <T as HasSchema>::Schema: TableInfo,
    {
        let tn = <T as HasSchema>::Schema::identifier();

        JoinBuilder {
            alias_asigner: sb.qb.alias_asigner.clone(),
            inner_alias: sb.qb.alias.clone(),
            inner_table: tn,
            ship,
            wheres: sb.qb.wheres,
            selects: sb.selects,
            ty: Join::Inner,
            subs: sb.joins,
        }
    }
}
