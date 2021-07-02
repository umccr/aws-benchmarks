use rusoto_blocking::Error;

use rusoto_blocking::download::do_download;

use serde::Deserialize;
use envy;

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

    do_download(config.aws_bucket, config.aws_prefix_key, config.aws_region).await?;

    Ok(())
}