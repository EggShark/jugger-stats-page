use argon2::PasswordVerifier;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use sqlx::Error as SqlError;

use crate::routers::auth::{BasicAuthRequest, create_token};

use crate::state::AppState;

pub(super) async fn sign_in(
    State(state): State<AppState>,
    Json(data): Json<BasicAuthRequest>,
) -> impl IntoResponse {
    let stored_password = match sqlx::query!(
        "SELECT password FROM users WHERE username = $1",
        &data.username
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v.password,
        Err(SqlError::RowNotFound) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // get 404 from idk

    let correct_pass = state
        .argon
        .verify_password(data.password.as_bytes(), stored_password.as_str())
        .is_ok();

    if !correct_pass {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(&data.username, &state.paseto);
    Ok(token)
}
