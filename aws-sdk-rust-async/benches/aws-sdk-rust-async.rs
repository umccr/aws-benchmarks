use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aws_sdk_bench_async::downloads::do_downloads;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("20x s3 download", |b| b.iter(|| do_downloads(black_box("bucket".to_string()),
                                                                                    black_box("key".to_string()),
                                                                                        black_box("region".to_string()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);