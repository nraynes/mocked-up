use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Table {
    data: HashMap<String, String>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.data
    }
}
