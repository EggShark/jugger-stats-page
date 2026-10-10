use std::fs;
use std::path::PathBuf;

use axum::Router;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::method_routing::get;
use axum::http::StatusCode;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest_service("/css", ServeDir::new("assets/css"))
        .nest_service("/ts", ServeDir::new("assets/ts"))
        .nest_service("/others", ServeDir::new("assets/others"))
        .fallback_service(ServeDir::new("assets/html").html_as_default_extension(true));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listenting on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
