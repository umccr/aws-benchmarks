# AWS Benchmarks

WIP WIP WIP

Runs [criterion-rs][criterion-rs] benchmarks against AWS (S3) service(s).

This project is inspired on [adamrk's S3 download comparison](https://github.com/adamrk/s3-download-comparison) 
and aims to use Criterion for more rigourous benchmarking.

# Quickstart

```
$ cargo install cargo-criterion
$ cargo bench
```

TODO:

* [ ] [Upload byte streams instead of creating local files](https://github.com/adamrk/s3-download-comparison/blob/main/rusoto-bench/src/main.rs#L8)
* [ ] Separate benchmarking for uploads vs downloads.
* [ ] Move to [.to_async()](https://bheisler.github.io/criterion.rs/criterion/struct.Bencher.html#method.to_async) and/or
[iai](https://github.com/bheisler/iai).
* [ ] Port rust-s3 code here.
* [ ] Port rust-s3-async code here.
* [ ] Port rusoto code here.
* [ ] Port Go code here.
* [ ] Implement Python Boto3.

[criterion-rs]: https://crates.io/crates/criterion
