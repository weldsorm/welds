use super::SelectBuilder;
use super::SelectColumn;
use super::select_column::SelectRender;
use crate::Syntax;
use crate::model_traits::{HasSchema, TableInfo};
use crate::query::clause::ParamArgs;
use crate::query::clause::manualparam::ManualParam;
use crate::query::clause::{AsFieldName, ClauseAdder};
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::writers::TableWriter;
use crate::writers::alias::TableAlias;
use std::marker::PhantomData;
use std::sync::Arc;
use welds_connections::Param;

#[derive(Clone)]
pub(crate) struct JoinBuilder {
    pub(crate) alias_asigner: Arc<TableAlias>,
    pub(crate) outer_key: String,
    pub(crate) inner_alias: String,
    pub(crate) inner_table: &'static [&'static str],
    pub(crate) inner_key: String,
    /// Additional conditions added to the `ON` part of the join
    pub(crate) on_conditions: Vec<OnCondition>,
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

/// An extra condition rendered inside the `ON ( ... )` of a join.
#[derive(Clone)]
pub(crate) enum OnCondition {
    /// `outer.col = inner.col`
    ColumnEq { outer: String, inner: String },
    /// A clause rendered against the outer table's alias
    Outer(Arc<Box<dyn ClauseAdder>>),
    /// A clause rendered against the inner (joined) table's alias
    Inner(Arc<Box<dyn ClauseAdder>>),
    /// Hand written SQL. `{outer}` / `{inner}` are swapped for the table aliases, `?` for params
    Manual(Arc<ManualOn>),
}

pub(crate) struct ManualOn {
    sql: String,
    params: Vec<Box<dyn Param + Send + Sync>>,
}

impl ManualOn {
    fn render(&self, outer_alias: &str, inner_alias: &str, next_params: &NextParam) -> String {
        let sql = self
            .sql
            .replace("{outer}", outer_alias)
            .replace("{inner}", inner_alias);
        let mut out = String::with_capacity(sql.len() + 8);
        for c in sql.chars() {
            match c {
                '?' => out.push_str(&next_params.next()),
                _ => out.push(c),
            }
        }
        format!("( {} )", out)
    }
}

/// Builder for additional conditions on the `ON` part of a join.
///
/// `Outer` is the schema of the table you are joining *from*,
/// `Inner` is the schema of the table being joined in.
///
/// Every condition is `AND`ed onto the relationship's key match:
/// `ON outer.fk = inner.pk AND <condition> AND <condition> ...`
///
/// ```ignore
/// Contact::all()
///     .select(|c| c.id)
///     .left_join_on(
///         |c| c.logins,
///         Login::all().select(|l| l.created_at),
///         |on| on.eq(|c| c.app, |l| l.app),
///     );
/// // ... LEFT JOIN logins t2 ON t1."id" = t2."contact_id" AND t1."app" = t2."app"
/// ```
pub struct JoinOn<Outer, Inner> {
    pub(crate) conditions: Vec<OnCondition>,
    _p: PhantomData<fn() -> (Outer, Inner)>,
}

