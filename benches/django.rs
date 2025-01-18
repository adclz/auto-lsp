#[cfg(feature = "python_test")]
use auto_lsp::python_workspace::create_python_workspace;
use criterion::{criterion_group, Criterion};
use lsp_types::Url;

use auto_lsp::{self as auto_lsp};

fn parse_django(c: &mut Criterion) {
    let text = include_str!("django.py").to_string();
    #[cfg(feature = "python_test")]
    c.bench_function("parse_django", move |b| {
        b.iter(|| {
            let uri = Url::parse("file:///test.py").unwrap();
            let workspace = create_python_workspace(uri, text.clone());
            workspace
        });
    });
}

criterion_group!(benches, parse_django);
