use axum::{
    extract::{Path, Request, State}, http::{HeaderMap, StatusCode}, middleware::Next, response::IntoResponse, Extension, Json
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

pub async fn check_owner_id_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let owner_header = headers.get("x-owner");
    let owner_id: i32;

    match owner_header {
        Some(h) => owner_id = h.to_str().unwrap().to_owned().parse().map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                json!({ "success": false, "message": "Invalid header" }).to_string()
            )
        })?,
        None => return Err((
            StatusCode::BAD_REQUEST,
            json!({ "success": false, "message": "Invalid header" }).to_string()
        ))
    };

    req.extensions_mut().insert(owner_id);
    Ok(next.run(req).await)
}

pub async fn get_workspaces(
    Extension(owner_id): Extension<i32>,
    State(pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspaces = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE owner_id = $1")
        .bind(owner_id)
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
    Extension(owner_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE id = $1 AND owner_id = $2")
        .bind(workspace_id)
        .bind(owner_id)
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
    Extension(owner_id): Extension<i32>,
    State(pool): State<PgPool>,
    Json(body): Json<CreateWorkspaceReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, WorkspaceRow>("INSERT INTO workspaces(name, description, status, owner_id) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(body.name)
        .bind(body.description)
        .bind(Status::Active)
        .bind(owner_id)
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
    Extension(owner_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(workspace_id): Path<i32>,
    Json(body): Json<UpdateWorkspaceStatusReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, WorkspaceRow>("UPDATE workspaces SET status = $1 WHERE id = $2 AND owner_id = $3 RETURNING *")
        .bind(body.status)
        .bind(workspace_id)
        .bind(owner_id)
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


// ---------------------- For easier development process ----------------------

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

// ---------------------- For tests ----------------------

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Method, Request}, middleware::from_fn, routing::get, Router};
    use serde_json::Value;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use tower::ServiceExt;          // allow 'oneshot' method
    use http_body_util::BodyExt;    // allow 'collect' method
    use super::*;

    async fn create_connection_pool() -> PgPool {
        dotenvy::dotenv().expect("Cannot access .env file");

        let database_url: String = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not found in .env file");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Cannot connect to database");
        
        return pool;
    }

    async fn app() -> Router {
        let pool = create_connection_pool().await;

        let app: Router = Router::new()
            .route("/workspaces", get(get_workspaces).post(create_workspace))
            .route("/workspaces/:workspace_id", get(get_workspace_by_id).put(update_workspace_status))
            .layer(from_fn(check_owner_id_middleware))
            .with_state(pool);

        return app;
    }

    #[tokio::test]
    async fn test_database_connectivity() {
        let pool = create_connection_pool().await;

        assert_eq!(pool.is_closed(), false);
    }

    #[tokio::test]
    async fn test_create_workspace_successfully() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::POST)
            .uri("/workspaces")
            .header("content-type", "application/json")
            .header("x-owner", 1)
            .body(Body::from(
                r#"{
                    "name": "w1 of u1",
                    "description": "w1 of u1"
                }"#,
            ))
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body_bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body["success"], true);
        assert_eq!(body["data"]["owner_id"], 1);
    }

    #[tokio::test]
    async fn test_create_workspace_failed_due_to_lack_of_ower_id_header() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::POST)
            .uri("/workspaces")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "name": "w1 of u1",
                    "description": "w1 of u1"
                }"#,
            ))
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_workspace_failed_due_to_lack_of_name_field() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::POST)
            .uri("/workspaces")
            .header("content-type", "application/json")
            .header("x-owner", 1)
            .body(Body::from(
                r#"{
                    "description": "w1 of u1"
                }"#,
            ))
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_get_workspaces() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::GET)
            .uri("/workspaces")
            .header("x-owner", 1)
            .body(Body::empty())
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_workspace_by_id() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::GET)
            .uri("/workspaces/1")
            .header("x-owner", 1)
            .body(Body::empty())
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_workspace_status_successfully() {
        let app: Router = app().await;

        let req = Request::builder()
            .method(Method::PUT)
            .uri("/workspaces/1")
            .header("content-type", "application/json")
            .header("x-owner", 1)
            .body(Body::from(
                r#"{
                    "status": "Expired"
                }"#,
            ))
            .unwrap();
        
        let res = app
            .oneshot(req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body_bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body["success"], true);
        assert_eq!(body["data"]["status"], "Expired");
    }
}
