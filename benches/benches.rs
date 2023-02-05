use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use glam::Vec3A;
use phy::{
    constraints::{Distance, Fixed},
    particles::Particle,
    simulate_iteration,
};

fn constraints(c: &mut Criterion) {
    let mut group = c.benchmark_group("constraints");
    let d_t = Duration::from_secs_f32(1.0 / 60.0);
    for size in 1..=8 {
        let size = size * 50;
        let mut particles = vec![Particle::stationary(Vec3A::ZERO, 1.0)];
        let mut constraints = vec![Fixed::new(Vec3A::ZERO, 0, 0.0).into()];

        for i in 1..=size {
            particles.push(Particle::stationary([0.0, 1.0 * i as f32, 0.0].into(), 1.0));
            constraints.push(Distance::new(1.0, i - 1, i, 0.0).into());
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _size| {
            b.iter(|| simulate_iteration(d_t, 40, &mut particles, &mut constraints));
        });
    }
    group.finish();
}

fn substeps(c: &mut Criterion) {
    let mut group = c.benchmark_group("substeps");
    let mut particles = vec![Particle::stationary(Vec3A::ZERO, 1.0)];
    let mut constraints = vec![Fixed::new(Vec3A::ZERO, 0, 0.0).into()];
    let d_t = Duration::from_secs_f32(1.0 / 60.0);
    for i in 1..=100 {
        particles.push(Particle::stationary([0.0, 1.0 * i as f32, 0.0].into(), 1.0));
        constraints.push(Distance::new(1.0, i - 1, i, 0.0).into());
    }

    for n in 1..=6 {
        let n = n * 50;
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| simulate_iteration(d_t, n, &mut particles, &mut constraints));
        });
    }
    group.finish();
}

criterion_group!(benches, constraints, substeps);
criterion_main!(benches);
