#[cfg(feature = "python")]
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp_core::workspace::Workspace;
use criterion::{criterion_group, Criterion};
use lsp_types::Url;

use auto_lsp::{self as auto_lsp};

pub fn parse_django(c: &mut Criterion) {
    let text = include_str!("django.py").to_string();
    #[cfg(feature = "python")]
    c.bench_function("parse_django", move |b| {
        b.iter(|| {
            let uri = Url::parse("file:///test.py").unwrap();
            let workspace =
                Workspace::from_utf8(PYTHON_PARSERS.get("python").unwrap(), uri, text.clone());
            workspace
        });
    });
}
