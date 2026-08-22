use crate::service::{request::Request, response::Response};

pub struct Call<F: Fn(Request) -> Response> {
    fn_call: F,
}

impl<F: Fn(Request) -> Response> Call<F> {
    pub fn call(&self, request: Request) -> Response {
        (self.fn_call)(request)
    }
}
