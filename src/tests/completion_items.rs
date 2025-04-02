use crate::core::ast::BuildCompletionItems;
use auto_lsp_core::ast::{BuildTriggeredCompletionItems, Traverse};
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
    i

def bar():
    pass  
"#])
}

#[rstest]
fn global_completion_items(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).to_symbol();

    // Module returns globally available completion items
    let module = root.as_ref().unwrap().read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut completion_items = vec![];
    module.build_completion_items(&document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "def ...");

    // Function should do the same
    let function = module.statements[0].read();

    let mut completion_items = vec![];
    function.build_completion_items(&document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "def ...");
}

#[rstest]
fn triggered_completion_items(mut foo_bar: impl BaseDatabase) {
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

    foo_bar
        .update(
            &Url::parse("file:///test0.py").expect("Invalid URL"),
            &vec![change],
        )
        .expect("Failed to update file");

    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).to_symbol();

    let node = root.as_ref().unwrap().descendant_at(75).unwrap();

    let mut completion_items = vec![];
    node.build_triggered_completion_items(".", &document, &mut completion_items);

    assert_eq!(completion_items.len(), 1);
    assert_eq!(completion_items[0].label, "triggered! ...");
}
