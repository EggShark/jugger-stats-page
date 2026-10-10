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
        // static file serving
        .route_service("/", ServeFile::new("assets/html/index.html"))
        .nest_service("/css", ServeDir::new("assets/css"))
        .nest_service("/ts", ServeDir::new("assets/ts"))
        .nest_service("/others", ServeDir::new("assets/others"))
        .route("/{path}", get(serve_html_file));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listenting on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// serves static HTML files from assets/html
async fn serve_html_file(Path(file_path): Path<String>) -> impl IntoResponse {
   let mut path = PathBuf::from("assets/html");
   path.push(file_path);
   path.add_extension("html");
   let data = match fs::read_to_string(&path) {
       Ok(v) => v,
       Err(e) => {
           println!("path is: {:?}", path);
           println!("error is: {:?}", e);
           return (StatusCode::NOT_FOUND, Html("404".to_string()))
       },
   };


   (StatusCode::OK, Html(data))
}
