use rusty_paseto::core::{Key, Local, PasetoSymmetricKey, V4};
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};

pub const PASTEO_SECRET: &[u8; 32] = include_bytes!("../PASTEOSECRET");

#[derive(Deserialize, Serialize)]
struct SignInClaims {
    username: String,
}
