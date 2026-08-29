use std::rc::Rc;

use crate::database::column::Column;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Primary {
    sequential: bool,
    this_column: Rc<Column>,
}
