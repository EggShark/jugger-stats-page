use std::convert::Infallible;

use axum::Extension;
use axum::extract::{FromRequestParts, OptionalFromRequestParts, State};
use axum::response::Response;
use axum::{
    extract::Request,
    http::{StatusCode, request},
    middleware::Next,
};
use rusty_paseto::prelude::*;

use serde::{Deserialize, Serialize};

use crate::cookies::Cookies;
use crate::routers::auth::UserNameError::{IllegalCharacter, NonAscii};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaim {
    pub username: String,
    // other details
}

impl OptionalFromRequestParts<AppState> for UserClaim {
    type Rejection = Infallible;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        // can't fail
        let cookies = Cookies::from_request_parts(parts, state).await.unwrap();

        let value = cookies
            .get_cookie("session")
            // maybe need to handle errors but like if its not a valid key then like you aren't logged in
            .and_then(|token| {
                PasetoParser::<V4, Local>::default()
                    .parse_into::<UserClaim>(token, &state.paseto)
                    .ok()
            });

        Ok(value)
    }
}

pub fn create_token(username: &str, key: &PasetoSymmetricKey<V4, Local>) -> String {
    PasetoBuilder::<V4, Local>::default()
        .set_claim(CustomClaim::new("username", username).unwrap())
        .build(key)
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct BasicAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub enum UserNameError {
    Length(usize),
    IllegalCharacter(char),
    NonAscii,
}

const ALLOWED_CHARACTERS: &[char] = &['-', '_', '+', '~', '.', '<', '>', '^', '%', ';'];

pub fn username_validator(username: &str) -> Result<(), UserNameError> {
    let length = username.len();

    if length > 32 {
        return Err(UserNameError::Length(length));
    }

    if !username.is_ascii() {
        return Err(NonAscii);
    }

    for uc in username.chars() {
        if !uc.is_ascii_alphanumeric() && !ALLOWED_CHARACTERS.contains(&uc) {
            return Err(IllegalCharacter(uc));
        }
    }

    Ok(())
}
