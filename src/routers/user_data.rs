use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{routers::auth::UserClaim, state::AppState};

#[derive(Deserialize)]
pub struct BasicDataRequest {
    username: String,
}

#[derive(Serialize)]
pub struct BasicDataResponse {
    description: String,
    owner_of_page: bool,
    // other quick stats
}

pub async fn get_basic_user_data(
    State(state): State<AppState>,
    Extension(claims): Extension<Option<UserClaim>>,
    Json(payload): Json<BasicDataRequest>,
) -> impl IntoResponse {
    let data = match sqlx::query!(
        "SELECT description FROM users WHERE username = $1",
        &payload.username
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = BasicDataResponse {
        description: data.description.unwrap_or_default(),
        owner_of_page: claims
            .map(|v| v.username == payload.username)
            .unwrap_or(false),
    };

    Ok(Json(response))
}
