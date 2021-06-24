use aws_sdk_bench_async::Error;
use aws_sdk_bench_async::downloads::do_downloads;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long = "bucket", default_value = "abk-test-rusoto-download")]
    bucket: String,
    #[structopt(long = "key-prefix", default_value = "test-object-8388608")]
    key_prefix: String,
    #[structopt(long = "region", default_value = "us-east-1")]
    region: String,
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let Args {
        bucket,
        key_prefix,
        region,
    } = Args::from_args();

    do_downloads(bucket, key_prefix, region).await?;

    Ok(())
}
