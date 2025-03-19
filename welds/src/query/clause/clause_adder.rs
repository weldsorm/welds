use super::{ClauseColManual, ClauseColVal, ClauseColValEqual, ClauseColValIn, ClauseColValList};
use super::{Param, ParamArgs};
use crate::writers::NextParam;
use crate::Syntax;

/// A `ClauseAdder` is a trait used to write the "clause" part of
/// a where, join, etc..
/// An equality check
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

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
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

        // override the "ilike" operator for sqlite
        let operator = match syntax {
            Syntax::Sqlite => {
                if self.operator == "ilike" {
                    "like"
                } else if self.operator == "not ilike" {
                    "not like"
                } else {
                    self.operator
                }
            }
            _ => self.operator,
        };

        // normal path
        parts.push(operator);
        let np = next_params.next();
        parts.push(&np);
        let clause: String = parts.join(" ");
        Some(clause)
    }
}

impl<T> ClauseAdder for ClauseColValEqual<T>
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

impl<T> ClauseAdder for ClauseColValList<T>
where
    Vec<T>: Clone + Send + Sync + Param,
{
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        args.push(&self.list);
    }

    fn clause(&self, _syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        // build the column name
        let col = format!("{}.{}", alias, self.col);
        let mut parts = vec![col.as_str()];

        // normal path
        parts.push(self.operator);
        let np = next_params.next();
        parts.push("(");
        parts.push(&np);
        parts.push(")");
        let clause: String = parts.join("");
        Some(clause)
    }
}

impl<T> ClauseAdder for ClauseColValIn<T>
where
    T: Clone + Send + Sync + Param,
{
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for item in &self.list {
            args.push(item);
        }
    }

    fn clause(&self, _syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        let col = format!("{}.{}", alias, self.col);
        let mut parts = vec![col];

        parts.push(" ".to_string());
        parts.push(self.operator.to_string());
        parts.push(" ".to_string());
        parts.push("(".to_string());
        for (i, _in) in self.list.iter().enumerate() {
            if i > 0 {
                parts.push(",".to_string())
            }
            parts.push(next_params.next());
        }
        parts.push(")".to_string());
        let clause: String = parts.join("");
        Some(clause)
    }
}

impl ClauseAdder for ClauseColManual {
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for p in &self.params {
            args.push(p.as_ref());
        }
    }

    fn clause(&self, _syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        // build the column name
        let mut parts = vec![];
        if let Some(colname) = &self.col {
            let col = format!("{}.{} ", alias, colname);
            parts.push(col);
        }

        // swap out all the '?' with the correct params type for the syntax
        // swap out all the '$' the table prefix/alias used in this table
        for char in self.sql.chars() {
            match char {
                '?' => parts.push(next_params.next()),
                '$' => parts.push(alias.to_string()),
                _ => parts.push(char.to_string()),
            }
        }

        let clause = parts.join("");
        Some(clause)
    }
}
