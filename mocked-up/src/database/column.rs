use std::{collections::HashMap, rc::Rc};

use crate::database::{cell::Cell, key::Key, map::Map, row::Row};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Column {
    key: Key,
    values: Map<Rc<Row>, Rc<Cell>>,
}

impl From<Key> for Column {
    fn from(value: Key) -> Self {
        Self {
            key: value,
            values: Map::new(HashMap::new()),
        }
    }
}

impl Column {}
