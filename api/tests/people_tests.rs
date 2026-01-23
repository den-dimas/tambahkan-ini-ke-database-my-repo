use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

mod common;

async fn get_token(app: &axum::Router, username: &str) -> String {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "username": username,
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    body["data"]["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_people_crud_and_search() {
    let (app, pool) = common::setup_app().await;
    let username = format!("user_{}", Uuid::new_v4());
    let token = get_token(&app, &username).await;

    // 1. Create Person
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/people")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": "John Doe",
                        "category": "Bini",
                        "description": "A test person"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: Value = serde_json::from_slice(&body).unwrap();
    let person = &res["data"];
    assert_eq!(person["name"], "John Doe");
    let person_id = person["id"].as_str().unwrap().to_string();

    // 2. List People
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/people")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: Value = serde_json::from_slice(&body).unwrap();
    let people = res["data"].as_array().unwrap();
    assert!(people.len() >= 1);

    // 3. Search People
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/people/search?q=John")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let res: Value = serde_json::from_slice(&body).unwrap();
    let results = res["data"].as_array().unwrap();
    assert_eq!(results[0]["name"], "John Doe");

    // Cleanup
    sqlx::query("DELETE FROM person_trackings WHERE id = $1::uuid")
        .bind(&person_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(&username)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_shared_people_sharing() {
    let (app, pool) = common::setup_app().await;

    let username1 = format!("user_{}", Uuid::new_v4());
    let token1 = get_token(&app, &username1).await;

    let username2 = format!("user_{}", Uuid::new_v4());
    let token2 = get_token(&app, &username2).await;

    let shared_name = format!("Waguri_{}", Uuid::new_v4());

    // 1. User 1 adds shared_name
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/people")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token1))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": shared_name,
                        "category": "Kisah",
                        "description": "User 1's Kisah"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::CREATED);

    // 2. User 2 adds shared_name
    let response2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/people")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token2))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": shared_name,
                        "category": "Bini",
                        "description": "User 2's Bini"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::CREATED);
    let body2 = response2.into_body().collect().await.unwrap().to_bytes();
    let res2: Value = serde_json::from_slice(&body2).unwrap();
    let person_id = res2["data"]["person_id"].as_str().unwrap().to_string();

    // 3. Verify in database that there is only ONE person_base for this name
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM person_bases WHERE name = $1")
        .bind(&shared_name)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // 4. Verify there are TWO trackings
    let tracking_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM person_trackings WHERE person_id = $1::uuid")
            .bind(&person_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(tracking_count, 2);

    // Cleanup
    sqlx::query("DELETE FROM person_trackings WHERE person_id = $1::uuid")
        .bind(&person_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM person_bases WHERE id = $1::uuid")
        .bind(&person_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE username IN ($1, $2)")
        .bind(&username1)
        .bind(&username2)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_people_unauthorized() {
    let (app, _) = common::setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/people")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
