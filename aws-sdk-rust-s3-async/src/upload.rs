use crate::MB;

use aws_sdk_s3::ByteStream;
use aws_sdk_s3 as s3;
use s3::Region;

pub async fn do_upload(obj_size: usize, bucket: String, key: String, region: String) -> Result<usize, aws_sdk_s3::Error> {
    let conf = s3::Config::builder().region(Region::new(region)).build();
    let client = s3::Client::from_conf(conf);


    let body = ByteStream::from(vec![0; obj_size]); // XXX: All 0's appropriate?
                                                                     // XXX: Affects benchmark inside loop, improve.

    client.put_object()
        .bucket(bucket.to_string())
        .key(format!("{}/{}-{}MB", key, "aws-sdk-rust-s3-async".to_string(), obj_size/MB))
        .body(body)
        .send()
        .await?;

    // XXX: Perhaps should return the size reported by the lib instead?
    Ok(obj_size)
}