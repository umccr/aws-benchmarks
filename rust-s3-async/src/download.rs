use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

pub async fn do_download(region: Region, bucket: String, key: String) -> Result<usize, Error> {
    let mut s3_obj_buffer = Cursor::new(Vec::new());
    let aws = Storage {
        region,
        credentials: Credentials::default()?,
        bucket,
    };

    let bucket = Bucket::new(&aws.bucket, aws.region, aws.credentials)?;
    bucket.get_object_stream(key, &mut s3_obj_buffer).await?;
    return Ok(s3_obj_buffer);
}