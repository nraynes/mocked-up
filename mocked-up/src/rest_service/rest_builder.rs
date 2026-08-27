use std::{collections::HashMap, rc::Rc, str::FromStr};

use crate::{
    MockError,
    database::Database,
    rest_service::{
        RestService, request::Request, response::Response, route::Route, route::RouteBuilder,
        url::Url,
    },
};

pub struct RestBuilder<F: Fn(Request, &Database) -> Response = fn(Request, &Database) -> Response> {
    base_url: Url,
    get_routes: HashMap<String, Route<F>>,
    post_routes: HashMap<String, Route<F>>,
    patch_routes: HashMap<String, Route<F>>,
    put_routes: HashMap<String, Route<F>>,
    delete_routes: HashMap<String, Route<F>>,
    database: Rc<Database>,
}

impl RestBuilder<fn(Request, &Database) -> Response> {
    pub fn new(base_url: &str) -> Result<Self, MockError> {
        Ok(Self {
            base_url: Url::from_str(base_url)?,
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
            patch_routes: HashMap::new(),
            put_routes: HashMap::new(),
            delete_routes: HashMap::new(),
            database: Rc::new(Database::new()),
        })
    }

    pub fn get<
        G: Fn(
            RouteBuilder<fn(Request, &Database) -> Response>,
        ) -> Route<fn(Request, &Database) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, &Database) -> Response>,
        inner: G,
    ) -> Self {
        self.get_routes.insert(
            route.to_string(),
            inner(RouteBuilder::new(Rc::clone(&self.database), call_method)),
        );
        self
    }

    pub fn post<
        G: Fn(
            RouteBuilder<fn(Request, &Database) -> Response>,
        ) -> Route<fn(Request, &Database) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, &Database) -> Response>,
        inner: G,
    ) -> Self {
        self.post_routes.insert(
            route.to_string(),
            inner(RouteBuilder::new(Rc::clone(&self.database), call_method)),
        );
        self
    }

    pub fn patch<
        G: Fn(
            RouteBuilder<fn(Request, &Database) -> Response>,
        ) -> Route<fn(Request, &Database) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, &Database) -> Response>,
        inner: G,
    ) -> Self {
        self.patch_routes.insert(
            route.to_string(),
            inner(RouteBuilder::new(Rc::clone(&self.database), call_method)),
        );
        self
    }

    pub fn put<
        G: Fn(
            RouteBuilder<fn(Request, &Database) -> Response>,
        ) -> Route<fn(Request, &Database) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, &Database) -> Response>,
        inner: G,
    ) -> Self {
        self.put_routes.insert(
            route.to_string(),
            inner(RouteBuilder::new(Rc::clone(&self.database), call_method)),
        );
        self
    }

    pub fn delete<
        G: Fn(
            RouteBuilder<fn(Request, &Database) -> Response>,
        ) -> Route<fn(Request, &Database) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, &Database) -> Response>,
        inner: G,
    ) -> Self {
        self.delete_routes.insert(
            route.to_string(),
            inner(RouteBuilder::new(Rc::clone(&self.database), call_method)),
        );
        self
    }

    pub fn build(self) -> RestService<fn(Request, &Database) -> Response> {
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
