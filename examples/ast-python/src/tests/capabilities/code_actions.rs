use crate::capabilities::code_actions::code_actions;
use crate::db::create_python_db;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{self, WorkDoneProgressParams};
use auto_lsp::lsp_types::{
    CodeActionContext, CodeActionParams, PartialResultParams, TextDocumentIdentifier, Url,
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
fn foo_bar_code_actions(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let results = code_actions(
        &foo_bar,
        CodeActionParams {
            text_document: TextDocumentIdentifier {
                uri: file.url(&foo_bar).clone(),
            },
            range: lsp_types::Range {
                start: lsp_types::Position::new(0, 0),
                end: lsp_types::Position::new(0, 0),
            },
            context: CodeActionContext {
                diagnostics: vec![],
                only: None,
                trigger_kind: None,
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

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &results[0] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &results[1] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }
}
