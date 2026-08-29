#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Cell {
    value: String,
}

impl Cell {
    pub fn get(&self) -> &String {
        &self.value
    }

    pub fn set(&mut self, value: String) {
        self.value = value;
    }
}
