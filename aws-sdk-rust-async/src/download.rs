use crate::Error;

use aws_sdk_s3 as s3;
use s3::Region;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;

pub async fn do_download(
    bucket: String,
    key: String,
    region: String,
) -> Result<usize, Error> {
    // XXX: Logs/tracer might affect perf against the other two impls?
    SubscriberBuilder::default()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let conf = s3::Config::builder().region(Region::new(region)).build();
    let client = s3::Client::from_conf(conf);

    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    let data = resp.body.collect().await?;

    Ok(data.into_bytes().len())
}
