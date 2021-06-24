use crate::Error;
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use aws_sdk_s3 as s3;
use s3::Region;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;

pub async fn do_downloads(bucket: String, key_prefix: String, region: String) -> Result<usize, Error> {
    // XXX: Logs/tracer might affect perf against the other two impls?
    SubscriberBuilder::default()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let conf = s3::Config::builder()
        .region(Region::new(region))
        .build();

    // XXX: Is it fair to compare blocking impls like the rust-s3 one with this async? Perhaps should implement rust-s3 async too.
    let client = s3::Client::from_conf(conf);

    let resp = client.get_object().bucket(bucket).key(key_prefix).send().await?;
    let data = resp.body.collect().await?;

    Ok(data.into_bytes().len())
}