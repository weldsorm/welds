use crate::Syntax;
use crate::query::clause::ParamArgs;
use crate::writers::{ColumnWriter, NextParam};
use std::sync::Arc;
use welds_connections::Param;

/// A single part of the SQL used to link two tables together.
///
/// A relationship is made up of one or more clauses that are ANDed together.
/// When written as SQL, a clause is given the alias of "my" table (the model
/// the relationship is defined on) and "their" table (the related model).
///
/// These are used to write the `ON` of a JOIN and the correlated part
/// of the sub-queries used by `where_relation`, `map_query` and `include`.
pub trait RelationshipClause: Send + Sync {
    /// Returns the SQL snippet for this part of the relationship.
    fn write_sql(&self, ctx: &ClauseContext) -> String;

    /// Add any arguments used by this clause to the list of arguments to send to the database
    /// MUST push exactly one arg for every call to `next_params.next()` in `write_sql`
    fn bind<'lam, 'args, 'p>(&'lam self, _args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
    }

    /// If this clause is a simple column to column equality, returns it.
    /// Used when a relationship needs to be written as `col IN (SELECT col ...)`
    fn as_key_pair(&self) -> Option<&KeyPair> {
        None
    }
}

/// Everything a `RelationshipClause` needs to know to write itself as SQL
pub struct ClauseContext<'a> {
    pub syntax: Syntax,
    /// The alias of the table the relationship is defined on
    pub my_alias: &'a str,
    /// The alias of the related table
    pub their_alias: &'a str,
    /// Used to get the next param placeholder ("?", "$1", "@p1") for the database
    pub next_params: &'a NextParam,
    /// The table that should be written on the left side of a comparison.
    /// Purely cosmetic, keeps the SQL reading outer/inner in a consistent order
    pub mine_on_left: bool,
}

/// Links a column on my table to a column on their table.
/// `my_table.my_key = their_table.their_key`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    pub their_key: String,
    pub my_key: String,
}

impl KeyPair {
    pub fn new(my_key: impl Into<String>, their_key: impl Into<String>) -> Self {
        Self {
            my_key: my_key.into(),
            their_key: their_key.into(),
        }
    }
}

impl RelationshipClause for KeyPair {
    fn write_sql(&self, ctx: &ClauseContext) -> String {
        let writer = ColumnWriter::new(ctx.syntax);
        let mine = format!("{}.{}", ctx.my_alias, writer.excape(&self.my_key));
        let theirs = format!("{}.{}", ctx.their_alias, writer.excape(&self.their_key));
        if ctx.mine_on_left {
            format!("{} = {}", mine, theirs)
        } else {
            format!("{} = {}", theirs, mine)
        }
    }

    fn as_key_pair(&self) -> Option<&KeyPair> {
        Some(self)
    }
}

/// Limits a relationship to rows where a column on their table equals a value.
/// `their_table.key = ?`
/// The value is sent to the database as a param.
#[derive(Clone)]
pub struct TheirKeyEq {
    pub key: String,
    pub value: Arc<dyn Param + Send + Sync>,
}

impl TheirKeyEq {
    pub fn new<P>(key: impl Into<String>, value: P) -> Self
    where
        P: 'static + Param + Send + Sync,
    {
        Self {
            key: key.into(),
            value: Arc::new(value),
        }
    }
}

/// Limits a relationship to rows where a column on my table equals a value.
/// `my_table.key = ?`
/// The value is sent to the database as a param.
#[derive(Clone)]
pub struct MyKeyEq {
    pub key: String,
    pub value: Arc<dyn Param + Send + Sync>,
}

impl MyKeyEq {
    pub fn new<P>(key: impl Into<String>, value: P) -> Self
    where
        P: 'static + Param + Send + Sync,
    {
        Self {
            key: key.into(),
            value: Arc::new(value),
        }
    }
}

fn write_value_eq(syntax: Syntax, alias: &str, key: &str, next_params: &NextParam) -> String {
    let writer = ColumnWriter::new(syntax);
    format!("{}.{} = {}", alias, writer.excape(key), next_params.next())
}

impl RelationshipClause for TheirKeyEq {
    fn write_sql(&self, ctx: &ClauseContext) -> String {
        write_value_eq(ctx.syntax, ctx.their_alias, &self.key, ctx.next_params)
    }

    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        args.push(self.value.as_ref());
    }
}

impl RelationshipClause for MyKeyEq {
    fn write_sql(&self, ctx: &ClauseContext) -> String {
        write_value_eq(ctx.syntax, ctx.my_alias, &self.key, ctx.next_params)
    }

    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        args.push(self.value.as_ref());
    }
}

// Params can't be compared by value, Two value clauses are considered
// equal if they point at the same column and share the same value instance (clones)
impl PartialEq for TheirKeyEq {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && std::ptr::addr_eq(Arc::as_ptr(&self.value), Arc::as_ptr(&other.value))
    }
}

impl PartialEq for MyKeyEq {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && std::ptr::addr_eq(Arc::as_ptr(&self.value), Arc::as_ptr(&other.value))
    }
}
