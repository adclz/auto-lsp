use criterion::profiler::Profiler;
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::ProfilerGuard;

mod django;
mod perf;
use django::parse_django;

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = parse_django
);
criterion_main!(benches);
