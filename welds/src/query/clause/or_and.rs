use crate::query::clause::{ClauseAdder, ParamArgs};
use crate::writers::NextParam;
use welds_connections::Syntax;

enum LogicalOp {
    And,
    Or,
}

pub struct LogicalClause {
    left_clause: Box<dyn ClauseAdder>,
    operator: LogicalOp,
    right_clause: Box<dyn ClauseAdder>,
}

impl LogicalOp {
    pub fn to_str(&self) -> &'static str {
        match self {
            LogicalOp::And => "AND",
            LogicalOp::Or => "OR",
        }
    }
}

pub fn or(
    left_clause: Box<dyn ClauseAdder>,
    right_clause: Box<dyn ClauseAdder>,
) -> Box<dyn ClauseAdder> {
    Box::new(LogicalClause {
        left_clause,
        operator: LogicalOp::Or,
        right_clause,
    })
}

pub fn and(
    left_clause: Box<dyn ClauseAdder>,
    right_clause: Box<dyn ClauseAdder>,
) -> Box<dyn ClauseAdder> {
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
        let left = self.left_clause.clause(syntax, alias, next_params);
        let right = self.right_clause.clause(syntax, alias, next_params);
        let operator = self.operator.to_str();

        // Both have some
        if let Some(left) = &left
            && let Some(right) = &right
        {
            return Some(format!("({left} {operator} {right})"));
        }
        // Left has some
        if left.is_some() {
            return left;
        }
        // Right has some
        if right.is_some() {
            return right;
        }
        // Both are none
        None
    }
}

/// Extensions on ClauseAdder to add builder style (and/or) methods
pub trait ClauseAdderAndOrExt {
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<dyn ClauseAdder>;
    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<dyn ClauseAdder>;
}

impl<CA> ClauseAdderAndOrExt for CA
where
    CA: ClauseAdder + 'static,
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<dyn ClauseAdder> {
        and(self, other)
    }
    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<dyn ClauseAdder> {
        or(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WeldsModel;
    use welds_connections::Syntax;

    #[derive(Debug, Default, WeldsModel)]
    #[welds(table = "test_table")]
    #[welds_path(crate)]
    struct TestModel {
        #[welds(primary_key)]
        pub id: i32,
        #[welds(rename = "name_column")]
        pub name: String,
        pub is_active: bool,
        pub score: f64,
    }

    #[test]
    fn test_and_logical_clause() {
        let a = TestModelSchema::default();

        let and_clause = and(a.id.equal(1), a.is_active.equal(true));

        let sql = and_clause.clause(Syntax::Postgres, "t1", &NextParam::new(Syntax::Postgres));
        assert!(sql.is_some());
        assert_eq!(sql.unwrap(), "(t1.id = $1 AND t1.is_active = $2)");
    }

    #[test]
    fn test_or_logical_clause() {
        let a = TestModelSchema::default();

        let or_clause = or(a.score.gt(0.5), a.name.equal("test".to_string()));

        let sql = or_clause.clause(Syntax::Postgres, "t1", &NextParam::new(Syntax::Postgres));
        assert!(sql.is_some());
        assert_eq!(sql.unwrap(), "(t1.score > $1 OR t1.name_column = $2)");
    }

    #[test]
    fn test_nested_logical_clauses() {
        let a = TestModelSchema::default();

        // (id = 1 AND is_active = true) OR score > 0.5
        let and_clause = and(a.id.equal(1), a.is_active.equal(true));
        let nested = or(and_clause, a.score.gte(0.5));

        let sql = nested.clause(Syntax::Postgres, "t1", &NextParam::new(Syntax::Postgres));
        assert!(sql.is_some());
        let sql_str = sql.unwrap();
        assert!(sql_str.contains("AND"));
        assert!(sql_str.contains("OR"));
        assert_eq!(
            sql_str,
            "((t1.id = $1 AND t1.is_active = $2) OR t1.score >= $3)"
        );
    }
}
