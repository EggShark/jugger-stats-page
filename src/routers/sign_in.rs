use argon2::{PasswordHasher, PasswordVerifier};
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{extract::State, response::IntoResponse};
use sqlx::Error as SqlError;

use crate::routers::auth::{BasicAuthRequest, create_token};

use crate::state::AppState;

pub(super) async fn sign_in(
    State(state): State<AppState>,
    Json(data): Json<BasicAuthRequest>,
) -> Response {
    let stored_password = match sqlx::query!(
        "SELECT password FROM users WHERE username = $1",
        &data.username
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v.password,
        Err(SqlError::RowNotFound) => {
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    // get 404 from idk

    let correct_pass = state
        .argon
        .verify_password(data.password.as_bytes(), stored_password.as_str())
        .is_ok();

    if correct_pass {
        let token = create_token(&data.username, &state.paseto);
        (StatusCode::OK, token).into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
