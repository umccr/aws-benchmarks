pub fn do_download(bucket: &str, key: &str, region: &str) -> Result<usize, Error> {
    let bucket = Bucket::new(
            bucket,
            region.parse().unwrap(),
            Credentials::default().unwrap(),
        );

    let data = bucket.get_object_blocking(&key).unwrap();
    data.len()
}