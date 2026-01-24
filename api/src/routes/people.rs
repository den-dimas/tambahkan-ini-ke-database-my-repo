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
            CreatePersonRequest, GetUploadUrlRequest, Person, PersonCategory, PersonEdit,
            ProposeEditRequest, UploadUrlResponse,
        },
        response::ApiResponse,
    },
    routes::auth_middleware::AuthenticatedUser,
    services::person_service::PersonService,
};
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_person).get(list_people))
        .route("/search", get(search_people))
        .route("/{id}", get(get_person))
        .route("/{id}/edit", post(propose_edit))
        .route("/edits", get(list_edits))
        .route("/edits/{id}/vote", post(vote_on_edit))
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
    let person = PersonService::create_person(&state.pool, user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::success(person))))
}

async fn list_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<ListPeopleQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<Vec<Person>>>)> {
    let people = PersonService::list_people(&state.pool, user_id, query.category).await?;
    Ok(Json(ApiResponse::success(people)))
}

async fn get_person(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    axum::extract::Path(person_id): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<Person>>)> {
    let person = PersonService::get_person(&state.pool, user_id, person_id).await?;
    Ok(Json(ApiResponse::success(person)))
}

async fn propose_edit(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    axum::extract::Path(person_id): axum::extract::Path<Uuid>,
    Json(payload): Json<ProposeEditRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<PersonEdit>>)> {
    let edit = PersonService::propose_edit(&state.pool, user_id, person_id, payload).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::success(edit))))
}

async fn list_edits(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<Vec<PersonEdit>>>)> {
    let edits = PersonService::list_edits(&state.pool).await?;
    Ok(Json(ApiResponse::success(edits)))
}

async fn vote_on_edit(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    axum::extract::Path(edit_id): axum::extract::Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<String>>)> {
    let is_approve = payload["approve"].as_bool().unwrap_or(true);
    let message = PersonService::vote_on_edit(&state.pool, user_id, edit_id, is_approve).await?;
    Ok(Json(ApiResponse::success(message)))
}

async fn search_people(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<serde_json::Value>>)> {
    let result_json =
        PersonService::search_people(&state.pool, &state.cache, user_id, query.q).await?;
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
                .map_err(|e: aws_sdk_s3::presigning::PresigningConfigError| {
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
