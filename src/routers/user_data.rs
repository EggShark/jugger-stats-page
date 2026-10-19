use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
struct BasicDataRequest {
    username: String,
}

fn get_basic_user_data(
    State(state): State<AppState>,
    Json(payload): Json<BasicDataRequest>,
) -> impl IntoResponse {
    // request data
    ()
}
