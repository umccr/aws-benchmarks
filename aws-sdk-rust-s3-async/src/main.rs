use aws_sdk_bench_async::MB;

//use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use aws_sdk_bench_async::Error;
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
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB].iter() {
        do_upload(config.aws_objects, *obj_size, config.aws_bucket.clone(), config.aws_prefix_key.clone(), config.aws_region.clone()).await?;
        //do_download(bucket.clone(), key.clone(), region.clone()).await?;
    }
    Ok(())
}
