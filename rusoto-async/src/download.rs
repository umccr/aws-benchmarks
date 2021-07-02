use crate::Error;

use rusoto_s3::{GetObjectRequest, S3Client, S3};

//use tokio::io::read_to_end;
//use tokio::io::{self, read_to_end};
use tokio::io::AsyncReadExt;
//use tokio::io::util::async_read_ext::AsyncReadExt;

pub async fn do_download(
    region: String,
    bucket: String,
    key: String,
) -> Result<usize, Error> {
    let client = S3Client::new(region.parse().unwrap());

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
    let mut buffer = Vec::new();
    body.unwrap().into_async_read().read_to_end(&mut buffer).await?;
    Ok(buffer.len())
}