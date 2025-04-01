use auto_lsp_core::salsa::{
    db::{BaseDatabase, BaseDb},
    tracked::get_ast,
};
use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::Url;
use texter::core::text;

pub static DJANGO: &str = include_str!("django.py");

pub fn parse_rayon(c: &mut Criterion) {
    let mut db = BaseDb::default();

    let uri = Url::parse("file:///test.py").unwrap();
    let text = text::Text::new(DJANGO.to_string());
    db.add_file_from_texter(
        auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
        &uri,
        text,
    )
    .unwrap();

    c.bench_function("parse_django_file_with_rayon", move |b| {
        b.iter(|| {
            let file = db.get_file(&uri).unwrap();
            let ast = get_ast(&db, file).clone().into_inner();
            assert!(ast.ast.is_some())
        });
    });
}
criterion_group!(benches, parse_rayon);
criterion_main!(benches);
