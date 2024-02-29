use crate::Syntax;

pub struct CountWriter {
    syntax: Syntax,
}

impl CountWriter {
    pub fn new(syntax: Syntax) -> Self {
        Self { syntax }
    }
    pub fn count(&self, prefix: Option<&str>, x: Option<&str>) -> String {
        match self.syntax {
            Syntax::Mysql => MySql::count(prefix, x),
            Syntax::Postgres => Postgres::count(prefix, x),
            Syntax::Sqlite => Sqlite::count(prefix, x),
            Syntax::Mssql => Mssql::count(prefix, x),
        }
    }
}

struct Postgres;
impl Postgres {
    fn count(prefix: Option<&str>, x: Option<&str>) -> String {
        let mut x = x.unwrap_or("*").to_owned();
        if let Some(prefix) = prefix {
            x = format!("{}.{}", prefix, x);
        }
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

struct Sqlite;
impl Sqlite {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

struct Mssql;
impl Mssql {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

struct MySql;
impl MySql {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("COUNT({})", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_should_counts() {
        let w = CountWriter::new(Syntax::Postgres);
        assert_eq!(w.count(None, None), "CAST( COUNT(*) as BIGINT )");
        assert_eq!(
            w.count(None, Some("sheep")),
            "CAST( COUNT(sheep) as BIGINT )"
        );
        assert_eq!(
            w.count(Some("t1"), Some("sheep")),
            "CAST( COUNT(t1.sheep) as BIGINT )"
        );
    }
}
