use crate::relations::RelationshipKeyPairs;
use crate::writers::ColumnWriter;

use super::Relationship;
use std::marker::PhantomData;

pub struct ManualRelationship<T, R> {
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    // These database column on self to use for this relationship
    self_key: &'static str,
    // The database column on the other table to use for this relationship
    other_key: &'static str,
}

impl<T, R> ManualRelationship<T, R> {
    pub fn using(
        self_col_key: &'static str,
        other_col_key: &'static str,
    ) -> ManualRelationship<T, R> {
        ManualRelationship {
            _t: Default::default(),
            _r: Default::default(),
            self_key: self_col_key,
            other_key: other_col_key,
        }
    }
}

impl<T, R> PartialEq for ManualRelationship<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.self_key == other.self_key && self.other_key == other.other_key
    }
}
impl<T, R> Clone for ManualRelationship<T, R> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            _r: Default::default(),
            self_key: self.self_key,
            other_key: self.other_key,
        }
    }
}

use super::RelationshipWriteContext;
use super::RelationshipWriter;

impl<T, R> Relationship<T, R> for ManualRelationship<T, R>
where
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
}

impl<T, R> RelationshipWriter for ManualRelationship<T, R>
where
    T: Send + Sync,
    R: Send + Sync,
{
    /// Returns the SQL snipit for this relationship
    fn to_sql(&self, context: RelationshipWriteContext) -> Option<String> {
        let table_a_key = RelationshipKeyPairs::a_key_column(self);
        let table_b_key = RelationshipKeyPairs::b_key_column(self);
        let RelationshipWriteContext {
            table_a_alias,
            table_b_alias,
            syntax,
        } = context;
        let col_writer = ColumnWriter::new(syntax);
        let table_a_key = col_writer.excape(&table_a_key);
        let table_b_key = col_writer.excape(&table_b_key);
        Some(format!(
            "{table_a_alias}.{table_a_key} = {table_b_alias}.{table_b_key}"
        ))
    }
}

// impl<T: Send, R: Send> Relationship<T, R> for ManualRelationship<T, R> {
//     fn my_key(&self) -> String {
//         self.self_key.to_owned()
//     }
//
//     fn their_key(&self) -> String {
//         self.other_key.to_owned()
//     }
// }

impl<T, R> super::RelationshipKeyPairs for ManualRelationship<T, R>
where
    T: Send + Sync,
    R: Send + Sync,
{
    fn a_key_column(&self) -> String {
        self.self_key.to_owned()
    }
    fn b_key_column(&self) -> String {
        self.other_key.to_owned()
    }
}

// RelationshipCompare is used for include Queries
//impl<T, R> RelationshipCompare<T, R> for Manual<R>
//where
//    Self: Relationship<R>,
//    T: PrimaryKeyValue + HasSchema,
//    T::Schema: UniqueIdentifier,
//    R: HasSchema,
//    R::Schema: UniqueIdentifier,
//    R: ForeignKeyPartialEq<T::PrimaryKeyType>,
//{
//    fn is_related(&self, source: &T, other: &R) -> bool {
//        let pk = source.primary_key_value();
//        let fk_field: String = Self::their_key::<R::Schema, T::Schema>(self);
//        other.eq(&fk_field, &pk)
//    }
//}
