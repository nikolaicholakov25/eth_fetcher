use axum::{
    extract::State,
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;

use crate::{
    config::AppState,
    utils::{
        auth::{decode_jwt, generate_jwt, return_jwt},
        db::user::{fetch_user, login_user},
        structs::auth::{AuthPayload, AuthResponse, AuthUser, DbUser, JwtPayload},
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/me", get(me))
}

pub async fn authenticate(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    match login_user(&state.db_connection, &payload.user, &payload.password).await {
        Ok(_) => return_jwt(generate_jwt(payload)).await,
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn me(State(state): State<AppState>, user: AuthUser) {
    println!("{:?}", user);
}
