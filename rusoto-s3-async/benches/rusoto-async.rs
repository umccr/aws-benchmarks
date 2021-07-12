use rusoto_async::MB;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use rusoto_async::download::do_download;
use rusoto_async::upload::do_upload;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    aws_objects: usize,
    aws_bucket: String,
    aws_prefix_key: String,
    aws_region: String,
}

fn transfers(c: &mut Criterion)  {
    let config = envy::from_env::<Config>().expect("Something went wrong!");

    let mut group = c.benchmark_group("transfers");
    for obj_size in [2 * MB, 8 * MB, 16 * MB, 32 * MB, 64 * MB].iter() {
        group.throughput(Throughput::Bytes(*obj_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(obj_size), obj_size, |b, &obj_size| {
            b.iter(|| do_upload(black_box(obj_size),
                                black_box(config.aws_bucket.clone()),
                                   black_box(config.aws_prefix_key.clone()),
                                   black_box(config.aws_region.clone())));

            b.iter(|| do_download(black_box(config.aws_bucket.clone()),
                                   black_box(config.aws_prefix_key.clone()),
                                   black_box(config.aws_region.clone())));
        });

    }
    group.finish();
}

criterion_group!(benches, transfers);
criterion_main!(benches);