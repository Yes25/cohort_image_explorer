use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod handlers;
use handlers::{fetch_bucket_content, fetch_image};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/image/{image_name}", get(fetch_image))
        .route("/api/bucket/{bucket_name}", get(fetch_bucket_content))
        .route("/api/buckets", get(fetch_bucket_content));

    let listener = tokio::net::TcpListener::bind("localhost:3030")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    info!("root route was hit");
    "Hello, World"
}
