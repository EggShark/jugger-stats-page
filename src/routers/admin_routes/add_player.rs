use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::{Error as SqlError, postgres::PgDatabaseError};

use crate::{routers::auth::UserClaim, state::AppState};

#[derive(Deserialize)]
pub struct AddPlayerReq {
    player_name: String,
    player_team_info: Option<PlayerTeamInfo>,
}

#[derive(Deserialize)]
pub struct PlayerTeamInfo {
    player_team_id: i32,
    player_jersey_num: String,
    player_jersey_name: String,
}

const PLAYER_NAME_LIMIT: usize = 50;
const JERSEY_NAME_LIMIT: usize = 32;
const JERSEY_NUMBER_LIMIT: usize = 4;

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

    if data.player_team_info.is_none() {
        return Ok(StatusCode::OK);
    }

    let team_info = data.player_team_info.unwrap();

    if team_info.player_jersey_name.len() > JERSEY_NAME_LIMIT
        || team_info.player_jersey_num.len() > JERSEY_NUMBER_LIMIT
        || !team_info.player_jersey_num.chars().all(char::is_numeric)
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query!(
        "INSERT INTO leauge_registrations VALUES (current_date, $1, $2, $3, $4)",
        team_info.player_jersey_num,
        team_info.player_jersey_name,
        player_id,
        team_info.player_team_id
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(SqlError::InvalidArgument(_)) => Err(StatusCode::BAD_REQUEST),
        Err(SqlError::Database(v)) => {
            if v.is_foreign_key_violation() {
                Err(StatusCode::BAD_REQUEST)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
