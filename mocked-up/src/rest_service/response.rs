use derive_getters::Getters;
use derive_new::new;

use crate::{MockError, rest_service::status::Status};

#[derive(Getters, new)]
pub struct Response {
    data: String,
    status: Status,
}

impl Response {
    pub fn no_data(status: Status) -> Self {
        Self {
            data: String::new(),
            status: status,
        }
    }

    pub fn deserialize_data<'a, T, E, D: Fn(&'a str) -> Result<T, E>>(
        &'a self,
        de: D,
    ) -> Result<T, MockError>
    where
        MockError: From<E>,
    {
        Ok(de(&self.data)?)
    }
}
