use crate::core::ast::BuildCompletionItems;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::{BuildTriggeredCompletionItems, Traverse};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::ast::Module;
use super::python_workspace::*;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass
    i

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn global_completion_items(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    // Module returns globally available completion items
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut completion_items = vec![];
    module.build_completion_items(document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "def ...");

    // Function should do the same
    let function = module.statements[0].read();

    let mut completion_items = vec![];
    function.build_completion_items(document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "def ...");
}

#[rstest]
fn triggered_completion_items(foo_bar: (Workspace, Document)) {
    let mut workspace = foo_bar.0;
    let mut document = foo_bar.1;

    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 3,
                character: 5,
            },
            end: lsp_types::Position {
                line: 3,
                character: 5,
            },
        }),
        range_length: Some(0),
        text: ".".into(),
    };

    document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(&document);

    let node = workspace.ast.as_ref().unwrap().descendant_at(75).unwrap();

    let mut completion_items = vec![];
    node.build_triggered_completion_items(".", &document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "triggered! ...");
}
