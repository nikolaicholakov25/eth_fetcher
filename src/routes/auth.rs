use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use reqwest::StatusCode;

use crate::{
    config::AppState,
    utils::{
        auth::{generate_jwt, return_jwt},
        db::{transaction::fetch_matching_transactions, user::login_user},
        structs::{
            auth::{AuthPayload, AuthResponse, AuthUser},
            transaction::FetchResponse,
        },
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/my", get(my))
}

pub async fn authenticate(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    match login_user(&state.db_connection, &payload.username, &payload.password).await {
        Ok(_) => return_jwt(generate_jwt(payload)).await,
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn my(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<FetchResponse>, StatusCode> {
    match fetch_matching_transactions(&state.db_connection, user.db_user().transactions.clone())
        .await
    {
        Ok(transactions) => Ok(Json(FetchResponse { transactions })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
