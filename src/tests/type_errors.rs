use crate::{
    core::workspace::Workspace,
    tests::python_utils::{get_mut_python_file, get_python_file},
};
use rstest::{fixture, rstest};

use super::python_utils::create_python_workspace;

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

#[fixture]
fn foo_bar_with_type_error() -> Workspace {
    create_python_workspace(
        r#"# foo comment
        def foo(param1, param2: int = "string"):
            pass
        
        def bar():
            pass  
        "#,
    )
}

#[rstest]
fn foo_has_type_error(foo_bar: Workspace, foo_bar_with_type_error: Workspace) {
    let (foo_bar, _) = get_python_file(&foo_bar);
    let (foo_bar_with_type_error, _) = get_python_file(&foo_bar_with_type_error);

    // foo_bar has no type errors
    assert!(foo_bar.ast_diagnostics.is_empty());
    assert!(foo_bar.unsolved_checks.is_empty());
    assert!(foo_bar.unsolved_references.is_empty());

    // foo_bar_with_type_error has one type error
    assert!(!foo_bar_with_type_error.ast_diagnostics.is_empty());
    assert!(!foo_bar_with_type_error.unsolved_checks.is_empty());
    assert!(foo_bar.unsolved_references.is_empty());

    assert_eq!(
        foo_bar_with_type_error.ast_diagnostics[0].message,
        "Invalid value \"string\" for type int"
    );
}

#[fixture]
fn foo_with_type_error() -> Workspace {
    create_python_workspace(r#"def foo(p: int = "x"): pass "#)
}

#[rstest]
fn non_redundant_edited_type_error(mut foo_with_type_error: Workspace) {
    let (root, document) = get_mut_python_file(&mut foo_with_type_error);
    // test to check if a same error is not reported twice between edits of the same error

    // foo_with_type_error has one type error
    assert!(!root.ast_diagnostics.is_empty());
    assert!(!root.unsolved_checks.is_empty());
    assert!(root.unsolved_references.is_empty());
    assert_eq!(
        root.ast_diagnostics[0].message,
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
    root.resolve_checks(document);

    // foo_with_type_error should have 1 error
    assert_eq!(root.ast_diagnostics.len(), 1);
    assert_eq!(root.unsolved_checks.len(), 1);
    assert_eq!(root.unsolved_references.len(), 0);
    assert_eq!(
        root.ast_diagnostics[0].message,
        "Invalid value \"xxxx\" for type int"
    );
}

#[rstest]
fn fix_type_error(mut foo_with_type_error: Workspace) {
    let (root, document) = get_mut_python_file(&mut foo_with_type_error);
    // Replaces "x" with 1 and therefore fixes the type error

    // foo_with_type_error has one type error
    assert!(!root.ast_diagnostics.is_empty());
    assert!(!root.unsolved_checks.is_empty());
    assert!(root.unsolved_references.is_empty());
    assert_eq!(
        root.ast_diagnostics[0].message,
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
    root.resolve_checks(document);

    // foo_with_type_error should have no type errors
    assert_eq!(root.ast_diagnostics.len(), 0);
    assert_eq!(root.unsolved_checks.len(), 0);
    assert_eq!(root.unsolved_references.len(), 0);
}
