use crate::core::ast::BuildCodeLenses;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
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
fn foo_bar_code_lens(foo_bar: impl WorkspaceDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = file.get_ast(&foo_bar).clone().into_inner();

    let ast = root.ast.as_ref().unwrap();

    let mut code_lens = vec![];
    ast.build_code_lenses(&document, &mut code_lens);

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
