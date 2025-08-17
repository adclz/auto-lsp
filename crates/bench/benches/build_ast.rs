extern crate ast_python;
extern crate auto_lsp;
extern crate divan;

use ast_python::db::PYTHON_PARSERS;
use auto_lsp::{
    core::errors::ParseErrorAccumulator,
    default::db::{file::File, tracked::get_ast, BaseDatabase, BaseDb, FileManager},
    lsp_types::{
        DidChangeTextDocumentParams, Position, Range, TextDocumentContentChangeEvent, Url,
        VersionedTextDocumentIdentifier,
    },
};
use divan::{AllocProfiler, Bencher};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

static DJANGO: &'static str = include_str!("./django.py");

fn main() {
    divan::main();
}

#[divan::bench]
fn parse_ts(bencher: Bencher) {
    let url = Url::parse("file:///django.py").expect("Failed to parse URL");
    let mut db = BaseDb::default();
    let file = File::from_string()
        .db(&db)
        .source(DJANGO.to_string())
        .url(&url)
        .parsers(
            PYTHON_PARSERS
                .get("python")
                .expect("Python parser not found"),
        )
        .call()
        .expect("Failed to create file");

    db.add_file(file).expect("Failed to add file");

    bencher
        //.counter(BytesCount::of_str(file.document(&db).as_str()))
        .bench_local(|| get_ast(&db, file))
}

#[divan::bench]
fn build_ast(bencher: Bencher) {
    let url = Url::parse("file:///django.py").expect("Failed to parse URL");
    let mut db = BaseDb::default();
    let file = File::from_string()
        .db(&db)
        .source(DJANGO.to_string())
        .url(&url)
        .parsers(
            PYTHON_PARSERS
                .get("python")
                .expect("Python parser not found"),
        )
        .call()
        .expect("Failed to create file");

    db.add_file(file).expect("Failed to add file");

    bencher
        //.counter(BytesCount::of_str(file.document(&db).as_str()))
        .bench_local(|| get_ast(&db, file));

    let errors = get_ast::accumulated::<ParseErrorAccumulator>(&db, file);
    assert!(errors.is_empty(), "Expected no errors, found: {:?}", errors);
}

#[divan::bench]
fn reparse(bencher: Bencher) {
    let url = Url::parse("file:///django.py").expect("Failed to parse URL");
    let mut db = BaseDb::default();
    let file = File::from_string()
        .db(&db)
        .source(DJANGO.to_string())
        .url(&url)
        .parsers(
            PYTHON_PARSERS
                .get("python")
                .expect("Python parser not found"),
        )
        .call()
        .expect("Failed to create file");

    db.add_file(file).expect("Failed to add file");

    let file = db
        .get_file(&Url::parse("file:///django.py").unwrap())
        .unwrap();

    let change_event = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: file.url(&db).clone(),
            version: 0,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 2486,
                    character: 0,
                },
                end: Position {
                    line: 2486,
                    character: 0,
                },
            }),
            range_length: None,
            text: "# Comment\n".to_string(),
        }],
    };

    bencher
        //.counter(BytesCount::of_str(file.document(&db).as_str()))
        .bench_local(|| file.update_edit(&mut db, &change_event));

    let errors = get_ast::accumulated::<ParseErrorAccumulator>(&db, file);
    assert!(errors.is_empty(), "Expected no errors, found: {:?}", errors);
}
