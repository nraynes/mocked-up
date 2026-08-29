use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    database::Database,
    rest_service::{Response, request::Request, route::RouteMut},
};

pub struct RouteBuilderMut<
    F: Fn(Request, RefMut<Database>) -> Response = fn(Request, RefMut<Database>) -> Response,
> {
    db: Rc<RefCell<Database>>,
    routes: HashMap<String, Self>,
    f: Option<F>,
}

impl RouteBuilderMut<fn(Request, RefMut<Database>) -> Response> {
    pub fn new(
        db: Rc<RefCell<Database>>,
        f: Option<fn(Request, RefMut<Database>) -> Response>,
    ) -> Self {
        Self {
            db,
            routes: HashMap::new(),
            f,
        }
    }

    pub fn add<
        G: Fn(
            RouteBuilderMut<fn(Request, RefMut<Database>) -> Response>,
        ) -> RouteBuilderMut<fn(Request, RefMut<Database>) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request, RefMut<Database>) -> Response>,
        inner: G,
    ) -> Self {
        self.routes.insert(
            route.to_string(),
            inner(Self::new(Rc::clone(&self.db), call_method)),
        );
        self
    }

    pub fn build(self) -> RouteMut<fn(Request, RefMut<Database>) -> Response> {
        RouteMut::new(
            self.db,
            self.routes
                .into_iter()
                .map(|(k, v)| (k, v.build()))
                .collect(),
            self.f,
        )
    }
}
