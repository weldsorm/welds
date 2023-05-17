use super::SelectBuilder;
use super::SelectColumn;
use crate::alias::TableAlias;
use crate::query::clause::ClauseAdder;
use crate::query::clause::NextParam;
use crate::table::{HasSchema, TableInfo};
use crate::writers::column::{ColumnWriter, DbColumnWriter};
use crate::writers::DbLimitSkipWriter;
use sqlx::database::HasArguments;
use sqlx::IntoArguments;
use std::rc::Rc;

pub(crate) struct JoinBuilder<'schema, DB: sqlx::Database> {
    pub(crate) alias_asigner: Rc<TableAlias>,
    pub(crate) outer_key: String,
    pub(crate) inner_alias: String,
    pub(crate) inner_table: String,
    pub(crate) inner_key: String,
    pub(crate) wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    pub(crate) selects: Vec<SelectColumn>,
    pub(crate) ty: Join,
    pub(crate) subs: Vec<JoinBuilder<'schema, DB>>,
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

impl<'schema, DB: sqlx::Database> JoinBuilder<'schema, DB> {
    pub(crate) fn set_aliases(&mut self, alias_asigner: &Rc<TableAlias>) {
        self.alias_asigner = alias_asigner.clone();
        self.inner_alias = self.alias_asigner.next();
    }

    pub(super) fn append_columns(&self, list: &mut Vec<String>)
    where
        DB: DbColumnWriter,
    {
        let writer = ColumnWriter::new::<DB>();
        let alias = &self.inner_alias;
        // Add these columns
        for col in &self.selects {
            let colname = writer.excape(&col.col_name);
            let fieldname = writer.excape(&col.field_name);
            if colname == fieldname {
                let col = format!("{}.{}", alias, colname);
                list.push(col);
            } else {
                let col = format!("{}.{} as {}", alias, colname, fieldname);
                list.push(col);
            }
        }
        for sub in &self.subs {
            sub.append_columns(list);
        }
    }

    pub(super) fn append_jointable(&self, list: &mut Vec<String>, outer_alias: &str)
    where
        DB: DbColumnWriter,
    {
        let writer = ColumnWriter::new::<DB>();
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
            sub.append_jointable(list, &self.inner_alias);
        }
    }

    pub(super) fn append_where<'args>(
        &self,
        list: &mut Vec<String>,
        next_params: &NextParam,
        args: &mut Option<<DB as HasArguments<'schema>>::Arguments>,
    ) where
        DB: sqlx::Database + DbLimitSkipWriter + DbColumnWriter,
        <DB as HasArguments<'schema>>::Arguments: IntoArguments<'args, DB>,
    {
        for clause in &self.wheres {
            if let Some(args) = args {
                clause.bind(args);
            }
            if let Some(p) = clause.clause(&self.inner_alias, next_params) {
                list.push(p);
            }
        }
        for sub in &self.subs {
            sub.append_where(list, next_params, args);
        }
    }

    pub(super) fn new<T>(
        sb: SelectBuilder<'schema, T, DB>,
        outer_key: String,
        inner_key: String,
    ) -> JoinBuilder<'schema, DB>
    where
        DB: sqlx::Database,
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
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
