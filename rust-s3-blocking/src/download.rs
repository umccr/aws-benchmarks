use crate::Error;

use s3::bucket::Bucket;
use s3::creds::Credentials;

pub fn do_download(bucket: String, key: String, region: String) -> Result<usize, Error> {
    let bucket = Bucket::new(
            bucket.as_str(),
            region.parse().unwrap(),
            Credentials::default().unwrap(),
        );

    let (data, _code) = bucket.unwrap().get_object_blocking(&key).unwrap();
    Ok(data.len())
}