use std::cell::Cell;
use std::cell::RefCell;

pub struct TableAlias {
    i: Cell<u32>,
    force: RefCell<Option<String>>,
}

impl Default for TableAlias {
    fn default() -> Self {
        Self::new()
    }
}

impl TableAlias {
    pub fn new() -> Self {
        TableAlias {
            i: Cell::new(1),
            force: Default::default(),
        }
    }

    /// Get the next Alias and bump it
    pub fn next(&self) -> String {
        let id = format!("t{}", self.i.get());
        self.i.set(self.i.get() + 1);
        let val = match self.force.borrow().as_ref() {
            Some(f) => f.clone(),
            None => id,
        };
        *self.force.borrow_mut() = None;
        val
    }

    /// Peek what the next value will be without bumping it
    pub fn peek(&self) -> String {
        let id = format!("t{}", self.i.get());
        match self.force.borrow().as_ref() {
            Some(f) => f.clone(),
            None => id,
        }
    }

    /// Bump the table Alias to the next
    pub fn bump(&self) {
        *self.force.borrow_mut() = None;
        self.i.set(self.i.get() + 1);
    }

    /// Force the next alias to be a specific value
    pub fn force_next(&self, force: String) {
        *self.force.borrow_mut() = Some(force);
    }
}
