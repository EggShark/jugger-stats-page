use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{routers::auth::UserClaim, state::AppState};

#[derive(Serialize, Debug)]
pub struct BasicDataResponse {
    description: String,
    owner_of_page: bool,
    // other quick stats
}

pub async fn get_basic_user_data(
    State(state): State<AppState>,
    claims: Option<UserClaim>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let data = match sqlx::query!(
        "SELECT description FROM users WHERE username = $1",
        &username
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            println!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("{:?}", claims);

    let response = BasicDataResponse {
        description: data.description.unwrap_or_default(),
        owner_of_page: claims.map(|v| v.username == username).unwrap_or(false),
    };

    Ok(Json(response))
}

#[derive(Deserialize, Debug)]
pub struct DescriptionChange {
    username: String,
    text: String,
}

pub async fn change_description(
    State(state): State<AppState>,
    claims: Option<UserClaim>,
    Json(payload): Json<DescriptionChange>,
) -> impl IntoResponse {
    if claims.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let claims = claims.unwrap();

    if claims.username != payload.username {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if payload.text.len() > 2000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query!(
        "UPDATE users SET description = $1 WHERE username = $2",
        payload.text,
        payload.username
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(v) => {
            if v.rows_affected() == 0 {
                return Err(StatusCode::UNAUTHORIZED);
            } else if v.rows_affected() != 1 {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(())
}
