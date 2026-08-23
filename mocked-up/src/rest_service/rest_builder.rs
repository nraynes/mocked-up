use std::{collections::HashMap, str::FromStr};

use crate::{
    MockError,
    rest_service::{
        RestService, RouteBuilder, request::Request, response::Response, route::Route, url::Url,
    },
};

pub struct RestBuilder<F: Fn(Request) -> Response = fn(Request) -> Response> {
    base_url: Url,
    get_routes: HashMap<String, Route<F>>,
    post_routes: HashMap<String, Route<F>>,
    patch_routes: HashMap<String, Route<F>>,
    put_routes: HashMap<String, Route<F>>,
    delete_routes: HashMap<String, Route<F>>,
}

impl RestBuilder<fn(Request) -> Response> {
    pub fn new(base_url: &str) -> Result<Self, MockError> {
        Ok(Self {
            base_url: Url::from_str(base_url)?,
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
            patch_routes: HashMap::new(),
            put_routes: HashMap::new(),
            delete_routes: HashMap::new(),
        })
    }

    pub fn get<G: Fn(RouteBuilder<fn(Request) -> Response>) -> Route<fn(Request) -> Response>>(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.get_routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn post<G: Fn(RouteBuilder<fn(Request) -> Response>) -> Route<fn(Request) -> Response>>(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.post_routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn patch<G: Fn(RouteBuilder<fn(Request) -> Response>) -> Route<fn(Request) -> Response>>(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.patch_routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn put<G: Fn(RouteBuilder<fn(Request) -> Response>) -> Route<fn(Request) -> Response>>(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.put_routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn delete<
        G: Fn(RouteBuilder<fn(Request) -> Response>) -> Route<fn(Request) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.delete_routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn build(self) -> RestService<fn(Request) -> Response> {
        RestService::new(
            self.base_url,
            self.get_routes,
            self.post_routes,
            self.patch_routes,
            self.put_routes,
            self.delete_routes,
        )
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_rest_builder() {
//         let service = RestBuilder::new("http://www.example.com")
//             .unwrap()
//             .get("app", None, |b| b
//                 .add("login", Some(|r| {

//                 }), |b| b))

//     }
// }
