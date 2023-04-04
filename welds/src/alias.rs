use std::cell::Cell;

pub struct TableAlias {
    i: Cell<u32>,
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

    /// Peek what the next value will be without bumping it
    pub fn peek(&self) -> String {
        format!("t{}", self.i.get())
    }

    /// Bump the table Alias to the next
    pub fn bump(&self) {
        self.i.set(self.i.get() + 1);
    }
}
