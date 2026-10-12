use crate::AppState;

use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    username: String,
    password: String,
}
// do stuff lmao

pub(super) async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    //dbg!(payload);

    // HASH PASSWORD
    // RETURN COOKIE?
    match sqlx::query!(
        "INSERT INTO users VALUES ($1, $2)",
        payload.username,
        payload.password
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
