use crate::Error;
use crate::MB;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}
pub fn do_upload (
    obj_size: usize,
    bucket: String,
    key_prefix: String,
    region: String
) -> Result<usize, Error>{
    let mut s3_obj_buffer = vec![0; obj_size];
    let aws = Storage {
        region: region.parse().unwrap(),
        credentials: Credentials::default()?,
        bucket,
    };

    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    let key = format!("{}/{}-{}MB", key_prefix, "rust-s3-blocking", obj_size/MB);

    //bucket.put_object_stream_blocking(key, &mut s3_obj_buffer);
    bucket.put_object_blocking(key, &mut s3_obj_buffer)?;

    return Ok(s3_obj_buffer.len());
}