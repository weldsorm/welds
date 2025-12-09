use welds_connections::Syntax;
use crate::query::clause::{ClauseAdder, ParamArgs};
use crate::writers::NextParam;

enum LogicalOp {
    And,
    Or,
}

impl LogicalOp {
    pub fn to_str(&self) -> &'static str
    {
        match self {
            LogicalOp::And => "AND",
            LogicalOp::Or => "OR",
        }
    }
}

pub struct LogicalClause {
    left_clause: Box<dyn ClauseAdder>,
    operator: LogicalOp,
    right_clause: Box<dyn ClauseAdder>,
}

impl LogicalClause {
    pub fn or(
        left_clause: Box<dyn ClauseAdder>,
        right_clause: Box<dyn ClauseAdder>,
    ) -> Self {
        Self {
            left_clause,
            operator: LogicalOp::Or,
            right_clause,
        }
    }

    pub fn and(
        left_clause: Box<dyn ClauseAdder>,
        right_clause: Box<dyn ClauseAdder>,
    ) -> Self {
        Self {
            left_clause,
            operator: LogicalOp::And,
            right_clause,
        }
    }
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

        format!("({} {} {}",
                self.left_clause.clause(syntax, alias, next_params)?,
                self.operator.to_str(),
                self.right_clause.clause(syntax, alias, next_params)?,
        ).into()
    }
}
