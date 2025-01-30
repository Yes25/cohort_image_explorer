use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;

use nifti_decoder::decode_nifti;

mod fetch_image;
use fetch_image::{build_base64_image_vec, ImageData};

mod s3_utils;
use s3_utils::{build_filename_list, get_bucket, ObjectList};

pub async fn fetch_image(Path(image_name): Path<String>) -> impl IntoResponse {
    info!("Image to fetch:::'{}'", image_name);

    let bucket = get_bucket("ixi-test-bucket", "minioadmin", "minioadmin");

    let response_data = bucket.get_object(image_name).await.unwrap();
    let image_data = response_data.to_vec();

    let (header, volume) = decode_nifti(image_data);

    let slices = build_base64_image_vec(&header, volume);

    // TODO: Tale out CORS-Header for PROD
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    (headers, Json(ImageData { slices }))
}

pub async fn fetch_bucket_content(Path(bucket_name): Path<String>) -> impl IntoResponse {
    let bucket = get_bucket(&bucket_name, "minioadmin", "minioadmin");

    let results = bucket
        .list("".to_string(), Some("".to_string()))
        .await
        .unwrap();

    let bucket_contents = build_filename_list(results);

    // TODO: Tale out CORS-Header for PROD
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    (headers, Json(ObjectList { bucket_contents }))
}
