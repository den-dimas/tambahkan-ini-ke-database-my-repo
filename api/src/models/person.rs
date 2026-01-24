use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "person_category", rename_all = "lowercase")]
pub enum PersonCategory {
    Bini,
    Janda,
    Kisah,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: Uuid,
    pub person_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub category: PersonCategory,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
    pub category: PersonCategory,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: Option<String>,
    pub category: Option<PersonCategory>,
    pub description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonSearchResult {
    pub person_id: Uuid,
    pub name: String,
    pub tracking_id: Option<Uuid>,
    pub category: Option<PersonCategory>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUploadUrlRequest {
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadUrlResponse {
    pub upload_url: String,
    pub public_url: String,
    pub key: String,
}
