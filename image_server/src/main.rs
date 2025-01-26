use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod handlers;
use handlers::fetch_image;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/image/{image_name}", get(fetch_image))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("localhost:3030")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    info!("root route was hit");
    "Hello, World"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    info!(?user, "debug print");
    info!("username of user {} is {}", user.id, user.username);

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Debug)]
struct User {
    id: u64,
    username: String,
}
