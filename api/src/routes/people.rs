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
    models::{
        person::{
            CreatePersonRequest, GetUploadUrlRequest, Person, PersonCategory, PersonSearchResult,
            UploadUrlResponse,
        },
        response::ApiResponse,
    },
    routes::auth_middleware::AuthenticatedUser,
};
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_person).get(list_people))
        .route("/search", get(search_people))
        .route("/upload-url", post(get_upload_url))
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
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<Person>>)> {
    let mut tx = state.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    // 1. Get or create person_base
    let name = payload.name.trim();
    let person_base_id = match sqlx::query!("SELECT id FROM person_bases WHERE name = $1", name)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })? {
        Some(row) => row.id,
        None => {
            sqlx::query!(
                "INSERT INTO person_bases (name) VALUES ($1) RETURNING id",
                name
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(e.to_string())),
                )
            })?
            .id
        }
    };

    // 2. Create person_tracking
    let person = sqlx::query_as::<_, Person>(
        r#"
        WITH inserted AS (
            INSERT INTO person_trackings (user_id, person_id, category, description, image_url)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, person_id, user_id, category, description, image_url, created_at
        )
        SELECT i.id, i.person_id, i.user_id, pb.name, i.category, i.description, i.image_url, i.created_at
        FROM inserted i
        JOIN person_bases pb ON i.person_id = pb.id
        "#,
    )
    .bind(user_id)
    .bind(person_base_id)
    .bind(&payload.category)
    .bind(&payload.description)
    .bind(&payload.image_url)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(person))))
}

async fn list_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<ListPeopleQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<Vec<Person>>>)> {
    let sql = r#"
        SELECT pt.id, pt.person_id, pt.user_id, pb.name, pt.category, pt.description, pt.image_url, pt.created_at
        FROM person_trackings pt
        JOIN person_bases pb ON pt.person_id = pb.id
        WHERE pt.user_id = $1
    "#;

    let people = if let Some(category) = query.category {
        sqlx::query_as::<_, Person>(&format!(
            "{} AND pt.category = $2 ORDER BY pt.created_at DESC",
            sql
        ))
        .bind(user_id)
        .bind(category)
        .fetch_all(&state.pool)
        .await
    } else {
        sqlx::query_as::<_, Person>(&format!("{} ORDER BY pt.created_at DESC", sql))
            .bind(user_id)
            .fetch_all(&state.pool)
            .await
    }
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    Ok(Json(ApiResponse::success(people)))
}
async fn search_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<serde_json::Value>>)> {
    let cache_key = format!("search:{}:{}", user_id, query.q);

    // Check cache
    if let Some(cached_result) = state.cache.get(&cache_key).await {
        return Ok(Json(ApiResponse::success(cached_result)));
    }

    // Predictive search: Search in person_bases globally
    let people = sqlx::query_as::<_, PersonSearchResult>(
        r#"
        SELECT
            pb.id as person_id,
            pb.name,
            pt.id as tracking_id,
            pt.category,
            pt.description,
            pt.image_url
        FROM person_bases pb
        LEFT JOIN person_trackings pt ON pb.id = pt.person_id AND pt.user_id = $1
        WHERE pb.name ILIKE $2
        ORDER BY pb.name ASC
        LIMIT 10
        "#,
    )
    .bind(user_id)
    .bind(format!("{}%", query.q))
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    let result_json = serde_json::to_value(&people).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(e.to_string())),
        )
    })?;

    // Update cache
    state.cache.insert(cache_key, result_json.clone()).await;

    Ok(Json(ApiResponse::success(result_json)))
}

async fn get_upload_url(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(payload): Json<GetUploadUrlRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<UploadUrlResponse>>)> {
    let file_extension = payload
        .filename
        .split('.')
        .last()
        .unwrap_or("jpg")
        .to_lowercase();
    let key = format!("users/{}/{}.{}", user_id, Uuid::new_v4(), file_extension);

    let presigned_request = state
        .s3_client
        .put_object()
        .bucket(&state.config.r2_bucket_name)
        .key(&key)
        .content_type(&payload.content_type)
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(300))
                .build()
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(e.to_string())),
                    )
                })?,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

    let upload_url = presigned_request.uri().to_string();
    let public_url = if let Some(domain) = &state.config.r2_public_domain {
        let domain = domain.trim_end_matches('/');
        format!("{}/{}", domain, key)
    } else {
        // If no domain is configured, we return the key
        // But we should warn the user in the logs
        tracing::warn!(
            "R2_PUBLIC_DOMAIN is not set. Image URLs will be relative: {}",
            key
        );
        key.clone()
    };

    Ok(Json(ApiResponse::success(UploadUrlResponse {
        upload_url,
        public_url,
        key,
    })))
}
