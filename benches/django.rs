use auto_lsp_core::{
    ast::{
        BuildCodeActions, BuildCodeLenses, BuildDocumentSymbols, BuildInlayHints,
        BuildSemanticTokens,
    },
    document_symbols_builder::DocumentSymbolsBuilder,
    salsa::{db::BaseDatabase, tracked::get_ast},
    semantic_tokens_builder::SemanticTokensBuilder,
};
use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::Url;
use texter::core::text;

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

    c.bench_function("parse_django_file", move |b| {
        b.iter(|| {
            let file = db.get_file(&uri).unwrap();
            let ast = get_ast(&db, file).clone().into_inner();
            assert!(ast.ast.is_some())
        });
    });
}

pub fn lsp_requests(c: &mut Criterion) {
    let mut db = auto_lsp_core::salsa::db::BaseDb::default();

    let uri = Url::parse("file:///test.py").unwrap();
    let text = text::Text::new(DJANGO.to_string());
    db.add_file_from_texter(
        auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
        &uri,
        text,
    )
    .unwrap();

    let file = db.get_file(&uri).unwrap();

    let ast = get_ast(&db, file).clone().into_inner();
    let document = file.document(&db).read();

    c.bench_function("code_actions", |b| {
        b.iter(|| {
            let mut acc = vec![];
            ast.ast
                .as_ref()
                .unwrap()
                .build_code_actions(&document, &mut acc);
            assert_eq!(acc.len(), 2);
        });
    });

    c.bench_function("code_lenses", |b| {
        b.iter(|| {
            let mut acc = vec![];
            ast.ast
                .as_ref()
                .unwrap()
                .build_code_lenses(&document, &mut acc);
            assert_eq!(acc.len(), 2);
        });
    });

    c.bench_function("document_symbols", |b| {
        b.iter(|| {
            let mut acc = DocumentSymbolsBuilder::default();
            ast.ast
                .as_ref()
                .unwrap()
                .build_document_symbols(&document, &mut acc);
            assert_eq!(acc.finalize().len(), 2);
        });
    });

    c.bench_function("inlay_hints", |b| {
        b.iter(|| {
            let mut acc = vec![];
            ast.ast
                .as_ref()
                .unwrap()
                .build_inlay_hints(&document, &mut acc);
            assert_eq!(acc.len(), 2);
        });
    });

    c.bench_function("semantic_tokens", |b| {
        b.iter(|| {
            let mut acc = SemanticTokensBuilder::new("".into());
            ast.ast
                .as_ref()
                .unwrap()
                .build_semantic_tokens(&document, &mut acc);
            assert_eq!(acc.build().data.len(), 2);
        });
    });
}

criterion_group!(benches, parse, lsp_requests);
criterion_main!(benches);
