mod request;
mod response;
mod rest_builder;
mod route;
mod status;
mod url;

pub use request::Request;
pub use response::Response;
pub use rest_builder::RestBuilder;
pub use route::{Route, RouteBuilder};
pub use status::Status;

use derive_new::new;
use std::{
    cell::{Ref, RefMut},
    collections::HashMap,
    str::FromStr,
};

use crate::{
    MockError,
    database::Database,
    rest_service::{route::RouteMut, url::Url},
};

#[derive(new)]
pub struct RestService<
    F: Fn(Request, Ref<Database>) -> Response = fn(Request, Ref<Database>) -> Response,
    M: Fn(Request, RefMut<Database>) -> Response = fn(Request, RefMut<Database>) -> Response,
> {
    base_url: Url,
    get_routes: HashMap<String, Route<F>>,
    post_routes: HashMap<String, RouteMut<M>>,
    patch_routes: HashMap<String, RouteMut<M>>,
    put_routes: HashMap<String, RouteMut<M>>,
    delete_routes: HashMap<String, RouteMut<M>>,
}

impl RestService {
    fn find_in<U: AsRef<str>, I: IntoIterator<Item = U>>(
        routes: &HashMap<String, Route>,
        route: I,
    ) -> Option<&Route> {
        let mut route_iter = route.into_iter();
        route_iter.next().and_then(|next_route| {
            routes
                .get(next_route.as_ref())
                .and_then(|x| x.find(route_iter))
        })
    }

    fn find_in_mut<U: AsRef<str>, I: IntoIterator<Item = U>>(
        routes: &mut HashMap<String, RouteMut>,
        route: I,
    ) -> Option<&mut RouteMut> {
        let mut route_iter = route.into_iter();
        route_iter.next().and_then(|next_route| {
            routes
                .get_mut(next_route.as_ref())
                .and_then(|x| x.find_mut(route_iter))
        })
    }

    fn extract_url(&self, url: &str) -> Result<Url, MockError> {
        let url = Url::from_str(url)?;
        if url.base() != self.base_url.base() {
            return Err(MockError::from("Url must match base url"));
        }
        Ok(url)
    }

    fn build_request<const N: usize, T: AsRef<str>>(
        &self,
        url: &Url,
        headers: [(&str, &str); N],
        body: T,
    ) -> Request {
        let headers_map_iter = headers
            .into_iter()
            .map(|x| (x.0.to_string(), x.1.to_string()));
        Request::new(
            HashMap::from_iter(headers_map_iter),
            body.as_ref().to_string().clone(),
            url.query_params().clone(),
        )
    }

    pub fn get<const N: usize, T: AsRef<str>>(
        &self,
        url: &str,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in(&self.get_routes, url.route_segments())
            .ok_or("No method implemented for that route.")?;
        Ok(route.call(request))
    }

    pub fn post<const N: usize, T: AsRef<str>>(
        &mut self,
        url: &str,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in_mut(&mut self.post_routes, url.route_segments())
            .ok_or("No method implemented for that route.")?;
        Ok(route.call(request))
    }

    pub fn patch<const N: usize, T: AsRef<str>>(
        &mut self,
        url: &str,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in_mut(&mut self.patch_routes, url.route_segments())
            .ok_or("No method implemented for that route.")?;
        Ok(route.call(request))
    }

    pub fn put<const N: usize, T: AsRef<str>>(
        &mut self,
        url: &str,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in_mut(&mut self.put_routes, url.route_segments())
            .ok_or("No method implemented for that route.")?;
        Ok(route.call(request))
    }

    pub fn delete<const N: usize, T: AsRef<str>>(
        &mut self,
        url: &str,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in_mut(&mut self.delete_routes, url.route_segments())
            .ok_or("No method implemented for that route.")?;
        Ok(route.call(request))
    }
}

#[cfg(test)]
mod test {
    use crate::rest_service::Status;

    use super::*;

    #[test]
    fn test_rest_service_and_builder() {
        let db = Database::new();
        let mut service = RestBuilder::new("http://www.example.com", db)
            .unwrap()
            .get("app", None, |b| {
                b.add(
                    "search",
                    Some(|r, _| {
                        if let Some(table) = r.query().get("table")
                            && let Some(search_phrase) = r.query().get("search_phrase")
                        {
                            return Response::new(
                                format!(
                                    "The table selected was {} and the search phrase is {}.",
                                    table, search_phrase
                                ),
                                Status::Ok,
                            );
                        }
                        Response::no_data(Status::BadRequest)
                    }),
                    |b| b,
                )
                .build()
            })
            .post("app", None, |b| {
                b.add(
                    "data",
                    Some(|r, _| {
                        let data = r.body();
                        Response::new(format!("The data given was {}", data), Status::Accepted)
                    }),
                    |b| b,
                )
                .build()
            })
            .build();

        let get_response_valid = service
            .get(
                "http://www.example.com/app/search?table=users&search_phrase=some_search",
                [],
                "",
            )
            .unwrap();

        assert_eq!(get_response_valid.status(), &Status::Ok);
        assert_eq!(
            get_response_valid.data(),
            "The table selected was users and the search phrase is some_search."
        );

        let get_response_invalid = service
            .get("http://www.example.com/app/search", [], "")
            .unwrap();

        assert_eq!(get_response_invalid.status(), &Status::BadRequest);
        assert_eq!(get_response_invalid.data(), "");

        let post_response_valid = service
            .post("http://www.example.com/app/data", [], "Some Data")
            .unwrap();

        assert_eq!(post_response_valid.status(), &Status::Accepted);
        assert_eq!(post_response_valid.data(), "The data given was Some Data");

        let post_response_invalid = service.post("http://www.example.com/app/", [], "").unwrap();

        assert_eq!(post_response_invalid.status(), &Status::NotImplemented);
        assert_eq!(post_response_invalid.data(), "");

        let put_response_invalid = service.put("http://www.example.com/app/", [], "");

        assert!(put_response_invalid.is_err());
    }
}
