use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

// middleware
pub async fn parse_cookies(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let cookies = if let Some(cookies) = headers.get("Cookies").map(|v| v.to_str().ok()).flatten() {
        Cookies::from_cookie_str(cookies)
    } else {
        Cookies::new()
    };

    req.extensions_mut().insert(cookies);

    Ok(next.run(req).await)
}

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
            let mut values = line.split("=");

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

            cookies.insert(name.to_string(), value.to_string());
        }

        Self {
            cookies: Arc::new(cookies),
        }
    }

    pub fn get_cookie(&self, value: &str) -> Option<&String> {
        self.cookies.get(value)
    }
}
