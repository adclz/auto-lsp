use crate::capabilities::code_lenses::code_lenses;
use crate::db::create_python_db;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{
    CodeLensParams, PartialResultParams, TextDocumentIdentifier, Url, WorkDoneProgressParams,
};
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
fn foo_bar_code_lens(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let results = code_lenses(
        &foo_bar,
        CodeLensParams {
            text_document: TextDocumentIdentifier {
                uri: file.url(&foo_bar).clone(),
            },
            partial_result_params: PartialResultParams {
                partial_result_token: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
    )
    .expect("Failed to build code actions")
    .unwrap();

    assert_eq!(results.len(), 2);

    assert_eq!(results[0].range.start.line, 1);
    assert_eq!(results[0].range.start.character, 4);
    assert_eq!(results[0].range.end.line, 1);
    assert_eq!(results[0].range.end.character, 7);

    assert_eq!(results[1].range.start.line, 4);
    assert_eq!(results[1].range.start.character, 4);
    assert_eq!(results[1].range.end.line, 4);
    assert_eq!(results[1].range.end.character, 7);
}
