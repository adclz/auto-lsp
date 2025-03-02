use criterion::{criterion_group, criterion_main, Criterion};

pub static DJANGO: &'static str = include_str!("django.py");

pub fn parse_rayon(c: &mut Criterion) {
    let _text = include_str!("django.py").to_string();
    c.bench_function("parse_django_file_with_rayon", move |b| {
        b.iter(|| {
            let uri = lsp_types::Url::parse("file:///test.py").unwrap();
            let root = auto_lsp_core::root::Root::from_utf8(
                auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
                uri,
                _text.clone(),
            )
            .unwrap();
            assert!(root.0.ast.is_some())
        });
    });
}
criterion_group!(benches, parse_rayon);
criterion_main!(benches);
