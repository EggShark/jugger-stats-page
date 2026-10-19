mod auth;
mod sign_in;
mod user_creation;

use axum::{Router, routing::post};
use tower_http::services::ServeDir;

use crate::{routers::sign_in::sign_in, state::AppState};
use user_creation::create_user;

pub fn create_routers(state: AppState) -> Router {
    Router::new()
        .nest_service("/css", ServeDir::new("assets/css"))
        .nest_service("/js", ServeDir::new("assets/js"))
        .nest_service("/others", ServeDir::new("assets/others"))
        .fallback_service(ServeDir::new("assets/html").html_as_default_extension(true))
        .route("/signin", post(sign_in))
        .route("/create_user", post(create_user))
        .with_state(state)
}
