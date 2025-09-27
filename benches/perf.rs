use criterion::{criterion_group, criterion_main, Criterion};
use radiation_sim::physics::radiative_power;


fn bench_power(c: &mut Criterion) {
    c.bench_function("radiative_power 1000K 1m2", |b| {
        b.iter(|| radiative_power(1000.0, 1.0, 1.0));
    });
}


criterion_group!(benches, bench_power);
criterion_main!(benches);