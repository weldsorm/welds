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
            #[cfg(feature = "mysql")]
            Syntax::Mysql => MySql::count(prefix, x),
            #[cfg(feature = "postgres")]
            Syntax::Postgres => Postgres::count(prefix, x),
            #[cfg(feature = "sqlite")]
            Syntax::Sqlite => Sqlite::count(prefix, x),
            #[cfg(feature = "mssql")]
            Syntax::Mssql => Mssql::count(prefix, x),
        }
    }
}

#[cfg(feature = "postgres")]
struct Postgres;

#[cfg(feature = "postgres")]
impl Postgres {
    fn count(prefix: Option<&str>, x: Option<&str>) -> String {
        let mut x = x.unwrap_or("*").to_owned();
        if let Some(prefix) = prefix {
            x = format!("{}.{}", prefix, x);
        }
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "sqlite")]
struct Sqlite;

#[cfg(feature = "sqlite")]
impl Sqlite {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mssql")]
struct Mssql;

#[cfg(feature = "mssql")]
impl Mssql {
    fn count(_prefix: Option<&str>, x: Option<&str>) -> String {
        let x = x.unwrap_or("*");
        format!("CAST( COUNT({}) as BIGINT )", x)
    }
}

#[cfg(feature = "mysql")]
struct MySql;

#[cfg(feature = "mysql")]
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