impl<Outer, Inner> JoinOn<Outer, Inner>
where
    Outer: Default,
    Inner: Default,
{
    pub(crate) fn new() -> Self {
        Self {
            conditions: Vec::default(),
            _p: PhantomData,
        }
    }

    /// Match a column on the outer table to a column on the joined table.
    /// Adds `AND outer.col = inner.col` to the `ON` clause.
    pub fn eq<V1, V2, F1, F2>(
        mut self,
        outer: impl Fn(Outer) -> F1,
        inner: impl Fn(Inner) -> F2,
    ) -> Self
    where
        F1: AsFieldName<V1>,
        F2: AsFieldName<V2>,
    {
        let o = outer(Default::default());
        let i = inner(Default::default());
        self.conditions.push(OnCondition::ColumnEq {
            outer: o.colname().to_string(),
            inner: i.colname().to_string(),
        });
        self
    }

    /// Add a filter on a column of the joined (inner) table to the `ON` clause.
    /// Unlike `where_col`, this does not filter out rows of a LEFT JOIN.
    ///
    /// `on.inner(|l| l.active.equal(true))` => `AND t2."active" = ?`
    pub fn inner(mut self, lam: impl Fn(Inner) -> Box<dyn ClauseAdder>) -> Self {
        let clause = lam(Default::default());
        self.conditions.push(OnCondition::Inner(Arc::new(clause)));
        self
    }

    /// Add a filter on a column of the outer table to the `ON` clause.
    ///
    /// `on.outer(|c| c.app.equal("x"))` => `AND t1."app" = ?`
    pub fn outer(mut self, lam: impl Fn(Outer) -> Box<dyn ClauseAdder>) -> Self {
        let clause = lam(Default::default());
        self.conditions.push(OnCondition::Outer(Arc::new(clause)));
        self
    }

    /// Hand write a condition for the `ON` clause.
    ///
    /// NOTE: use `{outer}` and `{inner}` for the table aliases, they are swapped out at runtime.
    ///
    /// NOTE: use '?' for params. They will be swapped out for the correct Syntax
    ///
    /// `on.manual("{outer}.app = {inner}.app OR {inner}.app = ?", ("global",))`
    pub fn manual(mut self, sql: &'static str, params: impl Into<ManualParam>) -> Self {
        let params: ManualParam = params.into();
        self.conditions.push(OnCondition::Manual(Arc::new(ManualOn {
            sql: sql.to_string(),
            params: params.into_inner(),
        })));
        self
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

    /// Writes the JOIN sql for this join (and all sub joins).
    /// Any params used in the ON clause are bound in the order they are written
    /// so this must be called before the WHERE clause is built.
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
        let writer = ColumnWriter::new(syntax);
        let inner_alias = self.inner_alias.as_str();

        let inner_table = TableWriter::new(syntax).write2(self.inner_table);

        let mut on_parts: Vec<String> = vec![format!(
            "{ota}.{otk} = {ita}.{itk}",
            ita = inner_alias,
            ota = outer_alias,
            otk = writer.excape(&self.outer_key),
            itk = writer.excape(&self.inner_key)
        )];

        for cond in &self.on_conditions {
            let part = match cond {
                OnCondition::ColumnEq { outer, inner } => Some(format!(
                    "{}.{} = {}.{}",
                    outer_alias,
                    writer.excape(outer),
                    inner_alias,
                    writer.excape(inner)
                )),
                OnCondition::Outer(clause) => {
                    if let Some(args) = args {
                        clause.bind(args);
                    }
                    clause.clause(syntax, outer_alias, next_params)
                }
                OnCondition::Inner(clause) => {
                    if let Some(args) = args {
                        clause.bind(args);
                    }
                    clause.clause(syntax, inner_alias, next_params)
                }
                OnCondition::Manual(manual) => {
                    if let Some(args) = args {
                        for p in &manual.params {
                            args.push(p.as_ref());
                        }
                    }
                    Some(manual.render(outer_alias, inner_alias, next_params))
                }
            };
            if let Some(part) = part {
                on_parts.push(part);
            }
        }

        let sql = format!(
            "{jointy} {itn} {ita} ON {on}",
            jointy = self.ty.to_sql(),
            itn = inner_table,
            ita = inner_alias,
            on = on_parts.join(" AND "),
        );
        list.push(sql);
        for sub in &self.subs {
            sub.append_jointable(syntax, list, inner_alias, next_params, args);
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
        let tn = <T as HasSchema>::Schema::identifier();

        JoinBuilder {
            alias_asigner: sb.qb.alias_asigner.clone(),
            inner_alias: sb.qb.alias.clone(),
            inner_table: tn,
            outer_key,
            inner_key,
            on_conditions: Vec::default(),
            wheres: sb.qb.wheres,
            selects: sb.selects,
            ty: Join::Inner,
            subs: sb.joins,
        }
    }
}
