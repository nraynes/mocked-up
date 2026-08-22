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

        // Extract query params.
        let mut query_params = HashMap::new();
        if let Some((url_segment, queries)) = url_segment.split_once("?") {
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
            port_segment = u32::from_str(port).ok();
            url_segment
        } else {
            url_segment
        };

        // Separate sub-domains.
        let domain_segments: Vec<String> = domain.split(".").map(|x| x.to_string()).collect();

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
        let query: String = self
            .query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
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
