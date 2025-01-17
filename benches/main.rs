use criterion::criterion_main;
mod django;

criterion_main!(django::benches);
