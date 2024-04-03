mod users;
mod workspaces;

use axum::{
    routing::{delete, get}, Router
};

use tokio::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use users::{get_users, create_user, delete_user};
use workspaces::{get_workspaces, get_workspace_by_id, create_workspace, update_workspace_status, delete_workspace};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Cannot access .env file");

    let database_url: String = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not found in .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Cannot connect to database");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/workspaces", get(get_workspaces).post(create_workspace))
        .route("/workspaces/:workspace_id", get(get_workspace_by_id).put(update_workspace_status).delete(delete_workspace))
        .with_state(db_pool);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}

// users

// #[derive(Serialize, sqlx::FromRow)]
// struct UserRow {
//     id: i32,
//     username: String,
// }

// #[derive(Deserialize)]
// struct CreateUserReq {
//     username: String
// }

// async fn get_users(
//     State(pool): State<PgPool>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let users = sqlx::query_as::<_, UserRow>("SELECT * FROM users")
//         .fetch_all(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;

//     Ok((
//         StatusCode::OK,
//         json!({ "success": true, "data": users }).to_string()
//     ))
// }

// async fn create_user(
//     State(pool): State<PgPool>,
//     Json(body): Json<CreateUserReq>
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let user = sqlx::query_as::<_, UserRow>("INSERT INTO users(username) VALUES ($1) RETURNING *")
//         .bind(body.username)
//         .fetch_one(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;
    
//     Ok((
//         StatusCode::CREATED,
//         json!({ "success": true, "data": user }).to_string()
//     ))
// }

// async fn delete_user(
//     State(pool): State<PgPool>,
//     Path(user_id): Path<i32>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     sqlx::query("DELETE FROM users WHERE id = $1")
//         .bind(user_id)
//         .execute(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;
    
//     Ok((
//         StatusCode::OK,
//         json!({ "success": true }).to_string()
//     ))
// }

// workspaces

// #[derive(Serialize, Deserialize, sqlx::Type)]
// enum Status {
//     Inactive,
//     Active,
//     Expired,
//     Provisioning
// }

// #[derive(Serialize, sqlx::FromRow)]
// struct WorkspaceRow {
//     id: i32,
//     name: String,
//     description: String,
//     create_time: DateTime<Utc>,
//     update_time: DateTime<Utc>,
//     status: Status,
//     owner_id: i32
// }

// #[derive(Deserialize)]
// struct CreateWorkspaceReq {
//     name: String,
//     description: String
// }

// #[derive(Deserialize)]
// struct UpdateWorkspaceStatusReq {
//     status: Status
// }

// async fn get_workspaces(
//     State(pool): State<PgPool>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let workspaces = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces")
//         .fetch_all(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;

//     Ok((
//         StatusCode::OK,
//         json!({ "success": true, "data": workspaces }).to_string()
//     ))
// }

// async fn get_workspace_by_id(
//     State(pool): State<PgPool>,
//     Path(workspace_id): Path<i32>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let workspace = sqlx::query_as::<_, WorkspaceRow>("SELECT * FROM workspaces WHERE id = $1")
//         .bind(workspace_id)
//         .fetch_one(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;

//     Ok((
//         StatusCode::OK,
//         json!({ "success": true, "data": workspace }).to_string()
//     ))
// }

// async fn create_workspace(
//     State(pool): State<PgPool>,
//     headers: HeaderMap,
//     Json(body): Json<CreateWorkspaceReq>
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

//     let workspace = sqlx::query_as::<_, WorkspaceRow>("INSERT INTO workspaces(name, description, status, owner_id) VALUES ($1, $2, $3, $4) RETURNING *")
//         .bind(body.name)
//         .bind(body.description)
//         .bind(Status::Active)
//         .bind(onwer_id)
//         .fetch_one(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;
    
//     Ok((
//         StatusCode::CREATED,
//         json!({ "success": true, "data": workspace }).to_string()
//     ))
// }

// async fn update_workspace_status(
//     State(pool): State<PgPool>,
//     headers: HeaderMap,
//     Path(workspace_id): Path<i32>,
//     Json(body): Json<UpdateWorkspaceStatusReq>
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     let onwer_id: i32 = headers.get("x-owner").unwrap().to_str().unwrap().to_owned().parse().expect("Invalid headers");

//     let workspace = sqlx::query_as::<_, WorkspaceRow>("UPDATE workspaces SET status = $1 WHERE id = $2 RETURNING *")
//         .bind(body.status)
//         .bind(workspace_id)
//         .bind(Status::Active)
//         .bind(onwer_id)
//         .fetch_one(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;
    
//     Ok((
//         StatusCode::CREATED,
//         json!({ "success": true, "data": workspace }).to_string()
//     ))
// }

// async fn delete_workspace(
//     State(pool): State<PgPool>,
//     Path(workspace_id): Path<i32>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//     sqlx::query("DELETE FROM workspaces WHERE id = $1")
//         .bind(workspace_id)
//         .execute(&pool)
//         .await
//         .map_err(|e| {
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 json!({ "success": false, "message": e.to_string() }).to_string()
//             )
//         })?;
    
//     Ok((
//         StatusCode::OK,
//         json!({ "success": true }).to_string()
//     ))
// }
