use axum::{
    Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::Route,
};

use crate::{cookies::Cookies, routers::auth::UserClaim, state::AppState};
use rusty_paseto::prelude::*;
use sqlx::Error as SqlError;

async fn check_admin(
    State(state): State<AppState>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let value = cookies.get_cookie("session").and_then(|token| {
        PasetoParser::<V4, Local>::default()
            .parse_into::<UserClaim>(token, &state.paseto)
            .ok()
    });

    if value.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let value = value.unwrap();

    // get from woofever data base etc etc
    let is_admin = match sqlx::query!(
        "SELECT site_admin from USERS WHERE username = $1",
        value.username
    )
    .fetch_one(&state.db_conn)
    .await
    {
        Ok(v) => v.site_admin,
        Err(SqlError::RowNotFound) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

// routes that require admin acess to use
pub fn make_admin_routes(state: AppState) -> Router {
    Router::new()
        .layer(middleware::from_fn_with_state(state.clone(), check_admin))
        .with_state(state)
}
