/* Copyright (c) 2019-2022 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use criterion::{criterion_group, criterion_main, Criterion};
use display_sim_testing::fake::FakeVideoInput;

fn bench_simulation_run_60_iterations(c: &mut Criterion) {
    c.bench_function("iterate 60 times", |b| b.iter(|| FakeVideoInput::default().iterate_times(60)));
}

criterion_group!(benches, bench_simulation_run_60_iterations);
criterion_main!(benches);
