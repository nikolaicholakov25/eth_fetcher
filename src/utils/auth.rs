use super::structs::auth::{AuthPayload, AuthResponse, JwtPayload};
use axum::Json;
use jsonwebtoken::{
    decode, encode, errors::Error, get_current_timestamp, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use reqwest::StatusCode;

pub fn generate_jwt(payload: AuthPayload) -> Result<String, Error> {
    encode(
        &Header::default(),
        &JwtPayload {
            user: payload.username,
            exp: get_current_timestamp() + 60 * 60 * 10, // 10 hours in sec,
        },
        &EncodingKey::from_secret("JWT_SECRET".as_ref()),
    )
}

pub async fn return_jwt(
    jwt_response: Result<String, Error>,
) -> Result<Json<AuthResponse>, StatusCode> {
    match jwt_response {
        Ok(jwt) => Ok(Json(AuthResponse { token: jwt })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn decode_jwt(token: String) -> Result<TokenData<JwtPayload>, Error> {
    decode::<JwtPayload>(
        token.as_str(),
        &DecodingKey::from_secret("JWT_SECRET".as_ref()),
        &Validation::default(),
    )
}
