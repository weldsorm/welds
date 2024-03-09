use crate::writers::NextParam;
use crate::Syntax;
use welds_connections::Param;

pub type ParamArgs<'a> = Vec<&'a (dyn Param + Sync)>;

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

//  Relationships / SubQueries
pub(crate) mod exists;
pub(crate) mod wherein;

pub(crate) mod orderby;
pub(crate) use orderby::OrderBy;

pub struct ClauseColVal<T> {
    pub null_clause: bool,
    pub not_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: Option<T>,
}

pub trait AsFieldName<T> {
    fn colname(&self) -> &str;
    fn fieldname(&self) -> &str;
}

pub trait ClauseAdder: Send + Sync {
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p;

    /// Returns the SQL snipit for this clause
    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String>;
}

impl<T> ClauseAdder for ClauseColVal<T>
where
    T: Clone + Send + Sync + Param,
{
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        if !self.null_clause {
            if let Some(val) = &self.val {
                args.push(val);
            }
        }
    }

    fn clause(&self, _syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
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
