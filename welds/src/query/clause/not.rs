use crate::query::clause::{ClauseAdder, ParamArgs};
use crate::writers::NextParam;
use welds_connections::Syntax;

pub struct NegateClause {
    inner: Box<dyn ClauseAdder>,
}

pub fn not(inner: Box<dyn ClauseAdder>) -> Box<dyn ClauseAdder> {
    Box::new(NegateClause { inner })
}

impl ClauseAdder for NegateClause {
    fn bind<'lam, 'args, 'p>(&'lam self, args: &'args mut ParamArgs<'p>)
    where
        'lam: 'p,
    {
        self.inner.bind(args);
    }

    fn clause(&self, syntax: Syntax, alias: &str, next_params: &NextParam) -> Option<String> {
        let inner_clause = self.inner.clause(syntax, alias, next_params)?;
        Some(format!("(NOT ({inner_clause}))"))
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
    fn should_be_able_to_write_a_not_clause() {
        let a = TestModelSchema::default();

        let not_clause = not(a.id.equal(1));
        let sql = not_clause.clause(Syntax::Postgres, "t1", &NextParam::new(Syntax::Postgres));
        assert!(sql.is_some());
        assert_eq!(sql.unwrap(), r#"(NOT (t1."id" = $1))"#);
    }
}
