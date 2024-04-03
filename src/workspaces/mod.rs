use axum::{
    extract::{Path, State}, http::{HeaderMap, StatusCode}, Json
};

use serde::{Serialize, Deserialize};
use serde_json::json;

use sqlx::PgPool;

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::Type)]
pub enum Status {
    Inactive,
    Active,
    Expired,
    Provisioning
}

#[derive(Serialize, sqlx::FromRow)]
pub struct WorkspaceRow {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub status: Status,
    pub owner_id: i32
}

#[derive(Deserialize)]
pub struct CreateWorkspaceReq {
    pub name: String,
    pub description: String
}

#[derive(Deserialize)]
pub struct UpdateWorkspaceStatusReq {
    pub status: Status
}

pub async fn get_workspaces(
    headers: HeaderMap,
    State(pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

    let workspaces = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE owner_id = $1")
        .bind(onwer_id)
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
        json!({ "success": true, "data": workspaces }).to_string()
    ))
}

pub async fn get_workspace_by_id(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

    let workspace = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE id = $1 AND owner_id = $2")
        .bind(workspace_id)
        .bind(onwer_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string()
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": workspace }).to_string()
    ))
}

pub async fn create_workspace(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Json(body): Json<CreateWorkspaceReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

    let workspace = sqlx::query_as::<_, WorkspaceRow>("INSERT INTO workspaces(name, description, status, owner_id) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(body.name)
        .bind(body.description)
        .bind(Status::Active)
        .bind(onwer_id)
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
        json!({ "success": true, "data": workspace }).to_string()
    ))
}

pub async fn update_workspace_status(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
    Json(body): Json<UpdateWorkspaceStatusReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

    let workspace = sqlx::query_as::<_, WorkspaceRow>("UPDATE workspaces SET status = $1 WHERE id = $2 AND owner_id = $3 RETURNING *")
        .bind(body.status)
        .bind(workspace_id)
        .bind(onwer_id)
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
        json!({ "success": true, "data": workspace }).to_string()
    ))
}


// For easier development process

pub async fn _get_workspaces(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspaces = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces")
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
        json!({ "success": true, "data": workspaces }).to_string()
    ))
}

pub async fn _get_workspace_by_id(
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE id = $1")
        .bind(workspace_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string()
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": workspace }).to_string()
    ))
}

pub async fn _update_workspace_status(
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
    Json(body): Json<UpdateWorkspaceStatusReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, WorkspaceRow>("UPDATE workspaces SET status = $1 WHERE id = $2 RETURNING *")
        .bind(body.status)
        .bind(workspace_id)
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
        json!({ "success": true, "data": workspace }).to_string()
    ))
}

pub async fn _delete_workspace(
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM workspaces WHERE id = $1")
        .bind(workspace_id)
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
