use welds_connections::{Param, Syntax};
use crate::query::clause::{ClauseAdder, ClauseColManual, ClauseColVal, ClauseColValEqual, ClauseColValIn, ClauseColValList, LogicalClause, LogicalOp, ParamArgs};
use crate::writers::NextParam;


impl LogicalOp {
    pub fn to_str(&self) -> &'static str
    {
        match self {
            LogicalOp::And => "AND",
            LogicalOp::Or => "OR",
        }
    }
}

pub fn or(
    left_clause: Box<dyn ClauseAdder>,
    right_clause: Box<dyn ClauseAdder>,
) -> Box<LogicalClause> {
    Box::new(LogicalClause {
        left_clause,
        operator: LogicalOp::Or,
        right_clause,
    })
}

pub fn and(
    left_clause: Box<dyn ClauseAdder>,
    right_clause: Box<dyn ClauseAdder>,
) -> Box<LogicalClause> {
    Box::new(LogicalClause {
        left_clause,
        operator: LogicalOp::And,
        right_clause,
    })
}

impl ClauseAdder for LogicalClause {
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        self.left_clause.bind(args);
        self.right_clause.bind(args);
    }

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {

        format!("({} {} {})",
                self.left_clause.clause(syntax, alias, next_params)?,
                self.operator.to_str(),
                self.right_clause.clause(syntax, alias, next_params)?,
        ).into()
    }
}

pub trait AndOrClauseTrait {
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause>;
    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause>;
}

impl<T> AndOrClauseTrait for ClauseColVal<T>
where
        for<'a> T: 'a,
        T: Clone + Send + Sync + Param,
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
}

impl<T> AndOrClauseTrait for ClauseColValEqual<T>
where
        for<'a> T: 'a,
    T: Clone + Send + Sync + Param,
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
}

impl<T> AndOrClauseTrait for ClauseColValList< T >
where
        for<'a> T: 'a,
        Vec<T>:  Clone + Send + Sync + Param,
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
}

impl<T> AndOrClauseTrait for ClauseColValIn<T>
where
    for<'a> T: 'a + Clone + Send + Sync + Param,
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
}

impl AndOrClauseTrait for ClauseColManual {
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
}

