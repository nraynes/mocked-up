use std::collections::HashMap;

use crate::database::{
    column::Column,
    key::{IdentKey, Key},
    map::Map,
    row::Row,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Table {
    identifier: IdentKey,
    columns: Map<String, Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(identifier: IdentKey, keys: HashMap<String, Key>) -> Self {
        Self {
            identifier,
            columns: Map::from(
                keys.into_iter()
                    .map(|(s, k)| (s, Column::from(k)))
                    .collect::<HashMap<String, Column>>(),
            ),
            rows: Vec::new(),
        }
    }

    pub fn identifier(&self) -> &IdentKey {
        &self.identifier
    }

    pub fn column(&self, name: &str) -> Option<&Column> {
        self.columns.values().get(name)
    }
}
