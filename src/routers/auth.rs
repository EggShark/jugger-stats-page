use rusty_paseto::prelude::*;

use serde::{Deserialize, Serialize};

use crate::routers::auth::UserNameError::{IllegalCharacter, NonAscii};

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

const ILLEGAL_CHARACTERS: &[char] = &[
    '@', '!', '=', '&', '{', '$', '!', '\'', '`', ' ', '#', ':', '/', '\\',
];

pub fn username_validator(username: &str) -> Result<(), UserNameError> {
    let length = username.len();

    if length > 32 {
        return Err(UserNameError::Length(length));
    }

    if !username.is_ascii() {
        return Err(NonAscii);
    }

    for uc in username.chars() {
        for &c in ILLEGAL_CHARACTERS {
            if uc == c {
                return Err(IllegalCharacter(c));
            }
        }
    }

    Ok(())
}
