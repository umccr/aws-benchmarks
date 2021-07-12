use rusoto_async::MB;
use rusoto_async::Error;

use rusoto_async::upload::do_upload;
//use rusoto_async::download::do_download;

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

    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB] {
        do_upload(obj_size, config.aws_bucket.clone(), config.aws_prefix_key.clone(), config.aws_region.clone()).await?;
        //do_download(config.aws_bucket.clone(), config.aws_prefix_key.clone(), config.aws_region.clone()).await?;
    }

    Ok(())
}