use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    config::AppState,
    utils::{auth::decode_jwt, db::user::fetch_user},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser(DbUser);
impl AuthUser {
    pub fn db_user(&self) -> &DbUser {
        &self.0 // Accessing the inner DbUser
    }
}
#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<AuthUser, Self::Rejection> {
        match parts.headers.get("auth_token") {
            Some(auth_token) => match decode_jwt(auth_token.to_str().unwrap().to_string()).await {
                Ok(token) => match fetch_user(&state.db_connection, &token.claims.user).await {
                    Ok(db_user) => Ok(AuthUser(db_user)),
                    Err(_) => Err(StatusCode::UNAUTHORIZED),
                },
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            },
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user: String,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DbUser {
    pub name: String,
    pub transactions: Vec<String>,
}
