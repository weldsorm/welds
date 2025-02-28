use super::{AssignmentManual, ClauseColValEqual, SetColNull, SetColVal};
use super::{Param, ParamArgs};
use crate::writers::ColumnWriter;
use crate::writers::NextParam;
use crate::Syntax;

/// A `AssignmentAdder` is a trait used to write the "assignment" part of an update
pub trait AssignmentAdder: Send + Sync {
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p;
    /// Returns the SQL snipit for this clause
    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String>;
}

impl<T> AssignmentAdder for ClauseColValEqual<T>
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

    fn clause(&self, syntax: Syntax, _alias: &str, next_params: &NextParam) -> Option<String> {
        // build the column name
        let colname = ColumnWriter::new(syntax).excape(&self.col);
        let mut parts = vec![colname.as_str()];

        // handle null clones
        if self.null_clause {
            parts.push("=");
            parts.push("NULL");
            let clause: String = parts.join("");
            return Some(clause);
        }

        // normal path
        parts.push(self.operator);
        let np = next_params.next();
        parts.push(&np);
        let clause: String = parts.join("");
        Some(clause)
    }
}

impl<T> AssignmentAdder for SetColVal<T>
where
    T: Clone + Send + Sync + Param,
{
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        args.push(&self.val);
    }

    /// Returns the SQL snipit for this clause
    fn clause(&self, syntax: Syntax, _alias: &str, next_params: &NextParam) -> Option<String> {
        let colname = ColumnWriter::new(syntax).excape(&self.col_raw);
        let sql = format!("{}={}", colname, next_params.next());
        Some(sql)
    }
}

impl AssignmentAdder for SetColNull {
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, _args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        // no args added
    }

    /// Returns the SQL snipit for this clause
    fn clause(&self, syntax: Syntax, _alias: &str, _next_params: &NextParam) -> Option<String> {
        let colname = ColumnWriter::new(syntax).excape(&self.col_raw);
        let sql = format!("{}=NULL", colname);
        Some(sql)
    }
}

impl AssignmentAdder for AssignmentManual {
    /// Add the argument to the list of Arguments to send to the database
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        for p in &self.params {
            args.push(p.as_ref());
        }
    }

    /// Returns the SQL snipit for this clause
    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        // build the column name
        let mut parts = vec![];

        let colname = ColumnWriter::new(syntax).excape(&self.col);
        parts.push(colname);
        parts.push(" = ( ".to_string());

        // swap out all the '?' with the correct params type for the syntax
        // swap out all the '$' the table prefix/alias used in this table
        for char in self.sql.chars() {
            match char {
                '?' => parts.push(next_params.next()),
                '$' => parts.push(alias.to_string()),
                _ => parts.push(char.to_string()),
            }
        }
        parts.push(" )".to_string());

        let clause = parts.join("");
        Some(clause)
    }
}
