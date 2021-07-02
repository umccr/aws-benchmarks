use rust_s3_blocking::Error;
use serde::Deserialize;

use rust_s3_blocking::download::do_download;

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

    do_download(config.aws_bucket, config.aws_prefix_key, config.aws_region)?;

    Ok(())
}