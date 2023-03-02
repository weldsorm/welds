use sqlx::database::HasArguments;
use sqlx::Arguments;

mod basic;
pub use basic::Basic;
mod basicopt;
pub use basicopt::BasicOpt;
mod numeric;
pub use numeric::Numeric;
mod numericopt;
pub use numericopt::NumericOpt;
mod nextparam;
pub use nextparam::{DbParam, NextParam};

pub struct ClauseColVal<T> {
    pub null_clause: Option<String>,
    pub col: String,
    pub operator: &'static str,
    pub val: T,
}

pub trait ClauseAdder<'args, DB: sqlx::Database> {
    /// Add the argument to the list of Arguments to send to the database
    fn bind(&self, args: &mut <DB as HasArguments<'args>>::Arguments);

    /// Returns the SQL snipit for this clause
    fn clause(&self, next_params: &NextParam) -> Option<String>;
}

impl<'args, T, DB> ClauseAdder<'args, DB> for ClauseColVal<T>
where
    DB: sqlx::Database,
    T: 'args + Clone + Send + sqlx::Type<DB> + sqlx::Encode<'args, DB>,
{
    fn bind(&self, args: &mut <DB as HasArguments<'args>>::Arguments) {
        if self.null_clause.is_none() {
            args.add(self.val.clone());
        }
    }

    fn clause(&self, next_params: &NextParam) -> Option<String> {
        // handle null clones
        if let Some(null_clause) = &self.null_clause {
            return Some(null_clause.clone());
        }

        // normal path
        let mut qb: Vec<&str> = Vec::default();
        qb.push(&self.col);
        qb.push(self.operator);
        let np = next_params.next();
        qb.push(&np);
        let clause: String = qb.join(" ");
        Some(clause)
    }
}
