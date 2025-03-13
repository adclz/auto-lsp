use crate::core::ast::Comment;
use crate::tests::python_utils::get_mut_python_file;
use auto_lsp_core::ast::GetSymbolData;
use auto_lsp_core::workspace::Workspace;
use rstest::{fixture, rstest};

use super::python_utils::{create_python_workspace, get_python_file};
use super::python_workspace::ast::Module;

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
fn foo_bar_comment(foo_bar: Workspace) {
    let (root, document) = get_python_file(&foo_bar);
    let ast = root.ast.as_ref().unwrap();

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
fn foo_bar_no_comments() -> Workspace {
    create_python_workspace(
        r#"def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#,
    )
}

#[rstest]
fn add_comments(mut foo_bar_no_comments: Workspace) {
    let (root, document) = get_mut_python_file(&mut foo_bar_no_comments);
    let ast = root.ast.as_ref().unwrap();

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

    document
        .update(
            &mut root.parsers.tree_sitter.parser.write(),
            &vec![foo_change, bar_change],
        )
        .unwrap();

    drop(ast);
    root.parse(document);
    root.set_comments(document);

    let ast = root.ast.as_ref().unwrap();
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
fn foo_bar_with_comments() -> Workspace {
    create_python_workspace(
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

# bar comment
def bar():
    pass  
"#,
    )
}

#[rstest]
fn remove_comments(mut foo_bar_with_comments: Workspace) {
    let (root, document) = get_mut_python_file(&mut foo_bar_with_comments);
    let ast = root.ast.as_ref().unwrap();
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

    document
        .update(
            &mut root.parsers.tree_sitter.parser.write(),
            &vec![foo_change, bar_change],
        )
        .unwrap();

    drop(ast);
    root.parse(document);
    root.set_comments(document);

    let ast = root.ast.as_ref().unwrap();
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
