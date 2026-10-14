mod routers;
mod state;

use std::env;
use std::sync::Arc;

use argon2::Argon2;
use routers::create_routers;
use rusty_paseto::prelude::*;
use sqlx::PgPool;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let conn_str = env::var("DATABASE_URL").unwrap();
    let paseto_secret = env::var("PASTEO_SECRET").unwrap();

    let paseto_secret_key: &[u8; 32] = paseto_secret.as_bytes().try_into().unwrap();
    let db_conn = PgPool::connect(&conn_str).await.unwrap();
    let argon2 = Argon2::default();
    let key = Arc::new(PasetoSymmetricKey::<V4, Local>::from(Key::from(
        paseto_secret_key,
    )));

    let state = AppState {
        db_conn,
        argon: argon2,
        paseto: key,
    };

    let app = create_routers(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listenting on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}