use crate::core::document::Document;
use crate::core::root::Root;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::*;

#[fixture]
fn foo_bar() -> (Root, Document) {
    Root::from_utf8(
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

#[fixture]
fn foo_bar_with_type_error() -> (Root, Document) {
    Root::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
        def foo(param1, param2: int = "string"):
            pass
        
        def bar():
            pass  
        "#
        .into(),
    )
    .unwrap()
}

#[rstest]
fn foo_has_type_error(foo_bar: (Root, Document), foo_bar_with_type_error: (Root, Document)) {
    let foo_bar = foo_bar.0;
    // foo_bar has no type errors
    assert!(foo_bar.diagnostics.is_empty());
    assert!(foo_bar.unsolved_checks.is_empty());
    assert!(foo_bar.unsolved_references.is_empty());

    let foo_bar_with_type_error = foo_bar_with_type_error.0;
    // foo_bar_with_type_error has one type error
    assert!(!foo_bar_with_type_error.diagnostics.is_empty());
    assert!(!foo_bar_with_type_error.unsolved_checks.is_empty());
    assert!(foo_bar.unsolved_references.is_empty());

    assert_eq!(
        foo_bar_with_type_error.diagnostics[0].message,
        "Invalid value \"string\" for type int"
    );
}

#[fixture]
fn foo_with_type_error() -> (Root, Document) {
    Root::from_utf8(
        PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(p: int = "x"): pass "#.into(),
    )
    .unwrap()
}

#[rstest]
fn non_redundant_edited_type_error(mut foo_with_type_error: (Root, Document)) {
    // test to check if a same error is not reported twice between edits of the same error

    // foo_with_type_error has one type error
    let mut root = foo_with_type_error.0;
    let document = &mut foo_with_type_error.1;
    assert!(!root.diagnostics.is_empty());
    assert!(!root.unsolved_checks.is_empty());
    assert!(root.unsolved_references.is_empty());
    assert_eq!(
        root.diagnostics[0].message,
        "Invalid value \"x\" for type int"
    );

    // Insert "xxxx"
    // "def foo(p: int = "x"): pass " -> "def foo(p: int = "xxxx"): pass "
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 18,
            },
            end: lsp_types::Position {
                line: 0,
                character: 19,
            },
        }),
        range_length: Some(1),
        text: "xxxx".into(),
    };

    document
        .update(&mut root.parsers.tree_sitter.parser.write(), &vec![change])
        .unwrap();
    root.parse(document);

    // foo_with_type_error should have 1 error
    assert_eq!(root.diagnostics.len(), 1);
    assert_eq!(root.unsolved_checks.len(), 1);
    assert_eq!(root.unsolved_references.len(), 0);
    assert_eq!(
        root.diagnostics[0].message,
        "Invalid value \"xxxx\" for type int"
    );
}

#[rstest]
fn fix_type_error(mut foo_with_type_error: (Root, Document)) {
    // Replaces "x" with 1 and therefore fixes the type error

    // foo_with_type_error has one type error
    let mut root = foo_with_type_error.0;
    let document = &mut foo_with_type_error.1;
    assert!(!root.diagnostics.is_empty());
    assert!(!root.unsolved_checks.is_empty());
    assert!(root.unsolved_references.is_empty());
    assert_eq!(
        root.diagnostics[0].message,
        "Invalid value \"x\" for type int"
    );

    // Replace "x" with 1
    // "def foo(p: int = "x"): pass " -> "def foo(p: int = 1): pass "
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 17,
            },
            end: lsp_types::Position {
                line: 0,
                character: 20,
            },
        }),
        range_length: Some(3),
        text: "1".into(),
    };
    document
        .update(&mut root.parsers.tree_sitter.parser.write(), &vec![change])
        .unwrap();

    root.parse(document);

    // foo_with_type_error should have no type errors
    assert_eq!(root.diagnostics.len(), 0);
    assert_eq!(root.unsolved_checks.len(), 0);
    assert_eq!(root.unsolved_references.len(), 0);
}
