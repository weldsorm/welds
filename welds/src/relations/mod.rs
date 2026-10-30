mod belongsto;
pub use belongsto::BelongsTo;

mod hasmany;
pub use hasmany::HasMany;

mod hasone;
pub use hasone::HasOne;

mod manual;
pub use manual::ManualRelationship;

pub struct KeyPair {
    pub their_key: String,
    pub my_key: String,
}

pub struct TheirKeyEq {
    pub key: String,
    pub value: &'static str, // use sql params
}

pub struct MyKeyEq {
    pub key: String,
    pub value: &'static str, // use sql params
}

trait RelationshipClause {}
impl RelationshipClause for KeyPair {}
impl RelationshipClause for TheirKeyEq {}
impl RelationshipClause for MyKeyEq {}

/// Describes how a relationship should be wired up.
/// Gives info about what DB columns to use on both Models
pub trait Relationship<SELF, R>: Clone + PartialEq + Send {
    /// a collection of the parts of the 'on' clause
    /// all the parts that are requires to join table `SELF` to table `R`
    fn clauses(&self) -> Vec<Box<dyn RelationshipClause>>;
}

pub trait RelationshipWriter {
    fn to_sql(table_a: &str, table_b: &str) -> String;
}

impl<T, S, R> RelationshipWriter for T
where
    T: Relationship<S, R>,
{
    fn to_sql(table_a: &str, table_b: &str) -> String {
        todo!()
    }
}

/// Used to check if a relationship holds between two models.
/// This is post SQL logic. It is used to get data between sets of data
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
