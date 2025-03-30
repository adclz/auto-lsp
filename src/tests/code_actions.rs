use auto_lsp_core::{ast::BuildCodeActions, salsa::db::WorkspaceDatabase};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;

#[fixture]
fn foo_bar() -> impl WorkspaceDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#])
}

#[rstest]
fn foo_bar_code_actions(foo_bar: impl WorkspaceDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = file.get_ast(&foo_bar).clone().into_inner();

    let ast = root.ast.as_ref().unwrap();

    let mut code_actions = vec![];
    ast.build_code_actions(&document, &mut code_actions);

    assert_eq!(code_actions.len(), 2);

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &code_actions[0] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &code_actions[1] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }
}
