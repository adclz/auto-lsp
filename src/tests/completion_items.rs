use crate::core::ast::BuildCompletionItems;
use crate::core::document::Document;
use crate::core::root::Root;
use auto_lsp_core::ast::{BuildTriggeredCompletionItems, Traverse};
use rstest::{fixture, rstest};

use super::python_utils::create_python_workspace;
use super::python_workspace::ast::Module;

#[fixture]
fn foo_bar() -> (Root, Document) {
    create_python_workspace(
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass
    i

def bar():
    pass  
"#,
    )
}

#[rstest]
fn global_completion_items(foo_bar: (Root, Document)) {
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
fn triggered_completion_items(foo_bar: (Root, Document)) {
    let mut root = foo_bar.0;
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
        .update(&mut root.parsers.tree_sitter.parser.write(), &vec![change])
        .unwrap();

    root.parse(&document);

    let node = root.ast.as_ref().unwrap().descendant_at(75).unwrap();

    let mut completion_items = vec![];
    node.build_triggered_completion_items(".", &document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "triggered! ...");
}
