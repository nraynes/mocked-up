use std::rc::Rc;

use crate::database::{column::Column, key::Primary};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Foreign {
    refers_to: Rc<Primary>,
    this_column: Rc<Column>,
}
