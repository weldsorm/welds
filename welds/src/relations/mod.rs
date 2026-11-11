mod belongsto;
pub use belongsto::BelongsTo;
use std::sync::Arc;
mod hasmany;
pub use hasmany::HasMany;
mod hasone;
pub use hasone::HasOne;
mod manual;
use crate::Syntax;
use crate::query::clause::ParamArgs;
pub use manual::ManualRelationship;
mod invert;
pub(crate) use invert::RelationshipKeyPairsInvert;

pub(crate) struct RelationshipWriteContext<'t> {
    pub syntax: Syntax,
    pub table_a_alias: &'t str,
    pub table_b_alias: &'t str,
}

/// A `Relationship` is a trait used to compile time guard how table are joined together
/// this is used for joins, existin, etc
pub trait Relationship<A, B>: RelationshipWriter + Clone + 'static + PartialEq {
    fn as_writer(self) -> Arc<Box<dyn RelationshipWriter>> {
        Arc::new(Box::new(self))
    }
}

/// How a relationship is write into sql and params are written to sql.
pub trait RelationshipWriter: Send + Sync {
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, _args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
    }

    /// Returns the SQL snipit for this relationship
    /// this is specifically the clause part of the JOIN.
    fn to_sql(&self, context: RelationshipWriteContext) -> Option<String>;
}

/// A relationship that can be used for exist and in where filtering
pub trait RelationshipKeyPairs: Send + Sync {
    // they key column for the A side of the relationship
    fn a_key_column(&self) -> String;
    // they key column for the B side of the relationship
    fn b_key_column(&self) -> String;
}

/// Used to check if a relationship holds between two models.
pub trait RelationshipCompare<T, R>
where
    Self: Relationship<T, R>,
{
    // return true if the source object is linked to the other object via a the given relationship
    fn is_related(&self, source: &T, other: &R) -> bool;
}

pub trait HasRelations {
    type Relation: Default;
}

pub trait RelationAdder {}

/// Used on many-to-many join tables to lookup the fk_column
/// Auto-implemented if your model includes
pub trait HasJoinTableForeignkey<T> {
    fn fk_column() -> &'static str;
}
