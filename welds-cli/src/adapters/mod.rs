pub mod sqlite;

use crate::errors::Result;
use crate::schema::Table;
use welds::database::Pool;

pub async fn schema(pool: &Pool) -> Result<Vec<Table>> {
    match pool {
        Pool::Sqlite(p) => sqlite::schema(p).await,
        _ => panic!("NOT YET"),
    }
}

#[derive(Debug, Clone)]
pub struct TableIdent {
    pub schema: String,
    pub name: String,
}

impl TableIdent {
    pub fn new(text: &str, pool: &Pool) -> TableIdent {
        let parts: Vec<&str> = text.split(".").collect();
        let parts: Vec<&str> = parts.iter().rev().take(2).cloned().collect();
        let name = parts.get(0).cloned().unwrap_or_default();
        let schema = parts.get(1).cloned();

        // of not given use default schema
        let schema = schema.unwrap_or_else(|| match pool {
            Pool::Sqlite(_) => "main",
            Pool::MySql(_) => "",
            Pool::Mssql(_) => "dbo",
            Pool::Postgres(_) => "public",
        });

        TableIdent {
            name: name.to_string(),
            schema: schema.to_string(),
        }
    }
}

impl PartialEq for TableIdent {
    fn eq(&self, other: &Self) -> bool {
        let s = self.schema.to_uppercase();
        let n = self.name.to_uppercase();
        let os = other.schema.to_uppercase();
        let on = other.name.to_uppercase();
        n == on && s == os
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}
