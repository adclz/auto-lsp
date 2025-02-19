use criterion::{criterion_group, criterion_main, Criterion};

mod django;
#[cfg(target_family = "unix")]
mod perf;
use django::parse_django;

#[cfg(target_family = "unix")]
criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = parse_django
);
#[cfg(target_family = "windows")]
criterion_group!(benches, parse_django);
criterion_main!(benches);
