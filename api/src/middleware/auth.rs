use axum::{
    Json,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{config::app::AppConfig, models::user::Claims, utils::api_response::ApiResponse};

pub struct AuthenticatedUser(pub Uuid);

pub struct OptionalUser(pub Option<Uuid>);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    &'static AppConfig: FromRef<S>,
{
    type Rejection = (StatusCode, Json<ApiResponse<serde_json::Value>>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse::error("Missing credentials")),
                    )
                })?;

        let config = <&'static AppConfig>::from_ref(state);

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Invalid token")),
            )
        })?;

        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}

impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    &'static AppConfig: FromRef<S>,
{
    type Rejection = (StatusCode, Json<ApiResponse<serde_json::Value>>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract authorization header
        // If missing or malformed, we return None (no user)
        // If present but invalid token, we return error? Or just None?
        // User said: "when not logged in... shouldve still show".
        // Usually optional auth means: if valid token, use it. If no token, guest. If invalid token, error (client bug/expiry).

        let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        else {
            return Ok(OptionalUser(None));
        };

        let config = <&'static AppConfig>::from_ref(state);

        match decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => Ok(OptionalUser(Some(token_data.claims.sub))),
            Err(_) => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Invalid token")),
            )),
        }
    }
}
