use super::{KeyPair, MyKeyEq, Relationship, RelationshipClause, TheirKeyEq};
use std::marker::PhantomData;
use welds_connections::Param;

/// A relationship between two tables that is defined by hand.
///
/// At its simplest it links a column on self to a column on the other table.
/// ```ignore
/// ManualRelationship::<Order, Extra>::using("extra_order_id1", "extra_order_id2")
/// ```
///
/// Additional clauses can be chained on to build a relationship that needs
/// more than a single column to link the tables. All clauses are ANDed together.
/// ```ignore
/// ManualRelationship::<Order, Extra>::using("tenant_id", "tenant_id")
///     .and_using("extra_order_id1", "extra_order_id2")
///     .and_their_eq("kind", "shipping")
/// ```
pub struct ManualRelationship<T, R> {
    _t: PhantomData<T>,
    _r: PhantomData<R>,
    clauses: Vec<ManualClause>,
}

#[derive(Clone, PartialEq)]
enum ManualClause {
    Keys(KeyPair),
    Mine(MyKeyEq),
    Theirs(TheirKeyEq),
}

impl<T, R> ManualRelationship<T, R> {
    /// Link a column on self to a column on the other table
    /// `self_col_key`: The database column on self to use for this relationship
    /// `other_col_key`: The database column on the other table to use for this relationship
    pub fn using(
        self_col_key: &'static str,
        other_col_key: &'static str,
    ) -> ManualRelationship<T, R> {
        ManualRelationship {
            _t: Default::default(),
            _r: Default::default(),
            clauses: vec![ManualClause::Keys(KeyPair::new(
                self_col_key,
                other_col_key,
            ))],
        }
    }

    /// Start building a relationship with no clauses.
    /// Add clauses with `and_using`, `and_my_eq`, and `and_their_eq`
    pub fn empty() -> ManualRelationship<T, R> {
        ManualRelationship {
            _t: Default::default(),
            _r: Default::default(),
            clauses: Vec::default(),
        }
    }

    /// Also require a column on self to equal a column on the other table
    pub fn and_using(mut self, self_col_key: &str, other_col_key: &str) -> Self {
        self.clauses.push(ManualClause::Keys(KeyPair::new(
            self_col_key,
            other_col_key,
        )));
        self
    }

    /// Also require a column on self to equal a given value
    pub fn and_my_eq<P>(mut self, self_col_key: &str, value: P) -> Self
    where
        P: 'static + Param + Send + Sync,
    {
        self.clauses
            .push(ManualClause::Mine(MyKeyEq::new(self_col_key, value)));
        self
    }

    /// Also require a column on the other table to equal a given value
    pub fn and_their_eq<P>(mut self, other_col_key: &str, value: P) -> Self
    where
        P: 'static + Param + Send + Sync,
    {
        self.clauses
            .push(ManualClause::Theirs(TheirKeyEq::new(other_col_key, value)));
        self
    }
}

impl<T, R> PartialEq for ManualRelationship<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.clauses == other.clauses
    }
}

impl<T, R> Clone for ManualRelationship<T, R> {
    fn clone(&self) -> Self {
        Self {
            _t: Default::default(),
            _r: Default::default(),
            clauses: self.clauses.clone(),
        }
    }
}

impl<T: Send, R: Send> Relationship<T, R> for ManualRelationship<T, R> {
    fn clauses(&self) -> Vec<Box<dyn RelationshipClause>> {
        self.clauses
            .iter()
            .map(|c| -> Box<dyn RelationshipClause> {
                match c {
                    ManualClause::Keys(x) => Box::new(x.clone()),
                    ManualClause::Mine(x) => Box::new(x.clone()),
                    ManualClause::Theirs(x) => Box::new(x.clone()),
                }
            })
            .collect()
    }
}
