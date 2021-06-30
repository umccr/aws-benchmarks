use aws_sdk_bench_async::MB;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long = "obj_number", default_value = "10")]
    obj_number: usize,
    #[structopt(long = "obj_size", default_value = "1024")]
    bucket: String,
    #[structopt(long = "key", default_value = "test-object-8388608", env)]
    key: String,
    #[structopt(long = "aws_region", default_value = "ap-southeast-2", env)]
    region: String,
}

fn transfers(c: &mut Criterion) {
    let Args {
        obj_number,
        bucket,
        key,
        region,
    } = Args::from_args();

    let mut group = c.benchmark_group("transfers");
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB].iter() {
        group.throughput(Throughput::Bytes(*obj_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(obj_size), obj_size, |b, &obj_size| {
            b.iter(|| do_upload(black_box(obj_number),
                                   black_box(obj_size),
                                black_box(bucket.clone()),
                                   black_box(key.clone()),
                                   black_box(region.clone())));

            b.iter(|| do_download(black_box(bucket.clone()),
                                   black_box(key.clone()),
                                   black_box(region.clone())));
        });

    }
    group.finish();
}

criterion_group!(benches, transfers);
criterion_main!(benches);