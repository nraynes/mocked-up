use std::collections::HashMap;

use crate::rest_service::{Response, request::Request, route::Route};

pub struct RouteBuilder<F: Fn(Request) -> Response = fn(Request) -> Response> {
    routes: HashMap<String, Self>,
    f: Option<F>,
}

impl RouteBuilder<fn(Request) -> Response> {
    pub fn new(f: Option<fn(Request) -> Response>) -> Self {
        Self {
            routes: HashMap::new(),
            f: f,
        }
    }

    pub fn add<
        G: Fn(RouteBuilder<fn(Request) -> Response>) -> RouteBuilder<fn(Request) -> Response>,
    >(
        mut self,
        route: &str,
        call_method: Option<fn(Request) -> Response>,
        inner: G,
    ) -> Self {
        self.routes
            .insert(route.to_string(), inner(RouteBuilder::new(call_method)));
        self
    }

    pub fn build(self) -> Route<fn(Request) -> Response> {
        Route::new(
            self.routes
                .into_iter()
                .map(|(k, v)| (k, v.build()))
                .collect(),
            self.f,
        )
    }
}

#[cfg(test)]
mod test {
    use serde_json::{Map, Value, json};

    use crate::rest_service::Status;

    use super::*;

    /// This one test hits most of the code for the rest service, including testing requests and responses,
    /// routes and the find algorithm for routes, making calls from routes, serialization and deserialization,
    /// and the builder itself.
    #[test]
    fn test_build_route() {
        let route = RouteBuilder::new(None)
            .add("one", None, |b| {
                b.add(
                    "one_one",
                    Some(|_| Response::new(String::new(), Status::BadRequest)),
                    |b| b,
                )
                .add("one_two", None, |b| {
                    b.add(
                        "one_two_one",
                        Some(|r| {
                            if let Ok(body) = r
                                .deserialize_body(|s| serde_json::from_str::<Map<String, Value>>(s))
                                && let Some(test_value) = body.get("test_key")
                                && let Some(test_string) = test_value.as_str()
                                && test_string == "This is a test."
                            {
                                let json_val = json!({
                                    "response_key": "Response Value!",
                                });
                                let val = json_val.to_string();
                                return Response::new(val, Status::Ok);
                            }
                            Response::new(String::new(), Status::NoContent)
                        }),
                        |b| b,
                    )
                })
            })
            .add(
                "two",
                Some(|_| Response::new(String::new(), Status::InternalServerError)),
                |b| b,
            )
            .add(
                "three",
                Some(|_| Response::new(String::new(), Status::NotFound)),
                |b| b,
            )
            .build();

        // Should be some. Test responses also.
        assert!(route.find(["one"]).is_some());
        assert!(route.find(["one", "one_one"]).is_some_and(|endpoint| {
            endpoint
                .call(Request::new(HashMap::new(), String::new(), HashMap::new()))
                .status()
                == &Status::BadRequest
                && endpoint.is_empty()
        }));
        assert!(route.find(["one", "one_two"]).is_some_and(|endpoint| {
            endpoint
                .call(Request::new(HashMap::new(), String::new(), HashMap::new()))
                .status()
                == &Status::NotImplemented
        }));
        assert!(
            route
                .find(["one", "one_two", "one_two_one"])
                .is_some_and(|endpoint| {
                    endpoint
                        .call(Request::new(HashMap::new(), String::new(), HashMap::new()))
                        .status()
                        == &Status::NoContent
                        && endpoint.is_empty()
                })
        );

        let implemented_route = route.find(["one", "one_two", "one_two_one"]).unwrap();
        let response = implemented_route.call(Request::new(
            HashMap::new(),
            serde_json::to_string(&json!({
                "test_key": "This is a test.",
            }))
            .unwrap(),
            HashMap::new(),
        ));
        let data = response
            .deserialize_data(|s| serde_json::from_str::<Map<String, Value>>(s))
            .unwrap();
        let response_value = data.get("response_key").unwrap();
        let response_str = response_value.as_str().unwrap();
        assert_eq!(response_str, "Response Value!");
        assert_eq!(response.status(), &Status::Ok);
        assert!(route.find(["two"]).is_some_and(|endpoint| {
            endpoint
                .call(Request::new(HashMap::new(), String::new(), HashMap::new()))
                .status()
                == &Status::InternalServerError
                && endpoint.is_empty()
        }));
        assert!(route.find(["three"]).is_some_and(|endpoint| {
            endpoint
                .call(Request::new(HashMap::new(), String::new(), HashMap::new()))
                .status()
                == &Status::NotFound
                && endpoint.is_empty()
        }));

        // Should be none.
        assert!(route.find(["four"]).is_none());
        assert!(route.find(["one", "seven"]).is_none());
        assert!(route.find(["one", "one_one", "twenty"]).is_none());
        assert!(route.find(["seven", "five"]).is_none());
        assert!(route.find(["eight", "eleven", "two"]).is_none());
        assert!(route.find(["nine", "one_one"]).is_none());
    }
}
