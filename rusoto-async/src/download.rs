use crate::Error;

use std::sync::Arc;

use rusoto_core::{Region};
use rusoto_s3::{GetObjectRequest, S3Client, S3};


pub async fn do_download(
    region: Region,
    bucket: &str,
    key: &str,
) -> Result<usize, Error> {
    let client = Arc::new(S3Client::new(region));

    let client_copy = Arc::clone(&client);
    let bucket = bucket.to_string();

    let request = GetObjectRequest {
        bucket: bucket.clone(),
        key: key.to_string(),
        ..Default::default()
    };
    let response = client_copy
        .get_object(request)
        .await
        .expect("Error getting object");
    let body = response.body.unwrap();
    let mut reader = body.into_async_read();
    reader.as_bytes().len().await?
}