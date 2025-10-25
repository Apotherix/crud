use axum::Json;
use axum::{
    extract::{Extension, Path},
};
use sqlx::SqlitePool;
use crate::models::user::{User, CreateUser};

// Post /user - Create a new user
pub async fn create_user(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let rec = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES (?1, ?2)
        RETURNING id as "id: i64", name, email
        "#,
        payload.name,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rec))
}

pub async fn get_user(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    tracing::info!("get_user handler invoked - id={}", id);
    let user = sqlx::query_as!(
        User,
        "SELECT id as \"id: i64\", name, email FROM users WHERE id = ?1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "User not found".into()))?;

    Ok(Json(user))
}

pub async fn update_user(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let updated = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET name = ?1, email = ?2
        WHERE id = ?3
        RETURNING id as "id: i64", name, email
        "#,
        payload.name,
        payload.email,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "User not found".into()))?;

    Ok(Json(updated))
}

pub async fn delete_user(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    sqlx::query!("DELETE FROM users WHERE id = ?1", id)
    .execute(&pool)
    .await
    .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "User not found".into()))?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}