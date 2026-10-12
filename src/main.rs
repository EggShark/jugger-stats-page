mod routers;
mod state;

use std::sync::Arc;

use routers::create_routers;

use sqlx::{Connection, PgConnection, PgPool};

use crate::state::AppState;

// psql connstring
const DB_CONN_STR: &str = include_str!("../DBSECRET");

#[tokio::main]
async fn main() {
    // build our application with a single route
    let db_conn = PgPool::connect(DB_CONN_STR).await.unwrap();

    let state = AppState { db_conn };

    let app = create_routers(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listenting on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
