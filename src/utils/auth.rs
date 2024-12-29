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

pub fn return_jwt(jwt_response: Result<String, Error>) -> Result<Json<AuthResponse>, StatusCode> {
    match jwt_response {
        Ok(jwt) => Ok(Json(AuthResponse { token: jwt })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn decode_jwt(token: String) -> Result<TokenData<JwtPayload>, Error> {
    decode::<JwtPayload>(
        token.as_str(),
        &DecodingKey::from_secret("JWT_SECRET".as_ref()),
        &Validation::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::errors::ErrorKind;

    fn fixture_generate_jwt(id: String) -> Result<String, Error> {
        let payload = AuthPayload {
            username: id.clone(),
            password: id.clone(),
        };
        generate_jwt(payload)
    }

    #[test]
    fn test_generate_jwt_success() {
        let jwt = fixture_generate_jwt("alice".to_string());

        assert!(jwt.is_ok(), "JWT generation should succeed");
        let jwt = jwt.unwrap();

        assert!(!jwt.is_empty(), "Generated JWT should not be empty");
    }

    #[test]
    fn test_return_jwt_success() {
        let jwt_result = fixture_generate_jwt("alice".to_string());
        let jwt_token = jwt_result.clone().unwrap();

        let return_jwt_result = return_jwt(jwt_result);
        assert!(
            return_jwt_result.is_ok(),
            "Return JWT should succeed with valid input"
        );

        assert_eq!(
            jwt_token,
            return_jwt_result.unwrap().token,
            "Returned token should match input JWT"
        );
    }

    #[test]
    fn test_return_jwt_failure() {
        let jwt_response: Result<String, Error> = Err(Error::from(ErrorKind::InvalidToken));

        let result = return_jwt(jwt_response);
        assert!(result.is_err(), "Return JWT should fail with invalid input");
        assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_decode_jwt_success() {
        let jwt_result = fixture_generate_jwt("alice".to_string());
        let jwt = jwt_result.unwrap();

        let result = decode_jwt(jwt);
        assert!(result.is_ok(), "Decoding a valid JWT should succeed");

        let token_data = result.unwrap();
        assert_eq!(
            token_data.claims.user, "alice",
            "Decoded username should match"
        );
    }

    #[test]
    fn test_decode_jwt_failure() {
        let invalid_jwt = "invalid_jwt_token".to_string();

        let result = decode_jwt(invalid_jwt);
        assert!(result.is_err(), "Decoding an invalid JWT should fail");
    }
}
