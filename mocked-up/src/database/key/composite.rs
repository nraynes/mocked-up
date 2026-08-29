use std::rc::Rc;

use crate::database::key::Alternate;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Composite {
    composed_of: Vec<Rc<Alternate>>,
}
