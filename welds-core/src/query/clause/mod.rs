mod basic;
pub use basic::Basic;
mod basicopt;
pub use basicopt::BasicOpt;
mod numeric;
pub use numeric::Numeric;
mod numericopt;
pub use numericopt::NumericOpt;

type QB<'q> = crate::query::GenericQueryBuilder<'q>;

pub trait QueryBuilderAdder<'args> {
    fn append_to(&self, qb: &mut QB<'args>);
}

pub struct ClauseColVal<T> {
    pub isnull_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: T,
}

impl<'args, T> QueryBuilderAdder<'args> for ClauseColVal<T>
where
    T: 'args + Clone + crate::row::ToRow<'args>,
{
    fn append_to(&self, qb: &mut QB<'args>) {
        qb.push(self.col.clone());
        qb.push(self.operator);
        qb.push_bind(self.val.clone());
    }
}
