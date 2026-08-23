use std::collections::HashMap;

use derive_getters::Getters;
use derive_new::new;

use crate::MockError;

#[derive(Getters, new)]
pub struct Request {
    headers: HashMap<String, String>,
    body: String,
    query: HashMap<String, String>,
}

impl Request {
    pub fn deserialize_body<'a, T, E, D: Fn(&'a str) -> Result<T, E>>(
        &'a self,
        de: D,
    ) -> Result<T, MockError>
    where
        MockError: From<E>,
    {
        Ok(de(&self.body)?)
    }
}
