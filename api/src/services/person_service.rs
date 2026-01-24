use crate::{
    models::person::{
        CategoryStat, CreatePersonRequest, EditStatus, Person, PersonCategory, PersonEdit,
        PersonSearchResult, ProposeEditRequest, StatsResponse,
    },
    utils::api_response::ApiResponse,
};
use axum::Json;
use axum::http::StatusCode;
use moka::future::Cache;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PersonService;

impl PersonService {
    pub async fn create_person(
        pool: &PgPool,
        user_id: Uuid,
        payload: CreatePersonRequest,
    ) -> Result<Person, (StatusCode, Json<ApiResponse<Person>>)> {
        let mut tx = pool.begin().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        // 1. Get or create person_base
        let name = payload.name.trim();
        let person_base = sqlx::query!(
            "SELECT id, creator_id FROM person_bases WHERE name = $1",
            name
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e: sqlx::Error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        let person_base_id = match person_base {
            Some(row) => row.id,
            None => {
                // First time registration: Add global info directly
                sqlx::query!(
                    r#"
                    INSERT INTO person_bases (name, global_description, age, image_url, creator_id, gender)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id
                    "#,
                    name,
                    payload.global_description,
                    payload.age,
                    payload.image_url,
                    user_id,
                    payload.gender
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e: sqlx::Error| {
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
                INSERT INTO person_trackings (user_id, person_id, category, description)
                VALUES ($1, $2, $3, $4)
                RETURNING id, person_id, user_id, category, description, created_at
            )
            SELECT
                i.id, i.person_id, i.user_id, pb.name, i.category, i.description,
                pb.global_description, pb.age, pb.gender, pb.image_url, pb.creator_id, i.created_at
            FROM inserted i
            JOIN person_bases pb ON i.person_id = pb.id
            "#,
        )
        .bind(user_id)
        .bind(person_base_id)
        .bind(&payload.category)
        .bind(&payload.description)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_e: sqlx::Error| {
            // If already tracking, return conflict
            (
                StatusCode::CONFLICT,
                Json(ApiResponse::error(
                    "You already have this person in your list",
                )),
            )
        })?;

        tx.commit().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok(person)
    }

    pub async fn list_people(
        pool: &PgPool,
        user_id: Uuid,
        category: Option<PersonCategory>,
    ) -> Result<Vec<Person>, (StatusCode, Json<ApiResponse<Vec<Person>>>)> {
        let sql = r#"
            SELECT
                pt.id, pt.person_id, pt.user_id, pb.name, pt.category, pt.description,
                pb.global_description, pb.age, pb.gender, pb.image_url, pb.creator_id, pt.created_at
            FROM person_trackings pt
            JOIN person_bases pb ON pt.person_id = pb.id
            WHERE pt.user_id = $1
        "#;

        let people = if let Some(category) = category {
            sqlx::query_as::<_, Person>(&format!(
                "{} AND pt.category = $2 ORDER BY pt.created_at DESC",
                sql
            ))
            .bind(user_id)
            .bind(category)
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, Person>(&format!("{} ORDER BY pt.created_at DESC", sql))
                .bind(user_id)
                .fetch_all(pool)
                .await
        }
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok(people)
    }

    pub async fn get_person(
        pool: &PgPool,
        user_id: Uuid,
        person_id: Uuid,
    ) -> Result<Person, (StatusCode, Json<ApiResponse<Person>>)> {
        let person = sqlx::query_as::<_, Person>(
            r#"
            SELECT
                pt.id, pb.id as person_id, pt.user_id, pb.name, pt.category, pt.description,
                pb.global_description, pb.age, pb.gender, pb.image_url, pb.creator_id, pt.created_at
            FROM person_bases pb
            LEFT JOIN person_trackings pt ON pb.id = pt.person_id AND pt.user_id = $1
            WHERE pb.id = $2
            "#,
        )
        .bind(user_id)
        .bind(person_id)
        .fetch_optional(pool)
        .await
        .map_err(|e: sqlx::Error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Person not found")),
        ))?;

        Ok(person)
    }

    pub async fn propose_edit(
        pool: &PgPool,
        user_id: Uuid,
        person_id: Uuid,
        payload: ProposeEditRequest,
    ) -> Result<PersonEdit, (StatusCode, Json<ApiResponse<PersonEdit>>)> {
        let edit = sqlx::query_as::<_, PersonEdit>(
            r#"
            INSERT INTO person_edits (person_id, proposer_id, new_description, new_age, new_image_url)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, person_id, proposer_id, new_description, new_age, new_image_url, status, created_at
            "#,
        )
        .bind(person_id)
        .bind(user_id)
        .bind(payload.description)
        .bind(payload.age)
        .bind(payload.image_url)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok(edit)
    }

    pub async fn list_edits(
        pool: &PgPool,
    ) -> Result<Vec<PersonEdit>, (StatusCode, Json<ApiResponse<Vec<PersonEdit>>>)> {
        let edits = sqlx::query_as::<_, PersonEdit>(
            "SELECT id, person_id, proposer_id, new_description, new_age, new_image_url, status, created_at FROM person_edits WHERE status = 'pending' ORDER BY created_at ASC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(e.to_string())))
        })?;

        Ok(edits)
    }

