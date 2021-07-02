use crate::Error;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

pub async fn do_download(region: String, bucket: String, key: String) -> Result<usize, Error> {
    let mut s3_obj_buffer = vec![];
    let aws = Storage {
        region: region.parse().unwrap(),
        credentials: Credentials::default()?,
        bucket,
    };

    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    bucket.get_object_stream(key, &mut s3_obj_buffer).await?;
    return Ok(s3_obj_buffer.len());
}