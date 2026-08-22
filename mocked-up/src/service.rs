mod call;
mod request;
mod response;
mod route;
mod status;
mod url;

use derive_new::new;
use std::collections::HashMap;

use crate::{
    MockError,
    service::{request::Request, response::Response, route::Route, url::Url},
};

#[derive(new)]
pub struct Service<F: Fn(Request) -> Response> {
    base_url: Url,
    get_routes: HashMap<String, Route<F>>,
    post_routes: HashMap<String, Route<F>>,
    patch_routes: HashMap<String, Route<F>>,
    put_routes: HashMap<String, Route<F>>,
    delete_routes: HashMap<String, Route<F>>,
}

impl<F: Fn(Request) -> Response> Service<F> {
    fn find_in<U: AsRef<str>, I: IntoIterator<Item = U>>(
        routes: &HashMap<String, Route<F>>,
        route: I,
    ) -> Option<&Route<F>> {
        let mut route_iter = route.into_iter();
        route_iter.next().and_then(|next_route| {
            routes
                .get(next_route.as_ref())
                .and_then(|x| x.find(route_iter))
        })
    }

    fn extract_url<U: Into<Url>>(&self, url: U) -> Result<Url, MockError> {
        let url = url.into();
        if url.base() != self.base_url.base() {
            return Err(MockError::from("Url must match base url"));
        }
        Ok(url)
    }

    fn build_request<const N: usize, T: AsRef<[u8]>>(
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
            Vec::from(body.as_ref()),
            url.query_params().clone(),
        )
    }

    pub fn req<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
        from_routes: &HashMap<String, Route<F>>,
    ) -> Result<Response, MockError> {
        let url = self.extract_url(url)?;
        let request = self.build_request(&url, headers, body);
        let route = Self::find_in(from_routes, url.route_segments())
            .ok_or("No get method implemented for that route.")?;
        Ok(route.call(request))
    }

    pub fn get<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        self.req(url, headers, body, &self.get_routes)
    }

    pub fn post<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        self.req(url, headers, body, &self.post_routes)
    }

    pub fn patch<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        self.req(url, headers, body, &self.patch_routes)
    }

    pub fn put<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        self.req(url, headers, body, &self.put_routes)
    }

    pub fn delete<U: Into<Url>, const N: usize, T: AsRef<[u8]>>(
        &self,
        url: U,
        headers: [(&str, &str); N],
        body: T,
    ) -> Result<Response, MockError> {
        self.req(url, headers, body, &self.delete_routes)
    }
}
