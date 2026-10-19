use crate::AppState;
use crate::routers::auth::{BasicAuthRequest, UserNameError, create_token, username_validator};

use sqlx::Error::Database;

use argon2::PasswordHasher;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::error::ErrorKind;

#[derive(Serialize)]
enum CreateUserResponse {
    // send pasteo String token
    Token(String),
    InternalServerError,
    UserExists,
    // send db message
    OtherDb(String),
    PasswordError(String),
    UserNameError(UserNameError),
}

pub(super) async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<BasicAuthRequest>,
) -> impl IntoResponse {
    // only restaint on password not sure if I really need any others
    if payload.password.len() > 128 || payload.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(CreateUserResponse::PasswordError(String::from(
                "Password length must be between 8 and 128",
            ))),
        );
    }

    match username_validator(&payload.username) {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CreateUserResponse::UserNameError(e)),
            );
        }
    }

    let hashed_password = state
        .argon
        .hash_password(payload.password.as_bytes())
        .unwrap()
        .to_string();

    // HASH PASSWORD
    // RETURN COOKIE?
    match sqlx::query!(
        "INSERT INTO users VALUES ($1, $2)",
        payload.username,
        hashed_password
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(_) => {
            let token = create_token(&payload.username, &state.paseto);
            (StatusCode::OK, Json(CreateUserResponse::Token(token)))
        }
        Err(Database(err)) => {
            println!("{:?}", err);
            let message = err.message();

            let res = if err.is_unique_violation() {
                CreateUserResponse::UserExists
            } else {
                CreateUserResponse::OtherDb(message.to_string())
            };

            (sqlx_error_to_response(err.kind()), Json(res))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateUserResponse::InternalServerError),
        ),
    }
}

fn sqlx_error_to_response(err: ErrorKind) -> StatusCode {
    match err {
        ErrorKind::UniqueViolation => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
