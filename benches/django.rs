use criterion::Criterion;

pub fn parse_django(_c: &mut Criterion) {
    let _text = include_str!("django.py").to_string();
    #[cfg(feature = "python")]
    _c.bench_function("parse_django", move |b| {
        b.iter(|| {
            let uri = lsp_types::Url::parse("file:///test.py").unwrap();
            let workspace = auto_lsp_core::workspace::Workspace::from_utf8(
                auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
                uri,
                _text.clone(),
            );
            workspace
        });
    });
}
