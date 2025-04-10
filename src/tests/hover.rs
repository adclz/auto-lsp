use std::ops::Deref;

use crate::core::ast::GetHover;
use crate::python::ast::{CompoundStatement, Statement};
use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::salsa::tracked::get_ast;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;
use super::python_workspace::ast::Module;

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
fn foo_bar_hover(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.unwrap();

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let foo = module.statements[0].read();
    if let Statement::Compound(CompoundStatement::Function(foo)) = foo.deref() {
        let foo_name = foo.name.read();

        let foo_hover = foo_name.get_hover(&document).unwrap();

        assert_eq!(
            foo_hover.unwrap().contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "hover foo".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }

    let bar = module.statements[1].read();

    if let Statement::Compound(CompoundStatement::Function(foo)) = bar.deref() {
        let bar_name = foo.name.read();

        let bar_hover = bar_name.get_hover(&document).unwrap();

        assert_eq!(
            bar_hover.unwrap().contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "hover bar".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }
}
