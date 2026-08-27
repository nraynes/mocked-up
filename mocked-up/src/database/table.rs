use std::collections::HashMap;

use derive_getters::Getters;

#[derive(Getters, PartialEq, Debug)]
pub struct Table {
    data: HashMap<String, String>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
