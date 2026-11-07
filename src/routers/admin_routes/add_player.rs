use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::Error as SqlError;

use crate::{routers::auth::UserClaim, state::AppState};

#[derive(Deserialize)]
pub struct AddPlayerReq {
    player_name: String,
    // wrap into inner struct?
    player_team_id: Option<i32>,
    player_jersey_num: Option<String>,
    player_jersey_name: Option<String>,
}

const PLAYER_NAME_LIMIT: usize = 50;

// no need user claims we know you are an admin rn
pub async fn add_player(
    State(state): State<AppState>,
    Json(data): Json<AddPlayerReq>,
) -> impl IntoResponse {
    // import into db
    if data.player_name.len() > PLAYER_NAME_LIMIT {
        return Err(StatusCode::BAD_REQUEST);
    }

    let player_id = match sqlx::query!(
        "INSERT INTO players (name) VALUES ($1) RETURNING player_id",
        data.player_name
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v.player_id,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let (player_team, player_jersey_num, player_jersey_name) = match (
        data.player_team_id,
        data.player_jersey_num,
        data.player_jersey_name,
    ) {
        (Some(t), Some(num), Some(name)) => (t, num, name),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match sqlx::query!(
        "INSERT INTO leauge_registrations VALUES (current_date, $1, $2, $3, $4)",
        player_jersey_num,
        player_jersey_name,
        player_id,
        player_team
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(SqlError::InvalidArgument(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
