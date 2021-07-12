use crate::{MB, Error};

use rusoto_core::{ByteStream};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub async fn do_upload (
    obj_size: usize,
    bucket: String,
    key: String,
    region: String
) -> Result<usize, Error> {
    let client = S3Client::new(region.parse().unwrap());
        let key = format!("{}/{}-{}MB", key, "rusoto-s3-async", obj_size/MB);
        let data = vec![0; obj_size];
        let request = PutObjectRequest {
            key,
            body: Some(ByteStream::from(data)),
            bucket: bucket.clone(),
            content_length: Some(obj_size as i64),
            ..Default::default()
        };
        client
            .put_object(request)
            .await?;

    // XXX: Perhaps should return the size reported by the lib instead?
    Ok(obj_size)
}