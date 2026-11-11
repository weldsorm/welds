use super::SelectBuilder;
use super::SelectColumn;
use super::select_column::SelectRender;
use crate::Syntax;
use crate::model_traits::{HasSchema, TableInfo};
use crate::query::clause::ClauseAdder;
use crate::query::clause::ParamArgs;
use crate::relations::Relationship;
use crate::relations::RelationshipWriteContext;
use crate::relations::RelationshipWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;
use crate::writers::alias::TableAlias;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct JoinBuilder {
    pub(crate) alias_asigner: Arc<TableAlias>,

    relationship_writer: Arc<Box<dyn RelationshipWriter>>,
    joining_table: &'static [&'static str],
    pub(crate) inner_alias: String,

    // pub(crate) outer_key: String,
    // pub(crate) inner_key: String,
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

    pub(super) fn append_jointable(
        &self,
        syntax: Syntax,
        list: &mut Vec<String>,
        outer_alias: &str,
    ) {
        let inner_table = TableWriter::new(syntax).write2(self.joining_table);

        let context = RelationshipWriteContext {
            syntax,
            table_a_alias: outer_alias,
            table_b_alias: &self.inner_alias,
        };

        let sql = format!(
            "{jointy} {itn} {ita} ON {join_clause}",
            jointy = self.ty.to_sql(),
            itn = inner_table,
            ita = self.inner_alias,
            join_clause = self
                .relationship_writer
                .to_sql(context)
                .unwrap_or_else(|| "1=0".to_owned() /* always false*/),
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

    pub(super) fn new<A, B, Ship>(sb: SelectBuilder<B>, relationship: Ship) -> JoinBuilder
    where
        B: Send + HasSchema,
        Ship: Relationship<A, B> + 'static,
        <B as HasSchema>::Schema: TableInfo,
    {
        let joining_table = <B as HasSchema>::Schema::identifier();

        JoinBuilder {
            alias_asigner: sb.qb.alias_asigner.clone(),
            inner_alias: sb.qb.alias.clone(),
            relationship_writer: relationship.as_writer(),
            joining_table,
            wheres: sb.qb.wheres,
            selects: sb.selects,
            ty: Join::Inner,
            subs: sb.joins,
        }
    }
}
