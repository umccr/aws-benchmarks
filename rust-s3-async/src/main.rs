use rust_s3_async::MB;
use rust_s3_async::Error;

use rust_s3_async::download::do_download;

use envy;
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
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB] {
        do_download(config.aws_bucket.clone(),
                    config.aws_prefix_key.clone(),
                    config.aws_region.clone()).await?;
    }
    Ok(())
}