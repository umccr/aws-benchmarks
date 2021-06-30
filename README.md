# AWS Benchmarks

WIP WIP WIP

Runs [criterion-rs][criterion-rs] benchmarks against AWS (S3) service(s).

Inspired by [adamrk's S3 download comparison](https://github.com/adamrk/s3-download-comparison) 
and aims to use Criterion for (statistically) rigourous benchmarking.

# Quickstart

```
$ cargo install cargo-criterion
$ export AWS_REGION="ap-southeast-2"
$ export AWS_BUCKET="umccr-test-data-dev"
$ export AWS_PREFIX_KEY="benchmarks"
$ export AWS_OBJECTS=10
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
