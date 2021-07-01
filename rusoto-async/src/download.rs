use crate::Error;

use rusoto_core::{Region};
use rusoto_s3::{GetObjectRequest, S3Client, S3};

pub async fn do_download(
    region: Region,
    bucket: &str,
    key: &str,
) -> Result<usize, Error> {
    let client = S3Client::new(region);

    let bucket = bucket.to_string();

    let request = GetObjectRequest {
        bucket: bucket.clone(),
        key: key.to_string(),
        ..Default::default()
    };
    let response = client
        .get_object(request)
        .await
        .expect("Error getting object");
    let body = response.body;
    body.unwrap().into_async_read()
}