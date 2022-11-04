use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maxine_lib::transforms::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("deadband 0.5", |b| b.iter(|| deadband(black_box(0.5))))
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
