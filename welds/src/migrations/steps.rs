use crate::migrations::MigrationWriter;
use welds_connections::Syntax;

/// Batch several DB changes into a single migration
#[derive(Default)]
pub struct Steps {
    inner: Vec<Box<dyn MigrationWriter>>,
}

impl Steps {
    pub fn new() -> Self {
        Steps::default()
    }

    /// Adds a step to the Batch of steps to run for this migration
    pub fn add<M: 'static + MigrationWriter>(mut self, step: M) -> Self {
        self.inner.push(Box::new(step));
        self
    }
}

impl MigrationWriter for Steps {
    fn up_sql(&self, syntax: Syntax) -> Vec<String> {
        self.inner.iter().flat_map(|s| s.up_sql(syntax)).collect()
    }

    fn down_sql(&self, syntax: Syntax) -> Vec<String> {
        self.inner.iter().flat_map(|s| s.down_sql(syntax)).collect()
    }
}
