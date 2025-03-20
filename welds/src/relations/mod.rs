use crate::model_traits::UniqueIdentifier;

mod belongsto;
pub use belongsto::BelongsTo;

mod hasmany;
pub use hasmany::HasMany;

mod hasone;
pub use hasone::HasOne;

mod belongstoone;
pub use belongstoone::BelongsToOne;

/// Describes how a relationship should be wired up.
/// Gives info about what DB columns to use on both Models
pub trait Relationship<R>: Clone + PartialEq + Send {
    fn their_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;

    fn my_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;
}

/// Used to check if a relationship holds between two models.
pub trait RelationshipCompare<T, R>
where
    Self: Relationship<R>,
{
    // return true if the source object is linked to the other object via a the given relationship
    fn is_related(&self, source: &T, other: &R) -> bool;
}

pub trait HasRelations {
    type Relation: Default;
}

pub trait RelationAdder {}
