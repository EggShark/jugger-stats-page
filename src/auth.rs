use rusty_paseto::core::{Key, Local, PasetoSymmetricKey, V4};
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SignInClaims {
    username: String,
}