use auto_lsp_core::salsa::{db::BaseDatabase, tracked::get_ast};
use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::Url;
use texter::core::text;

mod django;
#[cfg(target_family = "unix")]
use pprof::criterion::{Output, PProfProfiler};

pub static DJANGO: &str = include_str!("django.py");

pub fn parse(c: &mut Criterion) {
    let mut db = auto_lsp_core::salsa::db::BaseDb::default();

    let uri = Url::parse("file:///test.py").unwrap();
    let text = text::Text::new(DJANGO.to_string());

    db.add_file_from_texter(
        auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
        &uri,
        text,
    )
    .unwrap();
    c.bench_function("parse_flamegraph", move |b| {
        b.iter(|| {
            let file = db.get_file(&uri).unwrap();
            let ast = get_ast(&db, file).clone().into_inner();
            assert!(ast.ast.is_some())
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
