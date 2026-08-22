use std::collections::HashMap;

use derive_getters::Getters;
use derive_new::new;

#[derive(Getters, new)]
pub struct Request {
    headers: HashMap<String, String>,
    body: Vec<u8>,
    query: HashMap<String, String>,
}

impl Request {
    pub fn deserialize_body<'a, T: From<&'a Vec<u8>>>(&'a self) -> T {
        T::from(&self.body)
    }
}
