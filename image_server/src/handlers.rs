use std::io::Write;

use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use image::ImageEncoder;
use tracing::info;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use base64::prelude::*;
use serde::Serialize;

use nifti_decoder::decode_nifti;

pub async fn fetch_image(Path(image_name): Path<String>) -> impl IntoResponse {
    info!("Image to fetch:::'{}'", image_name);

    let bucket_name = "ixi-test-bucket";
    let region = Region::Custom {
        region: "".to_owned(),
        endpoint: "http://127.0.0.1:9000".to_owned(),
    };

    let creds = Credentials {
        access_key: Some("minioadmin".to_owned()),
        secret_key: Some("minioadmin".to_owned()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

    let response = Bucket::list_buckets(region.clone(), creds.clone())
        .await
        .unwrap();
    let found_buckets = response.bucket_names().collect::<Vec<String>>();
    println!("found buckets: {:#?}", found_buckets);

    let bucket = Bucket::new(bucket_name, region.clone(), creds.clone())
        .unwrap()
        .with_path_style();

    let response_data = bucket.get_object(image_name).await.unwrap();
    let image_data = response_data.to_vec();

    let (header, volume) = decode_nifti(image_data);

    let size_x = header.dimensions[0] as u32;
    let size_y = header.dimensions[1] as u32;
    let size_z = header.dimensions[2] as u32;

    let mut slices = Vec::with_capacity(size_z as usize);

    for i in 0..(size_z - 1) {
        let start_idx = (i * size_x * size_y) as usize;
        let end_idx = ((i + 1) * size_x * size_y) as usize;

        let raw_pix_data = &volume[start_idx..end_idx];

        let color = image::ExtendedColorType::L8;
        let mut encoded_image = Vec::new();

        image::codecs::png::PngEncoder::new(encoded_image.by_ref())
            .write_image(raw_pix_data, size_x, size_y, color)
            .expect("error encoding pixels as PNG");

        let base64_png = BASE64_STANDARD.encode(encoded_image);
        slices.push(base64_png);
    }

    // TODO: Tale out CORS-Header for PROD
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    (headers, Json(ImageData { slices }))
}

#[derive(Serialize, Debug)]
pub struct ImageData {
    pub slices: Vec<String>,
}
