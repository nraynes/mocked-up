use std::collections::HashMap;

use derive_new::new;

use crate::{
    if_dev,
    rest_service::{Status, request::Request, response::Response},
};

#[derive(new, PartialEq, Debug)]
pub struct Route<F: Fn(Request) -> Response> {
    routes: HashMap<String, Self>,
    f: Option<F>,
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
        match &self.f {
            Some(call_method) => (call_method)(request),
            None => Response::new(String::new(), Status::NotImplemented),
        }
    }

    if_dev! {
        pub fn is_empty(&self) -> bool {
            self.routes.is_empty()
        }
    }
}
