mod table;

use std::collections::HashMap;

use table::Table;

#[derive(PartialEq, Debug)]
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

    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.get(&name.to_string())
    }

    pub fn table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(&name.to_string())
    }
}
