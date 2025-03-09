use crate::model_traits::{HasSchema, PkValue, UniqueIdentifier};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct BelongsTo<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> BelongsTo<T> {
    pub fn using(fk: &'static str) -> BelongsTo<T> {
        BelongsTo {
            _t: Default::default(),
            foreign_key: fk,
        }
    }

    pub fn combine_related<Output, Parent, Child>(
        &self,
        lambda: impl Fn(Child, Parent) -> Output,
        children: &[Child],
        parents: &[Parent],
    ) -> Vec<Output>
    where
        Parent: HasSchema + Related<HasMany<Child>> + PkValue + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
        <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>
    {
        self.group_related(children, parents)
            .into_iter()
            .zip(children)
            .map(|(parents, child)| lambda(child.to_owned(), parents.first().unwrap().to_owned()))
            .collect()
    }

    pub fn group_related<Child, Parent>(&self, children: &[Child], parents: &[Parent]) -> Vec<Vec<Parent>>
    where
        Parent: HasSchema + Related<HasMany<Child>> + PkValue + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
        <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>,
    {
        let mut grouped: Vec<Vec<Parent>> = children.iter().map(|_| Vec::new()).collect();

        let indexed: HashMap<_, _> = parents.iter().enumerate()
            .map(|(index, parent)| {
                (parent.pk_value(), index)
            }).collect();

        let _: Vec<_> = children.iter().enumerate().map(|(c_index, child)| {
            if let Some(p_index) = indexed.get(&child.fk_value::<Parent>()) {
                grouped[c_index].push(parents.get(*p_index).unwrap().to_owned());
            }
        }).collect();

        grouped
    }
}

impl<R> Relationship<R> for BelongsTo<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        ME::id_column().name().to_owned()
    }
}

pub struct HasMany<T: ?Sized> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> HasMany<T> {
    pub fn using(fk: &'static str) -> HasMany<T> {
        HasMany {
            _t: Default::default(),
            foreign_key: fk,
        }
    }

    pub fn combine_related<Output, Parent, Child>(
        &self,
        lambda: impl Fn(Parent, Vec<Child>) -> Output,
        parents: &[Parent],
        children: &[Child],
    ) -> Vec<Output>
    where
        Parent: HasSchema + Related<HasMany<Child>> + PkValue + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
        <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>
    {
        self.group_related(parents, children)
            .into_iter()
            .zip(parents)
            .map(|(children, parent)| lambda(parent.to_owned(), children))
            .collect()
    }

    pub fn group_related<Parent, Child>(&self, parents: &[Parent], children: &[Child]) -> Vec<Vec<Child>>
    where
        Parent: HasSchema + Related<HasMany<Child>> + PkValue,
        Child: HasSchema + Related<BelongsTo<Parent>> + BelongsToFkValue<Parent> + ToOwned<Owned = Child>,
        <Parent as PkValue>::PkVal: Borrow<<Child as BelongsToFkValue<Parent>>::FkVal>
    {
        let mut grouped = Vec::new();

        let indexed: HashMap<_, _> = parents.iter().enumerate()
            .map(|(index, parent)| {
                grouped.push(Vec::new());
                (parent.pk_value(), index)
            }).collect();

        for child in children {
            if let Some(index) = indexed.get(&child.fk_value::<Parent>()) {
                grouped[*index].push(child.to_owned());
            }
        }

        grouped
    }
}

impl<R> Relationship<R> for HasMany<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        THEM::id_column().name().to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
}

pub struct HasOne<T> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> HasOne<T> {
    pub fn using(fk: &'static str) -> HasOne<T> {
        HasOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }

    pub fn combine_related<Output, Parent, Child>(
        &self,
        lambda: impl Fn(Parent, Vec<Child>) -> Output,
        parents: &[Parent],
        children: &[Child],
    ) -> Vec<Output>
    where
        Parent: HasSchema + Related<HasOne<Child>> + HasOneFkValue<Child> + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsToOne<Parent>> + PkValue + ToOwned<Owned = Child>,
        <Parent as HasOneFkValue<Child>>::HasOneFkValInner: Borrow<<Child as PkValue>::PkVal>
    {
        self.group_related(parents, children)
            .into_iter()
            .zip(parents)
            .map(|(children, parent)| lambda(parent.to_owned(), children))
            .collect()
    }

