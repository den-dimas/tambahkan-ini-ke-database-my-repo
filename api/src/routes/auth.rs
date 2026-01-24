use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    models::user::{AuthResponse, LoginRequest, RegisterRequest, User},
    services::auth_service::AuthService,
    state::AppState,
    utils::api_response::ApiResponse,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<serde_json::Value>>)> {
    if payload.username.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "Username must be at least 3 characters long",
            )),
        ));
    }

    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "Password must be at least 8 characters long",
            )),
        ));
    }

    let password_hash = AuthService::hash_password(&payload.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, created_at",
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("User already exists or database error: {}", e))),
        )
    })?;

    let token = AuthService::generate_token(user.id, &state.config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(AuthResponse {
            token,
            username: user.username,
        })),
    ))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<AuthResponse>>)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?
    .ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse::error("Invalid credentials")),
    ))?;

    if !AuthService::verify_password(&payload.password, &user.password_hash) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Invalid credentials")),
        ));
    }

    let token = AuthService::generate_token(user.id, &state.config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    Ok(Json(ApiResponse::success(AuthResponse {
        token,
        username: user.username,
    })))
}
