use std::rc::Rc;

use crate::database::column::Column;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Alternate {
    unique: bool,
    not_null: bool,
    this_column: Rc<Column>,
}
