use std::ops::Deref;

use crate::core::ast::GetHover;
use crate::core::document::Document;
use crate::core::root::Root;
use crate::python::ast::{CompoundStatement, Statement};
use rstest::{fixture, rstest};

use super::python_utils::create_python_workspace;
use super::python_workspace::ast::Module;

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
fn foo_bar_hover(foo_bar: (Root, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let foo = module.statements[0].read();
    if let Statement::Compound(CompoundStatement::Function(foo)) = foo.deref() {
        let foo_name = foo.name.read();

        let foo_hover = foo_name.get_hover(document).unwrap();

        assert_eq!(
            foo_hover.contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "# foo comment\nhover foo".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }

    let bar = module.statements[1].read();

    if let Statement::Compound(CompoundStatement::Function(foo)) = bar.deref() {
        let bar_name = foo.name.read();

        let bar_hover = bar_name.get_hover(document).unwrap();

        assert_eq!(
            bar_hover.contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "hover bar".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }
}
