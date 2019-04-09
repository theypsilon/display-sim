use criterion::{Criterion, criterion_group, criterion_main};
use screen_sim_native::iterate;

fn bench_simulation_run_60_iterations(c: &mut Criterion) {
    c.bench_function("iterate", |b| b.iter(|| iterate(60)));
}

criterion_group!(benches, bench_simulation_run_60_iterations);
criterion_main!(benches);