use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use aws_sdk_bench_async::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long = "obj_number", default_value = "10")]
    obj_number: usize,
    #[structopt(long = "obj_size", default_value = "1024")]
    obj_size: usize,
    #[structopt(long = "bucket", default_value = "abk-test-rusoto-download", env)]
    bucket: String,
    #[structopt(long = "key", default_value = "test-object-8388608", env)]
    key: String,
    #[structopt(long = "aws_region", default_value = "ap-southeast-2", env)]
    region: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let Args {
        obj_number,
        obj_size,
        bucket,
        key,
        region,
    } = Args::from_args();

    do_upload(obj_number, obj_size, bucket.clone(), key.clone(), region.clone()).await?;
    do_download(bucket.clone(), key.clone(), region.clone()).await?;

    Ok(())
}
