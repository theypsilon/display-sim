use criterion::{criterion_group, criterion_main, Criterion};
use screen_sim_testing::fake::FakeVideoInput;

fn bench_simulation_run_60_iterations(c: &mut Criterion) {
    c.bench_function("iterate 60 times", |b| b.iter(|| FakeVideoInput::default().iterate_times(60)));
}

criterion_group!(benches, bench_simulation_run_60_iterations);
criterion_main!(benches);
