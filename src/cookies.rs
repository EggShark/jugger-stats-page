use std::sync::Arc;
use std::{collections::HashMap, convert::Infallible};

use axum::extract::FromRequestParts;

use crate::state::AppState;

#[derive(Clone)]
pub struct Cookies {
    cookies: Arc<HashMap<String, String>>,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: Arc::new(HashMap::new()),
        }
    }

    // fails quietly
    pub fn from_cookie_str(line: &str) -> Self {
        let mut cookies = HashMap::new();

        let pairs = line.split(";");
        for p in pairs {
            let mut values = p.split("=");

            let Some(name) = values.next() else {
                continue;
            };

            let Some(value) = values.next() else {
                continue;
            };

            // three things in a cookie in consevable
            if values.next().is_some() {
                continue;
            }

            cookies.insert(name.trim().to_string(), value.trim().to_string());
        }

        Self {
            cookies: Arc::new(cookies),
        }
    }

    pub fn get_cookie(&self, value: &str) -> Option<&String> {
        self.cookies.get(value)
    }
}

impl FromRequestParts<AppState> for Cookies {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &AppState,
    ) -> Result<Cookies, Self::Rejection> {
        let cookies = if let Some(cookies) = parts
            .headers
            .get("cookie")
            .map(|v| v.to_str().ok())
            .flatten()
        {
            Cookies::from_cookie_str(cookies)
        } else {
            Cookies::new()
        };

        Ok(cookies)
    }
}
