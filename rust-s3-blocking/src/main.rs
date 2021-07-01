use s3::{creds::Credentials, Bucket};

fn main() {
    do_download(&bucket, &key_prefix, &region)
}
