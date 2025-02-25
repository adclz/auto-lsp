use crate::core::ast::Comment;
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::GetSymbolData;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::ast::Module;
use super::python_workspace::PYTHON_PARSERS;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
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
fn foo_bar_comment(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let function = module.statements[0].read();
    assert!(function.is_comment());
    assert_eq!(
        function.get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );
}

#[fixture]
fn foo_bar_no_comments() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn add_comments(mut foo_bar_no_comments: (Workspace, Document)) {
    let mut workspace = foo_bar_no_comments.0;
    let document = &mut foo_bar_no_comments.1;
    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // foo has no comment
    let foo = &ast.downcast_ref::<Module>().unwrap().statements[0];
    assert!(foo
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());

    // bar has no comment
    let bar = &ast.downcast_ref::<Module>().unwrap().statements[1];
    assert!(bar
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());

    // Insert comments
    let foo_change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: 0,
                character: 0,
            },
        }),
        range_length: Some(0),
        text: "# foo comment\n".into(),
    };

    let bar_change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 3,
                character: 0,
            },
            end: lsp_types::Position {
                line: 3,
                character: 0,
            },
        }),
        range_length: Some(0),
        text: "\n# bar comment".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![foo_change, bar_change],
        )
        .unwrap();

    drop(ast);
    workspace.parse(Some(&edits), document);

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // foo has comment
    let foo = &ast.downcast_ref::<Module>().unwrap().statements[0];
    assert_eq!(
        foo.read().get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );

    let bar = &ast.downcast_ref::<Module>().unwrap().statements[1];
    assert_eq!(
        bar.read().get_comment(document.texter.text.as_bytes()),
        Some("# bar comment")
    );
}

#[fixture]
fn foo_bar_with_comments() -> (Workspace, Document) {
    Workspace::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

# bar comment
def bar():
    pass  
"#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn remove_comments(mut foo_bar_with_comments: (Workspace, Document)) {
    let mut workspace = foo_bar_with_comments.0;
    let document = &mut foo_bar_with_comments.1;
    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // foo has comment
    let foo = &ast.downcast_ref::<Module>().unwrap().statements[0];
    assert_eq!(
        foo.read().get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );

    // bar has comment
    let bar = &ast.downcast_ref::<Module>().unwrap().statements[1];
    assert_eq!(
        bar.read().get_comment(document.texter.text.as_bytes()),
        Some("# bar comment")
    );

    // Remove comments
    let foo_change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: 0,
                character: 13,
            },
        }),
        range_length: Some(13),
        text: "".into(),
    };

    let bar_change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 4,
                character: 0,
            },
            end: lsp_types::Position {
                line: 4,
                character: 13,
            },
        }),
        range_length: Some(13),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![foo_change, bar_change],
        )
        .unwrap();

    drop(ast);
    workspace.parse(Some(&edits), document);

    let ast = workspace.ast.as_ref().unwrap();
    let ast = ast.read();

    // foo has no comment
    let foo = &ast.downcast_ref::<Module>().unwrap().statements[0];
    assert!(foo
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());

    let bar = &ast.downcast_ref::<Module>().unwrap().statements[1];
    assert!(bar
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());
}
