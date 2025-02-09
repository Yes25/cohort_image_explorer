use axum::http::HeaderMap;
use base64::prelude::*;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::serde_types::ListBucketResult;
use serde::Serialize;

// static S3_URL: &str = "http://127.0.0.1:9000";
static S3_URL: &str = "http://s3.lake-test.medicsh.de:9000";

pub fn get_s3_region_and_creds(access_key: &str, secret_key: &str) -> (Region, Credentials) {
    (
        Region::Custom {
            region: "".to_owned(),
            endpoint: S3_URL.to_owned(),
        },
        Credentials {
            access_key: Some(access_key.to_owned()),
            secret_key: Some(secret_key.to_owned()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
}

pub fn get_bucket(bucket_name: &str, access_key: &str, secret_key: &str) -> Box<Bucket> {
    let (region, creds) = get_s3_region_and_creds(access_key, secret_key);

    Bucket::new(bucket_name, region.clone(), creds.clone())
        .unwrap()
        .with_path_style()
}

#[derive(Serialize, Debug)]
pub struct ObjectList {
    pub bucket_contents: Vec<String>,
}

pub fn build_filename_list(results: Vec<ListBucketResult>) -> Vec<String> {
    let mut bucket_contents: Vec<String> = Vec::new();
    for result in results {
        for content in result.contents {
            bucket_contents.push(content.key);
        }
    }
    bucket_contents
}

pub fn get_usr_and_pwd(headers: HeaderMap) -> (String, String) {
    let authorization = headers
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let authorization = authorization.replace("Basic ", "");

    let decoded = String::from_utf8(BASE64_STANDARD.decode(authorization).unwrap()).unwrap();

    let split = decoded.split(":").collect::<Vec<&str>>();

    let username = split[0].to_owned();
    let password = split[1].to_owned();
    (username, password)
}
