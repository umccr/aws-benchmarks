use crate::Error;

use rusoto_s3::{GetObjectRequest, S3Client};
use tokio::runtime::Runtime;

pub fn do_download(
    region: String,
    bucket: String,
    key: String,
) -> Result<usize, Error> {
    let mut runtime = Runtime::new()?;
    let client = S3Client::new(region.parse().unwrap());

    let bucket = bucket.to_string();

    let obj_req = GetObjectRequest {
        bucket: bucket.clone(),
        key: key.to_string(),
        ..Default::default()
    };
    let result = runtime.block_on(obj_req);

    result?.body?.into_blocking_read()
}