use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;

use nifti_decoder::decode_nifti;

mod fetch_image;
use fetch_image::{build_base64_image_vec, ImageData};

use s3::bucket::Bucket;
mod s3_utils;
use s3_utils::{
    build_filename_list, get_bucket, get_s3_region_and_creds, get_usr_and_pwd, ObjectList,
};

pub async fn fetch_image(
    headers: HeaderMap,
    Path((bucket_name, image_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let (username, password) = get_usr_and_pwd(headers);
    let bucket = get_bucket(&bucket_name, &username, &password);

    let response_data = bucket.get_object(image_name).await.unwrap();
    let image_data = response_data.to_vec();

    let (header, volume) = decode_nifti(image_data);

    let slices = build_base64_image_vec(&header, volume);

    Json(ImageData { slices })
}

pub async fn fetch_bucket_content(
    headers: HeaderMap,
    Path(bucket_name): Path<String>,
) -> impl IntoResponse {
    let (username, password) = get_usr_and_pwd(headers);
    let bucket = get_bucket(&bucket_name, &username, &password);

    let results = bucket
        .list("".to_string(), Some("".to_string()))
        .await
        .unwrap();

    let bucket_contents = build_filename_list(results);

    Json(ObjectList { bucket_contents })
}

pub async fn fetch_buckets(headers: HeaderMap) -> impl IntoResponse {
    let (username, password) = get_usr_and_pwd(headers);

    let (region, creds) = get_s3_region_and_creds(&username, &password);
    let response = Bucket::list_buckets(region, creds).await.unwrap();
    let found_buckets = response.bucket_names().collect::<Vec<String>>();

    Json(found_buckets)
}
