use std::ops::Deref;

use crate::core::ast::GetHover;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use crate::python::ast::{CompoundStatement, Statement};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::ast::Module;
use super::python_workspace::PYTHON_PARSERS;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
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
fn foo_bar_hover(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let foo = module.statements[0].read();
    if let Statement::Compound(CompoundStatement::Function(foo)) = foo.deref() {
        let foo_name = foo.name.read();

        let foo_hover = foo_name.get_hover(&document).unwrap();

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

        let bar_hover = bar_name.get_hover(&document).unwrap();

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
