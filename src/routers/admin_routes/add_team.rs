use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use serde::Deserialize;
use sqlx::Error as SqlxError;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct TeamCreationReq {
    name: String,
    // scary scary should be updated
    location: String,
    captain_id: u32,
}

const TEAM_NAME_LIMIT: usize = 32;

// again want to be like geo spacial data like "Golden, CO"
const LOCATION_LIMIT: usize = 40;

pub async fn add_team(
    State(state): State<AppState>,
    Json(data): Json<TeamCreationReq>,
) -> impl IntoResponse {
    if data.name.len() > TEAM_NAME_LIMIT || data.location.len() > LOCATION_LIMIT {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query!(
        "INSERT INTO teams (name, current_location, captain_id) VALUES ($1, $2, $3)",
        data.name,
        data.location,
        data.captain_id as i32
    )
    .execute(&state.db_conn)
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(SqlxError::Database(v)) => {
            if v.is_foreign_key_violation() {
                Err(StatusCode::BAD_REQUEST)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
