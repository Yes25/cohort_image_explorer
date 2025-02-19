use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod handlers;
use handlers::{approve, fetch_bucket_content, fetch_buckets, fetch_image};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route(
            "/api/bucket/{bucket_name}/image/{image_name}",
            get(fetch_image),
        )
        .route("/api/bucket/{bucket_name}", get(fetch_bucket_content))
        .route("/api/buckets", get(fetch_buckets))
        .route("/api/approve/bucket/{bucket_name}", post(approve))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::very_permissive())
                .layer(CompressionLayer::new()),
        );

    let listener = tokio::net::TcpListener::bind("localhost:3030")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
