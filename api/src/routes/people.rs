use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    AppState,
    models::person::{CreatePersonRequest, Person, PersonCategory},
    routes::auth_middleware::AuthenticatedUser,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_person).get(list_people))
        .route("/search", get(search_people))
}

#[derive(Deserialize)]
struct ListPeopleQuery {
    category: Option<PersonCategory>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn create_person(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(payload): Json<CreatePersonRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let person = sqlx::query_as::<_, Person>(
        "INSERT INTO people (user_id, name, category, description) VALUES ($1, $2, $3, $4) RETURNING id, user_id, name, category, description, created_at",
    )
    .bind(user_id)
    .bind(&payload.name)
    .bind(&payload.category)
    .bind(&payload.description)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok((StatusCode::CREATED, Json(person)))
}

async fn list_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<ListPeopleQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let people = if let Some(category) = query.category {
        sqlx::query_as::<_, Person>(
            "SELECT id, user_id, name, category, description, created_at FROM people WHERE user_id = $1 AND category = $2 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .bind(category)
        .fetch_all(&state.pool)
        .await
    } else {
        sqlx::query_as::<_, Person>(
            "SELECT id, user_id, name, category, description, created_at FROM people WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    Ok(Json(people))
}

async fn search_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cache_key = format!("search:{}:{}", user_id, query.q);

    // Check cache
    if let Some(cached_result) = state.cache.get(&cache_key).await {
        return Ok(Json(cached_result));
    }

    // Predictive search using ILIKE for case-insensitive prefix matching
    let people = sqlx::query_as::<_, Person>(
        "SELECT id, user_id, name, category, description, created_at FROM people WHERE user_id = $1 AND name ILIKE $2 ORDER BY name ASC LIMIT 10",
    )
    .bind(user_id)
    .bind(format!("{}%", query.q))
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))))?;

    let result_json = serde_json::to_value(&people).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Update cache
    state.cache.insert(cache_key, result_json.clone()).await;

    Ok(Json(result_json))
}
