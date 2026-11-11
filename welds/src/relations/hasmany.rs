use super::Relationship;
use super::RelationshipCompare;
use crate::model_traits::ForeignKeyPartialEq;
use crate::model_traits::HasSchema;
use crate::model_traits::PrimaryKeyValue;
use crate::model_traits::UniqueIdentifier;
use crate::relations::RelationshipKeyPairs;
use crate::writers::ColumnWriter;
use std::marker::PhantomData;

pub struct HasMany<T, R> {
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    foreign_key: &'static str,
}

impl<T, R> HasMany<T, R> {
    pub fn using(fk: &'static str) -> HasMany<T, R> {
        HasMany {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: fk,
        }
    }
}

impl<T, R> PartialEq for HasMany<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.foreign_key == other.foreign_key
    }
}
impl<T, R> Clone for HasMany<T, R> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            _r: Default::default(),
            foreign_key: self.foreign_key,
        }
    }
}

// impl<T: Send, R: Send> Relationship<T, R> for HasMany<T, R>
// where
//     T: HasSchema,
//     <T as HasSchema>::Schema: UniqueIdentifier,
// {
//     fn my_key(&self) -> String {
//         <T as HasSchema>::Schema::id_column().name().to_owned()
//     }
//     fn their_key(&self) -> String {
//         self.foreign_key.to_owned()
//     }
// }

impl<T, R> RelationshipKeyPairs for HasMany<T, R>
where
    T: Send + Sync,
    R: Send + Sync,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier,
{
    fn a_key_column(&self) -> String {
        <T as HasSchema>::Schema::id_column().name().to_owned()
    }
    fn b_key_column(&self) -> String {
        self.foreign_key.to_owned()
    }
}

use super::RelationshipWriteContext;
use super::RelationshipWriter;

impl<T, R> Relationship<T, R> for HasMany<T, R>
where
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier,
{
}

impl<T, R> RelationshipWriter for HasMany<T, R>
where
    T: Send + Sync,
    R: Send + Sync,
    T: HasSchema,
    <T as HasSchema>::Schema: UniqueIdentifier,
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

impl<T, R> RelationshipCompare<T, R> for HasMany<T, R>
where
    Self: Relationship<T, R>,
    T: PrimaryKeyValue + HasSchema,
    T::Schema: UniqueIdentifier,
    R: HasSchema,
    R::Schema: UniqueIdentifier,
    R: ForeignKeyPartialEq<T::PrimaryKeyType>,
{
    fn is_related(&self, source: &T, other: &R) -> bool {
        let pk = source.primary_key_value();
        let fk_field: String = Self::b_key_column(self);
        other.eq(&fk_field, &pk)
    }
}
