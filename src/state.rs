use argon2::Argon2;
use rusty_paseto::core::{Local, PasetoSymmetricKey, V4};
use sqlx::PgPool;

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: PgPool,
    pub argon: Argon2<'static>,
    pub paseto: Arc<PasetoSymmetricKey<V4, Local>>,
}
