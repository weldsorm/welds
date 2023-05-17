use sqlx::database::HasArguments;
use sqlx::Arguments;

// Concrete Types
mod basic;
pub use basic::Basic;
mod basicopt;
pub use basicopt::BasicOpt;
mod numeric;
pub use numeric::Numeric;
mod numericopt;
pub use numericopt::NumericOpt;
mod text;
pub use text::Text;
mod textopt;
pub use textopt::TextOpt;

// Relationships / SubQueries
pub(crate) mod exists;
pub(crate) mod wherein;

mod nextparam;
pub use nextparam::{DbParam, NextParam};
pub(crate) mod orderby;
pub(crate) use orderby::OrderBy;

pub struct ClauseColVal<T> {
    pub null_clause: bool,
    pub not_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: T,
}

pub trait AsFieldName<T> {
    fn colname(&self) -> &str;
    fn fieldname(&self) -> &str;
}

pub trait ClauseAdder<'args, DB: sqlx::Database> {
    /// Add the argument to the list of Arguments to send to the database
    fn bind(&self, args: &mut <DB as HasArguments<'args>>::Arguments);

    /// Returns the SQL snipit for this clause
    fn clause(&self, alias: &str, next_params: &NextParam) -> Option<String>;
}

impl<'args, T, DB> ClauseAdder<'args, DB> for ClauseColVal<T>
where
    DB: sqlx::Database,
    T: 'args + Clone + Send + sqlx::Type<DB> + sqlx::Encode<'args, DB>,
{
    fn bind(&self, args: &mut <DB as HasArguments<'args>>::Arguments) {
        if !self.null_clause {
            args.add(self.val.clone());
        }
    }

    fn clause(&self, alias: &str, next_params: &NextParam) -> Option<String> {
        // build the column name
        let col = format!("{}.{}", alias, self.col);
        let mut parts = vec![col.as_str()];

        // handle null clones
        if self.null_clause {
            parts.push("IS");
            if self.not_clause {
                parts.push("NOT");
            }
            parts.push("NULL");
            let clause: String = parts.join(" ");
            return Some(clause);
        }

        // normal path
        parts.push(self.operator);
        let np = next_params.next();
        parts.push(&np);
        let clause: String = parts.join(" ");
        Some(clause)
    }
}
