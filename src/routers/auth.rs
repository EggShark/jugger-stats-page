use rusty_paseto::prelude::*;

use serde::Deserialize;

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
