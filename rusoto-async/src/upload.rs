use rusoto_core::{ByteStream, Region};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub async fn do_upload(
    file_size: usize,
    num_files: usize,
    region: Region,
    bucket: &str,
    key_prefix: &str,
) {
    let client = S3Client::new(region);
    for i in 0..num_files {
        let key = format!("{}-{}", key_prefix, i);
        let data = vec![0; file_size];
        let request = PutObjectRequest {
            key,
            body: Some(ByteStream::from(data)),
            bucket: bucket.to_string(),
            content_length: Some(file_size as i64),
            ..Default::default()
        };
        client
            .put_object(request)
            .await
            .expect("Error uploading file");
    }
}