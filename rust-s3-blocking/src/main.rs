use rust_s3_blocking::MB;
use rust_s3_blocking::Error;

use serde::Deserialize;

//use rust_s3_blocking::download::do_download;
use rust_s3_blocking::upload::do_upload;

#[derive(Deserialize, Debug)]
struct Config {
    aws_objects: usize,
    aws_bucket: String,
    aws_prefix_key: String,
    aws_region: String,
}

fn main() -> Result<(), Error> {
    let config = envy::from_env::<Config>().expect("Something went wrong!");
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB] {
        do_upload(obj_size, config.aws_bucket.clone(), config.aws_prefix_key.clone(), config.aws_region.clone())?;
        //do_download(config.aws_bucket.clone(), config.aws_prefix_key.clone(), config.aws_region.clone())?;
    }

    Ok(())
}