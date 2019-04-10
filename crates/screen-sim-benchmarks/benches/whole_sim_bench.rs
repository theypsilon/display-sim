use criterion::{criterion_group, criterion_main, Criterion};
use screen_sim_integration_test::iterate;

fn bench_simulation_run_60_iterations(c: &mut Criterion) {
    c.bench_function("iterate", |b| b.iter(|| iterate(60)));
}

criterion_group!(benches, bench_simulation_run_60_iterations);
criterion_main!(benches);
