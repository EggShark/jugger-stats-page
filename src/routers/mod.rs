mod auth;
mod sign_in;
mod user_creation;
mod user_data;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    routers::{sign_in::sign_in, user_data::change_description},
    state::AppState,
};
use user_creation::create_user;
use user_data::get_basic_user_data;
pub fn create_routers(state: AppState) -> Router {
    Router::new()
        .nest_service("/css", ServeDir::new("assets/css"))
        .nest_service("/js", ServeDir::new("assets/js"))
        .nest_service("/others", ServeDir::new("assets/others"))
        // these are gonna be client side rendered html files
        .nest_service("/user/{id}", ServeFile::new("assets/html/user_page.html"))
        .nest_service("/u/{id}", ServeFile::new("assets/html/user_page.html"))
        .fallback_service(ServeDir::new("assets/html").html_as_default_extension(true))
        .route("/signin", post(sign_in))
        .route("/create_user", post(create_user))
        .route("/change_user_description", post(change_description))
        .route("/basic_data/{username}", get(get_basic_user_data))
        .with_state(state)
}
