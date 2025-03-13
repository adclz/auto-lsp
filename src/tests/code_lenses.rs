use crate::core::ast::BuildCodeLenses;
use crate::core::workspace::Workspace;
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
fn foo_bar_code_lens(foo_bar: Workspace) {
    let (root, document) = get_python_file(&foo_bar);
    let ast = root.ast.as_ref().unwrap();

    let mut code_lens = vec![];
    ast.build_code_lenses(document, &mut code_lens);

    assert_eq!(code_lens.len(), 2);

    assert_eq!(code_lens[0].range.start.line, 1);
    assert_eq!(code_lens[0].range.start.character, 4);
    assert_eq!(code_lens[0].range.end.line, 1);
    assert_eq!(code_lens[0].range.end.character, 7);

    assert_eq!(code_lens[1].range.start.line, 4);
    assert_eq!(code_lens[1].range.start.character, 4);
    assert_eq!(code_lens[1].range.end.line, 4);
    assert_eq!(code_lens[1].range.end.character, 7);
}
