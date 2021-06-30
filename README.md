# Benchmarks

WIP WIP WIP

Runs [criterion-rs][criterion-rs] benchmarks against AWS (S3) service(s).

```
$ cargo install cargo-criterion
$ cargo bench
```

TODO:

* [ ] Figure out how to pass arguments to the function inside the Bencher, currently they are static inside a black_box
* [ ] Move to [.to_async()](https://bheisler.github.io/criterion.rs/criterion/struct.Bencher.html#method.to_async) and/or
[iai](https://github.com/bheisler/iai).
* [ ] Port rust-s3 code here.
* [ ] Port rust-s3-async code here.
* [ ] Port rusoto code here.
* [ ] Port Go code here.
* [ ] Implement Python Boto3.

[criterion-rs]: https://crates.io/crates/criterion
