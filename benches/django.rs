use auto_lsp::core::root::Root;
use auto_lsp_core::{
    ast::{
        BuildCodeActions, BuildCodeLenses, BuildDocumentSymbols, BuildInlayHints,
        BuildSemanticTokens,
    },
    document_symbols_builder::DocumentSymbolsBuilder,
    semantic_tokens_builder::SemanticTokensBuilder,
    workspace::Workspace,
};
use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::Url;

pub static DJANGO: &str = include_str!("django.py");

pub fn parse(c: &mut Criterion) {
    c.bench_function("parse_django_file", move |b| {
        b.iter(|| {
            let uri = Url::parse("file:///test.py").unwrap();
            let root = Root::from_utf8(
                auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
                uri,
                DJANGO.to_string().clone(),
            )
            .unwrap();
            assert!(root.0.ast.is_some())
        });
    });
}

pub fn lsp_requests(c: &mut Criterion) {
    let mut workspace = Workspace::default();

    let uri = Url::parse("file:///test.py").unwrap();
    let root = Root::from_utf8(
        auto_lsp::python::PYTHON_PARSERS.get("python").unwrap(),
        uri.clone(),
        DJANGO.to_string().clone(),
    )
    .unwrap();

    workspace.roots.insert(uri.clone(), root);

    c.bench_function("code_actions", |b| {
        b.iter(|| {
            let (root, document) = workspace.roots.get(&uri).as_ref().unwrap();
            let mut acc = vec![];
            root.ast
                .as_ref()
                .unwrap()
                .build_code_actions(document, &mut acc);
            assert_eq!(acc.len(), 4);
        });
    });

    c.bench_function("code_lenses", |b| {
        b.iter(|| {
            let (root, document) = workspace.roots.get(&uri).as_ref().unwrap();
            let mut acc = vec![];
            root.ast
                .as_ref()
                .unwrap()
                .build_code_lenses(document, &mut acc);
            assert_eq!(acc.len(), 4);
        });
    });

    c.bench_function("document_symbols", |b| {
        b.iter(|| {
            let (root, document) = workspace.roots.get(&uri).as_ref().unwrap();
            let mut acc = DocumentSymbolsBuilder::default();
            root.ast
                .as_ref()
                .unwrap()
                .build_document_symbols(document, &mut acc);
            assert_eq!(acc.finalize().len(), 4);
        });
    });

    c.bench_function("inlay_hints", |b| {
        b.iter(|| {
            let (root, document) = workspace.roots.get(&uri).as_ref().unwrap();
            let mut acc = vec![];
            root.ast
                .as_ref()
                .unwrap()
                .build_inlay_hints(document, &mut acc);
            assert_eq!(acc.len(), 4);
        });
    });

    c.bench_function("semantic_tokens", |b| {
        b.iter(|| {
            let (root, document) = workspace.roots.get(&uri).as_ref().unwrap();
            let mut acc = SemanticTokensBuilder::new("".into());
            root.ast
                .as_ref()
                .unwrap()
                .build_semantic_tokens(document, &mut acc);
            assert_eq!(acc.build().data.len(), 4);
        });
    });
}

criterion_group!(benches, parse, lsp_requests);
criterion_main!(benches);
