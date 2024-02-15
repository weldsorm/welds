use std::sync::Mutex;

pub struct TableAlias {
    i: Mutex<u32>,
}

impl Default for TableAlias {
    fn default() -> Self {
        Self::new()
    }
}

impl TableAlias {
    pub fn new() -> Self {
        TableAlias { i: Mutex::new(1) }
    }

    /// Get the next Alias and bump it
    pub fn next(&self) -> String {
        let mut i = self.i.lock().unwrap();
        let id = format!("t{}", *i);
        *i += 1;
        id
    }
}
