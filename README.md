# AWS Benchmarks

WIP WIP WIP

Runs [criterion-rs][criterion-rs] benchmarks against AWS (S3) service(s).

Inspired by [adamrk's S3 download comparison](https://github.com/adamrk/s3-download-comparison) 
and aims to use Criterion for (statistically) rigourous benchmarking.

# Quickstart

Make sure you have [yawsso][yawsso] handy because, unfortunately, AWS SDK for Rust still [does not support modern
AWS credential providers support (upvote the relevant issues on their tracker please!)][aws-sdk-rust-creds].

```
$ cargo install cargo-criterion
$ export AWS_REGION="ap-southeast-2"
$ export AWS_BUCKET="umccr-temp-dev"
$ export AWS_PREFIX_KEY="benchmarks"
$ export AWS_OBJECTS=10
$ yawsso -p dev -e | source /dev/stdin   # v1 AWS creds required for AWS-SDK-RUST :_(
$ cargo bench
```

TODO:

* [ ] Separate benchmarking for uploads vs downloads.
* [ ] Port rust-s3 code here.
* [ ] Port rust-s3-async code here.
* [ ] Port rusoto code here.
* [ ] Port Go code here.
* [ ] Implement Python Boto3.
* [ ] Move to [.to_async()](https://bheisler.github.io/criterion.rs/criterion/struct.Bencher.html#method.to_async) and/or
  [iai](https://github.com/bheisler/iai).
  

[criterion-rs]: https://crates.io/crates/criterion
[yawsso]: https://github.com/victorskl/yawsso
[aws-sdk-rust-creds]: https://github.com/awslabs/aws-sdk-rust/issues?page=2&q=is%3Aissue+is%3Aopen