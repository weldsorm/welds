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

impl AndOrClauseTrait for LogicalClause
{
    fn and(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        and(self, other)
    }

    fn or(self: Box<Self>, other: Box<dyn ClauseAdder>) -> Box<LogicalClause> {
        or(self, other)
    }
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
        assert_eq!(sql_str, "((t1.id = $1 AND t1.is_active = $2) OR t1.score >= $3)");
    }

    #[test]
    fn test_nested_linked_logical_clauses() {
        let a = TestModelSchema::default();

        // ((id != 0 OR name_column == 'empty) AND is_active != true) OR score > 0.5
        let or_clause = a.id.not_equal(0).or( a.name.equal("empty"));
        let and_clause = or_clause.and( a.is_active.not_equal(true));
        let nested = and_clause.or( a.score.gte(0.5));

        let sql = nested.clause(Syntax::Postgres, "t1", &NextParam::new(Syntax::Postgres));
        assert!(sql.is_some());
        let sql_str = sql.unwrap();
        assert!(sql_str.contains("AND"));
        assert!(sql_str.contains("OR"));
        assert_eq!(sql_str, "(((t1.id != $1 OR t1.name_column = $2) AND t1.is_active != $3) OR t1.score >= $4)");
    }
}
