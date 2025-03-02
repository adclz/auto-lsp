use criterion::{criterion_group, criterion_main, Criterion};

mod django;
#[cfg(target_family = "unix")]
use pprof::criterion::{Output, PProfProfiler};

pub static DJANGO: &'static str = include_str!("django.py");

pub fn parse(c: &mut Criterion) {
    c.bench_function("parse_flamegraph", move |b| {
        b.iter(|| {
            let uri = lsp_types::Url::parse("file:///test.py").unwrap();
            let root = auto_lsp_core::root::Root::from_utf8(
                auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
                uri,
                DJANGO.to_string().clone(),
            )
            .unwrap();
            assert!(root.0.ast.is_some())
        });
    });
}

#[cfg(target_family = "unix")]
criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = parse
);

#[cfg(target_family = "windows")]
criterion_group!(benches, parse);
criterion_main!(benches);
