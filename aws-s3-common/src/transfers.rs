use async_trait::async_trait;
use crate::Error;

#[async_trait]
trait AsyncTransfers {
    fn async_download(obj_size: usize, bucket: String, key: String, region: String) -> Result<usize, Error>;
    fn async_upload(obj_size: usize, bucket: String, key: String, region: String) -> Result<usize, Error>;
}

trait BlockingTransfers {
    fn blocking_download(obj_size: usize, bucket: String, key: String, region: String) -> Result<usize, Error>;
    fn blocking_upload(obj_size: usize, bucket: String, key: String, region: String) -> Result<usize, Error>;
}