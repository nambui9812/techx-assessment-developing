mod users;
mod workspaces;

use axum::{middleware::from_fn, routing::{delete, get}, Router};

use tokio::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use users::{get_users, create_user, delete_user};
use workspaces::{check_owner_id_middleware, create_workspace, get_workspace_by_id, get_workspaces, update_workspace_status };

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

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let app = Router::new()
        .route("/workspaces", get(get_workspaces).post(create_workspace))
        .route("/workspaces/:workspace_id", get(get_workspace_by_id).put(update_workspace_status))
        .layer(from_fn(check_owner_id_middleware))
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:user_id", delete(delete_user))
        .with_state(db_pool);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}
