use std::f64::consts::FRAC_PI_6;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maxine_lib::transforms::*;
use nalgebra::{Isometry2, Vector2};

fn bench_deadbands(c: &mut Criterion) {
    c.bench_function("deadband 0.5", |b| b.iter(|| deadband(black_box(0.5))));
    c.bench_function("deadband 1.", |b| b.iter(|| deadband(black_box(1.))));
    c.bench_function("deadband π/6", |b| {
        b.iter(|| deadband(black_box(FRAC_PI_6)))
    });
}

fn bench_swerve_calculations(c: &mut Criterion) {
    c.bench_function("swerve 1, 1, 0", |b| {
        b.iter(|| swerve(black_box(Isometry2::new(Vector2::new(1., 1.), 0.))))
    });
}

criterion_group!(benches, bench_deadbands, bench_swerve_calculations);
criterion_main!(benches);