    pub fn group_related<Parent, Child>(&self, parents: &[Parent], children: &[Child]) -> Vec<Vec<Child>>
    where
        Parent: HasSchema + Related<HasOne<Child>> + HasOneFkValue<Child>,
        Child: HasSchema + Related<BelongsToOne<Parent>> + PkValue + ToOwned<Owned = Child>,
        <Parent as HasOneFkValue<Child>>::HasOneFkValInner: Borrow<<Child as PkValue>::PkVal>
    {
        let mut grouped = Vec::new();

        let indexed: HashMap<_, _> = parents.iter().enumerate()
            .map(|(index, parent)| {
                grouped.push(Vec::new());
                (parent.fk_value_inner::<Child>(), index)
            }).collect();

        for child in children {
            if let Some(index) = indexed.get(&child.pk_value()) {
                grouped[*index].push(child.to_owned());
            }
        }

        grouped
    }
}

impl<R> Relationship<R> for HasOne<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        ME::id_column().name().to_owned()
    }
}

pub struct BelongsToOne<T: ?Sized> {
    _t: PhantomData<T>,
    foreign_key: &'static str,
}

impl<T> BelongsToOne<T> {
    pub fn using(fk: &'static str) -> BelongsToOne<T> {
        BelongsToOne {
            _t: Default::default(),
            foreign_key: fk,
        }
    }

    pub fn combine_related<Output, Child, Parent>(
        &self,
        lambda: impl Fn(Child, Parent) -> Output,
        children: &[Child],
        parents: &[Parent],
    ) -> Vec<Output>
    where
        Parent: HasSchema + Related<HasOne<Child>> + HasOneFkValue<Child> + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsToOne<Parent>> + PkValue + ToOwned<Owned = Child>,
        <Child as PkValue>::PkVal: Borrow<<Parent as HasOneFkValue<Child>>::HasOneFkValInner>
    {
        self.group_related(children, parents)
            .into_iter()
            .zip(children)
            .map(|(parents, child)| lambda(child.to_owned(), parents.first().unwrap().to_owned()))
            .collect()
    }

    pub fn group_related<Child, Parent>(&self, children: &[Child], parents: &[Parent]) -> Vec<Vec<Parent>>
    where
        Parent: HasSchema + Related<HasOne<Child>> + HasOneFkValue<Child> + ToOwned<Owned = Parent>,
        Child: HasSchema + Related<BelongsToOne<Parent>> + PkValue + ToOwned<Owned = Child>,
        <Child as PkValue>::PkVal: Borrow<<Parent as HasOneFkValue<Child>>::HasOneFkValInner>
    {
        let mut grouped = Vec::new();

        let indexed: HashMap<_, _> = children.iter().enumerate()
            .map(|(index, child)| {
                grouped.push(Vec::new());
                (child.pk_value(), index)
            }).collect();

        for parent in parents {
            if let Some(index) = indexed.get(&parent.fk_value_inner::<Child>()) {
                grouped[*index].push(parent.to_owned());
            }
        }

        grouped
    }
}

impl<R> Relationship<R> for BelongsToOne<R> {
    fn my_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        THEM::id_column().name().to_owned()
    }
    fn their_key<ME, THEM>(&self) -> String
    where
        ME: UniqueIdentifier,
        THEM: UniqueIdentifier,
    {
        self.foreign_key.to_owned()
    }
}

pub trait Relationship<R> {
    fn their_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;

    fn my_key<R2, T>(&self) -> String
    where
        T: UniqueIdentifier,
        R2: UniqueIdentifier;
}

pub trait Related<R> {}

pub trait BelongsToFkValue<R>
where
    Self: HasSchema + Related<BelongsTo<R>>,
    R: HasSchema + Related<HasMany<Self>>
{
    type FkVal: Hash + Eq + 'static;

    /// Returns the value of a model's foreign key for relationship with <T>
    fn fk_value<T>(&self) -> Self::FkVal;
}

pub trait HasOneFkValue<R>
where
    Self: HasSchema + Related<HasOne<R>>,
    R: HasSchema + Related<BelongsToOne<Self>>
{
    type HasOneFkVal: Hash + Eq + 'static;
    type HasOneFkValInner: Hash + Eq + 'static;

    /// Returns the value of a model's foreign key for relationship with <T>
    fn fk_value<T>(&self) -> Self::HasOneFkVal;
    fn fk_value_inner<T>(&self) -> Self::HasOneFkValInner;
}

pub trait HasRelations {
    type Relation: Default;

    fn relations() -> Self::Relation;
}

pub trait RelationAdder {}
