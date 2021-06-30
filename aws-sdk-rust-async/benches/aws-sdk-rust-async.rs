use aws_sdk_bench_async::MB;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use aws_sdk_bench_async::download::do_download;
use aws_sdk_bench_async::upload::do_upload;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Config {
    AWS_BENCH_OBJECT_NUM: usize,
    AWS_BUCKET: String,
    AWS_KEY: String,
    AWS_REGION: String,
}

fn transfers(c: &mut Criterion)  {
    let config = envy::from_env::<Config>().expect("Something went wrong!");

    let mut group = c.benchmark_group("transfers");
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB].iter() {
        group.throughput(Throughput::Bytes(*obj_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(obj_size), obj_size, |b, &obj_size| {
            b.iter(|| do_upload(black_box(config.AWS_BENCH_OBJECT_NUM),
                                   black_box(obj_size),
                                black_box(config.AWS_BUCKET.clone()),
                                   black_box(config.AWS_KEY.clone()),
                                   black_box(config.AWS_REGION.clone())));

            b.iter(|| do_download(black_box(config.AWS_BUCKET.clone()),
                                   black_box(config.AWS_KEY.clone()),
                                   black_box(config.AWS_REGION.clone())));
        });

    }
    group.finish();
}

criterion_group!(benches, transfers);
criterion_main!(benches);