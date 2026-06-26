use crate::capabilities::inlay_hints::inlay_hints;
use crate::db::create_python_db;
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::Url;
use auto_lsp::{default::db::BaseDatabase, lsp_types::InlayHintParams};
use rstest::{fixture, rstest};

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass
"#])
}

#[rstest]
fn foo_bar_inlay_hints(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let hints = inlay_hints(
        &foo_bar,
        InlayHintParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: file.url(&foo_bar).clone(),
            },
            range: lsp_types::Range {
                start: lsp_types::Position::new(0, 0),
                end: lsp_types::Position::new(0, 0),
            },
            work_done_progress_params: Default::default(),
        },
    )
    .unwrap()
    .unwrap();

    assert_eq!(hints.len(), 2);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}