    pub async fn vote_on_edit(
        pool: &PgPool,
        user_id: Uuid,
        edit_id: Uuid,
        is_approve: bool,
    ) -> Result<String, (StatusCode, Json<ApiResponse<String>>)> {
        let mut tx = pool.begin().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        // 1. Record vote
        sqlx::query!(
            "INSERT INTO person_votes (edit_id, voter_id, is_approve) VALUES ($1, $2, $3) ON CONFLICT (edit_id, voter_id) DO UPDATE SET is_approve = EXCLUDED.is_approve",
            edit_id,
            user_id,
            is_approve
        )
        .execute(&mut *tx)
        .await
        .map_err(|e: sqlx::Error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        // 2. Check consensus (simple: if 1 more person approves, we approve)
        if is_approve {
            let vote_count: i64 = sqlx::query_scalar!(
                "SELECT count(*) FROM person_votes WHERE edit_id = $1 AND is_approve = true",
                edit_id
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(e.to_string())),
                )
            })?
            .unwrap_or(0);

            if vote_count >= 1 {
                let edit = sqlx::query!(
                    r#"
                    SELECT id, person_id, proposer_id, new_description, new_age, new_image_url, status AS "status: EditStatus", created_at
                    FROM person_edits WHERE id = $1
                    "#,
                    edit_id
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(e.to_string())),
                    )
                })?;

                if edit.proposer_id != user_id {
                    // Apply edit
                    sqlx::query!(
                        r#"
                        UPDATE person_bases
                        SET
                            global_description = COALESCE($1, global_description),
                            age = COALESCE($2, age),
                            image_url = COALESCE($3, image_url)
                        WHERE id = $4
                        "#,
                        edit.new_description,
                        edit.new_age,
                        edit.new_image_url,
                        edit.person_id
                    )
                    .execute(&mut *tx)
                    .await
                    .map_err(|e: sqlx::Error| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::error(e.to_string())),
                        )
                    })?;

                    sqlx::query!(
                        "UPDATE person_edits SET status = 'approved' WHERE id = $1",
                        edit_id
                    )
                    .execute(&mut *tx)
                    .await
                    .map_err(|e: sqlx::Error| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::error(e.to_string())),
                        )
                    })?;
                }
            }
        }

        tx.commit().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok("Vote recorded".to_string())
    }

    pub async fn search_people(
        pool: &PgPool,
        cache: &Cache<String, serde_json::Value>,
        user_id: Option<Uuid>,
        q: String,
    ) -> Result<serde_json::Value, (StatusCode, Json<ApiResponse<serde_json::Value>>)> {
        let user_key = user_id
            .map(|u| u.to_string())
            .unwrap_or_else(|| "guest".to_string());
        let cache_key = format!("search:{}:{}", user_key, q);

        // Check cache
        if let Some(cached_result) = cache.get(&cache_key).await {
            return Ok(cached_result);
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
                pb.global_description,
                pb.age,
                pb.gender,
                pb.image_url
            FROM person_bases pb
            LEFT JOIN person_trackings pt ON pb.id = pt.person_id AND pt.user_id = $1
            WHERE pb.name ILIKE $2
            ORDER BY pb.name ASC
            LIMIT 10
            "#,
        )
        .bind(user_id)
        .bind(format!("{}%", q))
        .fetch_all(pool)
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
        cache.insert(cache_key, result_json.clone()).await;

        Ok(result_json)
    }

    pub async fn list_feed(
        pool: &PgPool,
        user_id: Option<Uuid>,
    ) -> Result<Vec<PersonSearchResult>, (StatusCode, Json<ApiResponse<Vec<PersonSearchResult>>>)>
    {
        // Return random people
        // If user_id is provided, check if they are tracking
        let people = sqlx::query_as::<_, PersonSearchResult>(
            r#"
            SELECT
                pb.id as person_id,
                pb.name,
                pt.id as tracking_id,
                pt.category,
                pt.description,
                pb.global_description,
                pb.age,
                pb.gender,
                pb.image_url
            FROM person_bases pb
            LEFT JOIN person_trackings pt ON pb.id = pt.person_id AND pt.user_id = $1
            ORDER BY RANDOM()
            LIMIT 20
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok(people)
    }

    pub async fn get_stats(
        pool: &PgPool,
        user_id: Option<Uuid>,
    ) -> Result<StatsResponse, (StatusCode, Json<ApiResponse<StatsResponse>>)> {
        // 1. Total people count
        let total_people = sqlx::query_scalar!("SELECT count(*) FROM person_bases")
            .fetch_one(pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(e.to_string())),
                )
            })?
            .unwrap_or(0);

        // 2. Category counts (global or specific? Let's do global usage of categories)
        let category_counts = sqlx::query_as::<_, CategoryStat>(
            r#"
            SELECT category, count(*) as count
            FROM person_trackings
            GROUP BY category
            ORDER BY count DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        // 3. Top tracked people
        // We want to return PersonSearchResult format
        let top_tracked = sqlx::query_as::<_, PersonSearchResult>(
            r#"
            WITH top AS (
                SELECT person_id, count(*) as track_count
                FROM person_trackings
                GROUP BY person_id
                ORDER BY track_count DESC
                LIMIT 5
            )
            SELECT
                pb.id as person_id,
                pb.name,
                pt.id as tracking_id,
                pt.category,
                pt.description,
                pb.global_description,
                pb.age,
                pb.gender,
                pb.image_url
            FROM top
            JOIN person_bases pb ON top.person_id = pb.id
            LEFT JOIN person_trackings pt ON pb.id = pt.person_id AND pt.user_id = $1
            ORDER BY top.track_count DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(e.to_string())),
            )
        })?;

        Ok(StatsResponse {
            total_people,
            category_counts,
            top_tracked,
        })
    }
}
