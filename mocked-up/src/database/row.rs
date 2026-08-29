use std::{collections::HashMap, rc::Rc};

use crate::database::{cell::Cell, column::Column, map::Map};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Row {
    values: Map<Rc<Column>, Rc<Cell>>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            values: Map::new(HashMap::new()),
        }
    }

    pub fn get<'a>(&'a self, column: &Column) -> Option<&'a Rc<Cell>> {
        self.values.values().get(column)
    }
}
