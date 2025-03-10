use crate::core::document::Document;
use crate::core::root::Root;
use auto_lsp_core::ast::BuildCodeActions;
use rstest::{fixture, rstest};

use super::python_utils::create_python_workspace;

#[fixture]
fn foo_bar() -> (Root, Document) {
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
fn foo_bar_code_actions(foo_bar: (Root, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

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
