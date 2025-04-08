use crate::migrations::MigrationWriter;
use crate::migrations::utils::split_sql_commands;

pub struct Manual {
    up: String,
    down: String,
}

impl Manual {
    /// write some RAW sql that will be executed as a migration
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn up(sql: impl Into<String>) -> Manual {
        Manual {
            up: sql.into(),
            down: "".to_string(),
        }
    }

    /// Write the down part of this manual migration.
    /// NOTE: you can write multiple commands executing each after ';'
    pub fn down(mut self, sql: impl Into<String>) -> Manual {
        self.down = sql.into();
        self
    }
}

impl MigrationWriter for Manual {
    fn up_sql(&self, _syntax: welds_connections::Syntax) -> Vec<String> {
        split_sql_commands(&self.up)
    }

    fn down_sql(&self, _syntax: welds_connections::Syntax) -> Vec<String> {
        split_sql_commands(&self.down)
    }
}
