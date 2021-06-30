use aws_sdk_s3 as s3;
use s3::Region;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;
use aws_sdk_s3::ByteStream;

pub async fn do_upload(obj_number: usize, obj_size: usize, bucket: String, key: String, region: String) -> Result<(), aws_sdk_s3::Error> {
    // XXX: Logs/tracer might affect perf against the other two impls?
    SubscriberBuilder::default()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let conf = s3::Config::builder().region(Region::new(region)).build();
    let client = s3::Client::from_conf(conf);

    for i in 0..obj_number {
        let body = ByteStream::from(vec![0; obj_size]); // XXX: All 0's appropriate?
                                                                         // XXX: Affects benchmark inside loop, improve.

        client.put_object()
            .bucket(bucket.to_string())
            .key(format!("{}-{}", key, i))
            .body(body)
            .send()
            .await?;
    };

    Ok(())
}