use auto_lsp_core::{ast::BuildCodeActions, workspace::Workspace};
use rstest::{fixture, rstest};

use super::python_utils::{create_python_workspace, get_python_file};

#[fixture]
fn foo_bar() -> Workspace {
    create_python_workspace(
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#,
    )
}

#[rstest]
fn foo_bar_code_actions(foo_bar: Workspace) {
    let (root, document) = get_python_file(&foo_bar);
    let ast = root.ast.as_ref().unwrap();

    let mut code_actions = vec![];
    ast.build_code_actions(document, &mut code_actions);

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
