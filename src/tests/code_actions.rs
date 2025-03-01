use crate::core::document::Document;
use crate::core::root::Root;
use auto_lsp_core::ast::BuildCodeActions;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::*;

#[fixture]
fn foo_bar() -> (Root, Document) {
    Root::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn foo_bar_code_actions(foo_bar: (Root, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let mut code_actions = vec![];
    ast.build_code_actions(document, &mut code_actions);

    assert_eq!(code_actions.len(), 2);

    assert_eq!(code_actions[0].title, "A code action");

    assert_eq!(code_actions[1].title, "A code action");
}
