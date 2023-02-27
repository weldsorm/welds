mod basic;
pub use basic::Basic;
mod basicopt;
pub use basicopt::BasicOpt;
mod numeric;
pub use numeric::Numeric;
mod numericopt;
pub use numericopt::NumericOpt;

type QB<'q, DB> = sqlx::QueryBuilder<'q, DB>;

pub trait QueryBuilderAdder<'args, DB: sqlx::Database> {
    fn append_to(&self, qb: &mut QB<'args, DB>);
}

pub struct ClauseColVal<T> {
    pub isnull_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: T,
}

impl<'args, T, DB> QueryBuilderAdder<'args, DB> for ClauseColVal<T>
where
    DB: sqlx::Database,
    T: 'args + Clone + Send + sqlx::Type<DB> + sqlx::Encode<'args, DB>,
{
    fn append_to(&self, qb: &mut QB<'args, DB>) {
        qb.push(self.col.clone());
        qb.push(self.operator);
        qb.push_bind(self.val.clone());
    }
}
