use crate::python::check::type_check_default_parameters;
use auto_lsp_core::salsa::{
    db::BaseDatabase,
    tracked::DiagnosticAccumulator,
};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass
"#])
}

#[fixture]
fn foo_bar_with_type_error() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
        def foo(param1, param2: int = "string"):
            pass

        def bar():
            pass
        "#])
}

#[rstest]
fn foo_has_type_error(foo_bar: impl BaseDatabase, foo_bar_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_bar.get_file(&file0_url).unwrap();

    let foo_bar_diagnostics =
        type_check_default_parameters::accumulated::<DiagnosticAccumulator>(&foo_bar, file);

    // foo_bar has no type errors
    assert!(foo_bar_diagnostics.is_empty());

    let file = foo_bar_with_type_error.get_file(&file0_url).unwrap();

    let foo_bar_diagnostics = type_check_default_parameters::accumulated::<DiagnosticAccumulator>(
        &foo_bar_with_type_error,
        file,
    );

    // foo_bar_with_type_error has one type error
    assert!(!foo_bar_diagnostics.is_empty());

    assert_eq!(
        foo_bar_diagnostics[0].0.message,
        "Invalid value \"string\" for type int"
    );
}

#[fixture]
fn foo_with_type_error() -> impl BaseDatabase {
    create_python_db(&[r#"def foo(p: int = "x"): pass "#])
}

#[rstest]
fn non_redundant_edited_type_error(mut foo_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_with_type_error.get_file(&file0_url).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        DiagnosticAccumulator,
    >(&foo_with_type_error, file);

    // test to check if a same error is not reported twice between edits of the same error

    // foo_with_type_error has one type error
    assert!(!foo_with_type_error_diagnostics.is_empty());
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
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

    foo_with_type_error
        .update(&file0_url, &[change])
        .unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        DiagnosticAccumulator,
    >(&foo_with_type_error, file);

    // foo_with_type_error should have 1 error
    assert_eq!(foo_with_type_error_diagnostics.len(), 1);
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
        "Invalid value \"xxxx\" for type int"
    );
}

#[rstest]
fn fix_type_error(mut foo_with_type_error: impl BaseDatabase) {
    let file0_url = Url::parse("file:///test0.py").unwrap();
    let file = foo_with_type_error.get_file(&file0_url).unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        DiagnosticAccumulator,
    >(&foo_with_type_error, file);
    // Replaces "x" with 1 and therefore fixes the type error

    // foo_with_type_error has one type error
    assert!(!foo_with_type_error_diagnostics.is_empty());
    assert_eq!(
        foo_with_type_error_diagnostics[0].0.message,
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

    foo_with_type_error
        .update(&file0_url, &[change])
        .unwrap();

    let foo_with_type_error_diagnostics = type_check_default_parameters::accumulated::<
        DiagnosticAccumulator,
    >(&foo_with_type_error, file);

    // foo_with_type_error should have no type errors
    assert_eq!(foo_with_type_error_diagnostics.len(), 0);
}
