use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::serde_types::ListBucketResult;
use serde::Serialize;

pub fn get_bucket(bucket_name: &str, access_key: &str, secret_key: &str) -> Box<Bucket> {
    let region = Region::Custom {
        region: "".to_owned(),
        endpoint: "http://127.0.0.1:9000".to_owned(),
    };

    let creds = Credentials {
        access_key: Some(access_key.to_owned()),
        secret_key: Some(secret_key.to_owned()),
        security_token: None,
        session_token: None,
        expiration: None,
    };

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
