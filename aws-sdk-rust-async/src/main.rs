//use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use aws_sdk_bench_async::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    aws_objects: usize,
    aws_bucket: String,
    aws_prefix_key: String,
    aws_region: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = envy::from_env::<Config>().expect("Something went wrong!");

    do_upload(config.aws_objects, 1024*1024, config.aws_bucket, config.aws_prefix_key, config.aws_region).await?;
    //do_download(bucket.clone(), key.clone(), region.clone()).await?;

    Ok(())
}
