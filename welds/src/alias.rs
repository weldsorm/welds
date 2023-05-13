use std::cell::Cell;

pub struct TableAlias {
    i: Cell<u32>,
}

impl Default for TableAlias {
    fn default() -> Self {
        Self::new()
    }
}

impl TableAlias {
    pub fn new() -> Self {
        TableAlias { i: Cell::new(1) }
    }

    /// Get the next Alias and bump it
    pub fn next(&self) -> String {
        let id = format!("t{}", self.i.get());
        self.i.set(self.i.get() + 1);
        id
    }
}
