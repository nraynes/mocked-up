mod table;

use std::collections::HashMap;

use derive_getters::Getters;
use table::Table;

#[derive(Getters, PartialEq, Debug)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(mut self, name: &str) -> Self {
        self.tables.insert(name.to_string(), Table::new());
        self
    }
}
