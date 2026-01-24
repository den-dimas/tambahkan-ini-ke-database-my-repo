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

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "edit_status", rename_all = "lowercase")]
pub enum EditStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: Uuid,        // tracking_id
    pub person_id: Uuid, // global person_id
    pub user_id: Uuid,
    pub name: String,
    pub category: PersonCategory,
    pub description: Option<String>, // User's private description
    pub global_description: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub image_url: Option<String>,
    pub creator_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonPhoto {
    pub id: Uuid,
    pub person_id: Uuid,
    pub url: String,
    pub order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
    pub category: PersonCategory,
    pub description: Option<String>, // Personal description
    pub global_description: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: Option<String>,
    pub category: Option<PersonCategory>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposeEditRequest {
    pub description: Option<String>,
    pub age: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonSearchResult {
    pub person_id: Uuid,
    pub name: String,
    pub tracking_id: Option<Uuid>,
    pub category: Option<PersonCategory>,
    pub description: Option<String>,
    pub global_description: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonEdit {
    pub id: Uuid,
    pub person_id: Uuid,
    pub proposer_id: Uuid,
    pub new_description: Option<String>,
    pub new_age: Option<i32>,
    pub new_image_url: Option<String>,
    pub status: EditStatus,
    pub created_at: DateTime<Utc>,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CategoryStat {
    pub category: PersonCategory,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_people: i64,
    pub category_counts: Vec<CategoryStat>,
    pub top_tracked: Vec<PersonSearchResult>, // Reusing this for simplicity
}
