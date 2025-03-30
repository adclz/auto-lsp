use crate::core::ast::BuildInlayHints;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;
use super::python_workspace::ast::Module;

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
fn foo_bar_inlay_hints(foo_bar: impl WorkspaceDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = file.get_ast(&foo_bar).clone().into_inner();

    let ast = root.ast.as_ref().unwrap();

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut hints = vec![];
    module.build_inlay_hints(&document, &mut hints);

    assert_eq!(hints.len(), 2);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}
