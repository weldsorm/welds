mod belongsto;
pub use belongsto::BelongsTo;

mod hasmany;
pub use hasmany::HasMany;

mod hasone;
pub use hasone::HasOne;

mod manual;
pub use manual::ManualRelationship;

/// Describes how a relationship should be wired up.
/// Gives info about what DB columns to use on both Models
pub trait Relationship<SELF, R>: Clone + PartialEq + Send {
    fn their_key(&self) -> String;
    fn my_key(&self) -> String;
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
