use aws_sdk_bench_async::MB;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long = "obj_size", default_value = "1024")]
    obj_size: usize,
    #[structopt(long = "bucket", default_value = "abk-test-rusoto-download", env)]
    bucket: String,
    #[structopt(long = "key", default_value = "test-object-8388608", env)]
    key: String,
    #[structopt(long = "region", default_value = "ap-southeast-2", env)]
    region: String,
}

fn transfers(c: &mut Criterion) {
    let mut group = c.benchmark_group("transfers");
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB].iter() {
        group.throughput(Throughput::Bytes(*obj_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(obj_size), obj_size, |b, &obj_size| {

        b.iter(|| do_upload(black_box(obj_size),
                                   black_box("bucket".to_string()),
                                   black_box("key".to_string()),
                                   black_box("region".to_string())));

        b.iter(|| do_download(black_box(obj_size),
                                   black_box("bucket".to_string()),
                                   black_box("key".to_string()),
                                   black_box("region".to_string())));
        });

    }
    group.finish();
}

criterion_group!(benches, transfers);
criterion_main!(benches);