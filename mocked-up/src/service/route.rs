use std::collections::HashMap;

use derive_new::new;

use crate::service::{call::Call, request::Request, response::Response};

#[derive(new)]
pub struct Route<F: Fn(Request) -> Response> {
    routes: HashMap<String, Self>,
    caller: Call<F>,
}

impl<F: Fn(Request) -> Response> Route<F> {
    pub fn find<U: AsRef<str>, I: IntoIterator<Item = U>>(&self, route: I) -> Option<&Self> {
        let mut route_iter = route.into_iter();
        match route_iter.next() {
            Some(next_route) => self
                .routes
                .get(next_route.as_ref())
                .and_then(|x| x.find(route_iter)),
            None => Some(self),
        }
    }

    pub fn call(&self, request: Request) -> Response {
        self.caller.call(request)
    }
}
