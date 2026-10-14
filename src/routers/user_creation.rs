use crate::AppState;

use rusty_paseto::prelude::*;
use sqlx::Error::{self as SqlError, Database};

use argon2::PasswordHasher;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::error::ErrorKind;
use sqlx::postgres::PgDatabaseError;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
enum CreateUserResponse {
    // send pasteo String token
    Token(String),
    InternalServerError,
    UserExists,
    // send db message
    OtherDb(String),
}

pub(super) async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
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

// move key to app state?
fn create_token(username: &str, key: &PasetoSymmetricKey<V4, Local>) -> String {
    PasetoBuilder::<V4, Local>::default()
        .set_claim(CustomClaim::new("username", username).unwrap())
        .build(key)
        .unwrap()
}