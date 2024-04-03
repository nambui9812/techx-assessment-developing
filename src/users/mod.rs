use axum::{
    extract::{Path, State}, http::StatusCode, Json
};

use serde::{Serialize, Deserialize};
use serde_json::json;

use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct UserRow {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUserReq {
    pub username: String
}

pub async fn get_users(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let users = sqlx::query_as::<_, UserRow>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string()
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": users }).to_string()
    ))
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUserReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user = sqlx::query_as::<_, UserRow>("INSERT INTO users(username) VALUES ($1) RETURNING *")
        .bind(body.username)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string()
            )
        })?;
    
    Ok((
        StatusCode::CREATED,
        json!({ "success": true, "data": user }).to_string()
    ))
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string()
            )
        })?;
    
    Ok((
        StatusCode::OK,
        json!({ "success": true }).to_string()
    ))
}