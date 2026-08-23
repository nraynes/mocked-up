use std::{collections::HashMap, str::FromStr};

use derive_getters::Getters;

use crate::MockError;

#[derive(Getters)]
pub struct Url {
    protocol: String,
    domain_segments: Vec<String>,
    port_segment: Option<u32>,
    route_segments: Vec<String>,
    query_params: HashMap<String, String>,
}

impl FromStr for Url {
    type Err = MockError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (protocol, url_segment) = s
            .split_once("://")
            .ok_or("Url not valid. Cannot separate protocol from domain.")?;

        // Ensure protocol is not empty.
        if protocol.is_empty() {
            return Err(MockError::from("Protocol must not be empty."));
        }

        // Extract query params.
        let mut query_params = HashMap::new();
        let url_segment = if let Some((url_segment, queries)) = url_segment.split_once("?") {
            for individual_query in queries.split("&").map(|x| x.split_once("=")) {
                let (name, value) = individual_query.ok_or("Invalid query syntax.")?;
                if !name.is_empty() && !value.is_empty() {
                    query_params.insert(name.to_string(), value.to_string());
                } else {
                    return Err(MockError::from("Query params must not be empty."));
                }
            }
            url_segment
        } else {
            url_segment
        };

        // Extract routes.
        let mut route_segments: Vec<String> = Vec::new();
        let url_segment = if let Some((url_segment, routes)) = url_segment.split_once("/") {
            route_segments = routes
                .split("/")
                .filter(|x| *x != "")
                .map(|x| x.to_string())
                .collect();
            url_segment
        } else {
            url_segment
        };

        // Extract port.
        let mut port_segment = None;
        let domain = if let Some((url_segment, port)) = url_segment.split_once(":") {
            port_segment = Some(u32::from_str(port)?);
            url_segment
        } else {
            url_segment
        };

        // Separate sub-domains.
        let domain_segments: Vec<String> = domain.split(".").map(|x| x.to_string()).collect();

        // Ensure at least one domain value exists.
        if domain_segments.is_empty() {
            return Err(MockError::from("Domain must not be empty."));
        }

        Ok(Self {
            protocol: protocol.to_string(),
            domain_segments,
            port_segment,
            route_segments,
            query_params,
        })
    }
}

impl ToString for Url {
    fn to_string(&self) -> String {
        let mut url = self.base();

        // Add routes.
        let route = self.route_segments.join("/");
        if !route.is_empty() {
            url = format!("{}/{}", url, route);
        }

        // Add query string.
        let mut query_vec = self
            .query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>();
        query_vec.sort();
        let query: String = query_vec.join("&");
        if !query.is_empty() {
            url = format!("{}?{}", url, query);
        } else {
            url = format!("{}/", url);
        }

        url
    }
}

impl Url {
    pub fn base(&self) -> String {
        let mut domain = self.domain_segments.join(".");
        if let Some(port) = self.port_segment {
            domain = format!("{}:{}", domain, port);
        }
        format!("{}://{}", self.protocol, domain)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_url_working() {
        let small_url = Url::from_str("http://www.something.com/").unwrap();
        assert_eq!(small_url.base(), "http://www.something.com");
        assert!(small_url.port_segment().is_none());
        assert_eq!(small_url.protocol(), "http");
        assert!(small_url.query_params().is_empty());
        assert!(small_url.route_segments().is_empty());
        assert_eq!(small_url.to_string(), "http://www.something.com/");

        let long_url = Url::from_str("http://www.something.com/login/page").unwrap();
        assert_eq!(long_url.base(), "http://www.something.com");
        assert!(long_url.port_segment().is_none());
        assert_eq!(long_url.protocol(), "http");
        assert!(long_url.query_params().is_empty());
        assert_eq!(long_url.route_segments(), &vec!["login", "page"]);
        assert_eq!(long_url.to_string(), "http://www.something.com/login/page/");

        let local_url = Url::from_str("sql://127.0.0.1:8080/").unwrap();
        assert_eq!(local_url.base(), "sql://127.0.0.1:8080");
        assert_eq!(local_url.port_segment(), &Some(8080));
        assert_eq!(local_url.protocol(), "sql");
        assert!(local_url.query_params().is_empty());
        assert!(local_url.route_segments().is_empty());
        assert_eq!(local_url.to_string(), "sql://127.0.0.1:8080/");

        let long_url_w_queries =
            Url::from_str("http://www.something.com/login/page?one_param=899&two_param=test_value")
                .unwrap();
        assert_eq!(long_url_w_queries.base(), "http://www.something.com");
        assert!(long_url_w_queries.port_segment().is_none());
        assert_eq!(long_url_w_queries.protocol(), "http");
        assert_eq!(
            long_url_w_queries.query_params(),
            &HashMap::from_iter([
                ("one_param".into(), "899".into()),
                ("two_param".into(), "test_value".into()),
            ])
        );
        assert_eq!(long_url_w_queries.route_segments(), &vec!["login", "page"]);
        assert_eq!(
            long_url_w_queries.to_string(),
            "http://www.something.com/login/page?one_param=899&two_param=test_value"
        );

        let full_url =
            Url::from_str("sql://127.0.0.1:8080/login/page?one_param=899&two_param=test_value")
                .unwrap();
        assert_eq!(full_url.base(), "sql://127.0.0.1:8080");
        assert_eq!(full_url.port_segment(), &Some(8080));
        assert_eq!(full_url.protocol(), "sql");
        assert_eq!(
            full_url.query_params(),
            &HashMap::from_iter([
                ("one_param".into(), "899".into()),
                ("two_param".into(), "test_value".into()),
            ])
        );
        assert_eq!(full_url.route_segments(), &vec!["login", "page"]);
        assert_eq!(
            full_url.to_string(),
            "sql://127.0.0.1:8080/login/page?one_param=899&two_param=test_value"
        );
    }

    #[test]
    fn test_url_not_working() {
        assert!(Url::from_str("The quick brown fox.").is_err());
        assert!(Url::from_str("http://www.something.com:notaport").is_err());
        assert!(Url::from_str("httpwww.something.com").is_err());
        assert!(Url::from_str("1234567890").is_err());
        assert!(Url::from_str("://").is_err());
        assert!(Url::from_str("").is_err());
        assert!(Url::from_str("The quick brown ?lll=uu&this=isntfox.").is_err());
    }
}
