use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Deserialize, Debug)]
pub struct PageParams {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    skip: i64,
}

#[inline]
const fn default_limit() -> i64 {
    15
}

#[derive(Serialize)]
struct TeamDataRow {
    name: String,
    current_location: String,
    captain_name: String,
    team_id: i64,
}

// fetches all the teams
pub async fn get_teams(
    State(state): State<AppState>,
    Query(page): Query<PageParams>,
) -> impl IntoResponse {
    match sqlx::query_as!(
    	TeamDataRow,
        "SELECT t.name, t.team_id, t.current_location, p.name as captain_name FROM teams as t JOIN players as p ON p.player_id = t.captain_id ORDER BY t.name DESC LIMIT $1 OFFSET $2",
        page.limit,
        page.skip
    ).fetch_all(&state.db_conn)
    .await {
    	Ok(v) => Ok(Json(v)),
     	Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
struct PlayerDataRow {
    name: String,
    player_id: i64,
    recent_team: Option<String>,
}

pub async fn get_players(
    State(state): State<AppState>,
    Query(page): Query<PageParams>,
) -> impl IntoResponse {
    println!("limit: {}, skip: {}", page.limit, page.skip);
    match sqlx::query_as!(
		PlayerDataRow,
		"WITH team_registrations AS (SELECT t.name, lr.player_id, lr.date FROM teams as t JOIN leauge_registrations as lr ON lr.team_id = t.team_id)
		SELECT DISTINCT ON(p.player_id) p.name, p.player_id, tr.name as \"recent_team?\" FROM players as p
			LEFT JOIN team_registrations as tr ON p.player_id = tr.player_id ORDER BY p.player_id, tr.date DESC
			LIMIT $1 OFFSET $2",
		page.limit,
		page.skip
	).fetch_all(&state.db_conn)
	.await {
		Ok(v) => Ok(Json(v)),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}
