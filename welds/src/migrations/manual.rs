use welds_connections::Syntax;

use crate::migrations::MigrationWriter;
use crate::migrations::utils::split_sql_commands;
use std::collections::HashMap;

pub struct Manual {
    up_default: String,
    down_default: String,
    up_syntax: HashMap<Syntax, String>,
    down_syntax: HashMap<Syntax, String>,
}

impl Manual {
    /// write some RAW sql that will be executed as a migration
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn up(sql: impl Into<String>) -> Manual {
        Manual {
            up_default: sql.into(),
            down_default: "".to_string(),
            up_syntax: HashMap::new(),
            down_syntax: HashMap::new(),
        }
    }

    /// Write the down part of this manual migration.
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn down(mut self, sql: impl Into<String>) -> Manual {
        self.down_default = sql.into();
        self
    }

    /// write some RAW for a specific Database syntax
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn up_for(mut self, syntax: Syntax, sql: impl Into<String>) -> Manual {
        self.up_syntax.insert(syntax, sql.into());
        self
    }

    /// Write the down part of this manual migration for only a given syntax
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn down_for(mut self, sql: impl Into<String>) -> Manual {
        self.down_default = sql.into();
        self
    }
}

impl MigrationWriter for Manual {
    fn up_sql(&self, syntax: welds_connections::Syntax) -> Vec<String> {
        if let Some(sql) = self.up_syntax.get(&syntax) {
            return split_sql_commands(sql);
        }
        split_sql_commands(&self.up_default)
    }

    fn down_sql(&self, syntax: welds_connections::Syntax) -> Vec<String> {
        if let Some(sql) = self.down_syntax.get(&syntax) {
            return split_sql_commands(sql);
        }
        split_sql_commands(&self.down_default)
    }
}
