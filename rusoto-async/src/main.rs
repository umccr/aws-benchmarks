use rusoto_core::{ByteStream, Region};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use tokio::{io::AsyncReadExt, sync::Mutex};

use rusoto_async::download::do_download;

#[tokio::main]
async fn main() {
    do_download();
}