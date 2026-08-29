use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use derive_new::new;

use crate::{
    database::Database,
    if_dev,
    rest_service::{Status, request::Request, response::Response},
};

#[derive(new, PartialEq, Debug)]
pub struct RouteMut<
    F: Fn(Request, RefMut<Database>) -> Response = fn(Request, RefMut<Database>) -> Response,
> {
    db: Rc<RefCell<Database>>,
    routes: HashMap<String, Self>,
    f: Option<F>,
}

impl RouteMut {
    pub fn find_mut<U: AsRef<str>, I: IntoIterator<Item = U>>(
        &mut self,
        route: I,
    ) -> Option<&mut Self> {
        let mut route_iter = route.into_iter();
        match route_iter.next() {
            Some(next_route) => self
                .routes
                .get_mut(next_route.as_ref())
                .and_then(|x| x.find_mut(route_iter)),
            None => Some(self),
        }
    }

    pub fn call(&mut self, request: Request) -> Response {
        match &self.f {
            Some(call_method) => (call_method)(
                request,
                match self.db.try_borrow_mut() {
                    Ok(db) => db,
                    Err(e) => return Response::new(e.to_string(), Status::InternalServerError),
                },
            ),
            None => Response::new(String::new(), Status::NotImplemented),
        }
    }

    if_dev! {
        pub fn is_empty(&self) -> bool {
            self.routes.is_empty()
        }
    }
}
