use derive_getters::Getters;
use derive_new::new;

use crate::service::status::Status;

#[derive(Getters, new)]
pub struct Response {
    data: Vec<u8>,
    status: Status,
}

impl Response {
    pub fn deserialize_data<'a, T: From<&'a Vec<u8>>>(&'a self) -> T {
        T::from(&self.data)
    }
}
