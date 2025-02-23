use std::ops::Deref;

use crate::core::document::Document;
use crate::core::workspace::Workspace;
use crate::python::ast::{CompoundStatement, ParameterBuilder, Statement, StatementBuilder};
use auto_lsp_core::ast::ChangeReport;
use auto_lsp_core::ast::GetSymbolData;
use auto_lsp_core::build::Queryable;
use auto_lsp_core::document::ChangeKind;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::html_workspace::*;
use super::python_workspace::ast::Module;
use super::python_workspace::PYTHON_PARSERS;

#[fixture]
fn empty() -> (Workspace, Document) {
    Workspace::from_utf8(
        &HTML_PARSERS.get("html").unwrap(),
        Url::parse("file:///sample_file.html").unwrap(),
        r#""#.into(),
    )
    .unwrap()
}

#[rstest]
fn empty_document(empty: (Workspace, Document)) {
    let mut workspace = empty.0;
    let mut document = empty.1;

    // Should not have an AST
    assert!(workspace.ast.is_none());
    assert!(workspace.diagnostics.is_empty());
    assert!(workspace.unsolved_checks.is_empty());
    assert!(workspace.unsolved_references.is_empty());

    let change = lsp_types::TextDocumentContentChangeEvent {
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
        range_length: Some(26),
        text: "<div></div>".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), &document);

    // Should have an AST
    assert!(workspace.ast.is_some());

    let html = workspace.ast.unwrap();
    let html = html.read();
    let html = html.downcast_ref::<HtmlDocument>().unwrap();
    let tags = &html.tags;

    // Should contain div

    assert_eq!(tags.len(), 1);
    assert!(matches!(*tags[0].read(), Node::Element(_)));
}

#[fixture]
fn foo() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(param1, param2: int, param3: int = 5):
    pass"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn replace_first_parameter_name(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Change "param1" to "p" (delete "aram1")
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 9,
            },
            end: lsp_types::Position {
                line: 0,
                character: 14,
            },
        }),
        range_length: Some(5),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // param1 is at index 0
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Replace(0, ParameterBuilder::QUERY_NAMES)
    ));
}

#[rstest]
fn remove_last_parameter(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Replace "param3: int = 5" to ""
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 29,
            },
            end: lsp_types::Position {
                line: 0,
                character: 45,
            },
        }),
        range_length: Some(16),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // param3 is at index 2
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(2, ParameterBuilder::QUERY_NAMES)
    ));
}

#[rstest]
fn replace_two_last_parameters(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    // Replace 2 last parameters with the same text
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 16,
            },
            end: lsp_types::Position {
                line: 0,
                character: 44,
            },
        }),
        range_length: Some(28),
        text: "param2: int, param3: int = 5".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(1, ParameterBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Remove(2, ParameterBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[2],
        ChangeReport::Replace(0, ParameterBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[3],
        ChangeReport::Insert(1, ParameterBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[4],
        ChangeReport::Insert(2, ParameterBuilder::QUERY_NAMES)
    ));
}

#[rstest]
fn insert_bar(mut foo: (Workspace, Document)) {
    let mut workspace = foo.0;
    let document = &mut foo.1;

    {
        let ast = workspace.ast.as_mut().unwrap();
        let ast = ast.read();
        let module = ast.downcast_ref::<Module>().unwrap();
        let function = &module.statements[0];
        let function = function.read();
        if let Statement::Compound(CompoundStatement::Function(foo)) = function.deref() {
            let pass = foo.body[0].read();
            assert_eq!(pass.get_range(), 51..55);
        } else {
            panic!("Expected function statement");
        }
    }

    // add bar under foo
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 1,
                character: 8,
            },
            end: lsp_types::Position {
                line: 1,
                character: 8,
            },
        }),
        range_length: Some(0),
        text: "\ndef bar():\n    pass".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits[0].kind, ChangeKind::Insert);
    assert_eq!(edits[0].input_edit.start_byte, 55);
    assert_eq!(edits[0].trim_start, 1);

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Replace(0, StatementBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Insert(1, StatementBuilder::QUERY_NAMES)
    ));
}

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(param1, param2: int, param3: int = 5):
    pass
def bar():
    pass"#
            .into(),
    )
    .unwrap()
}

#[rstest]
fn remove_bar(mut foo_bar: (Workspace, Document)) {
    let mut workspace = foo_bar.0;
    let document = &mut foo_bar.1;

    // Remove bar
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 2,
                character: 0,
            },
            end: lsp_types::Position {
                line: 3,
                character: 8,
            },
        }),
        range_length: Some(18),
        text: "".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits[0].kind, ChangeKind::Delete);
    assert_eq!(edits[0].input_edit.start_byte, 56);
    assert_eq!(edits[0].trim_start, 0);

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Remove(1, StatementBuilder::QUERY_NAMES)
    ));
}

#[rstest]
fn insert_baz_between(mut foo_bar: (Workspace, Document)) {
    let mut workspace = foo_bar.0;
    let document = &mut foo_bar.1;

    // insert baz under foo (between foo and bar)
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 1,
                character: 8,
            },
            end: lsp_types::Position {
                line: 1,
                character: 8,
            },
        }),
        range_length: Some(0),
        text: "\ndef baz():\n    pass".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits[0].kind, ChangeKind::Insert);
    assert_eq!(edits[0].input_edit.start_byte, 55);
    assert_eq!(edits[0].trim_start, 1);

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Replace(0, StatementBuilder::QUERY_NAMES)
    ));

    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Insert(1, StatementBuilder::QUERY_NAMES)
    ));
}

#[fixture]
fn string_filter() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PYTHON_PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def stringfilter(func):

    """
    Decorator for filters which should only receive strings. The object
    passed as the first positional argument will be converted to a string.
    """



    def _dec(first, *args, **kwargs):
        first = str(first)
        result = func(first, *args, **kwargs)
        if isinstance(first, SafeData) and getattr(unwrap(func), "is_safe", False):
            result = mark_safe(result)
        return result

    return _dec"#
            .into(),
    )
        .unwrap()
}

#[rstest]
fn insert_between(mut string_filter: (Workspace, Document)) {
    let mut workspace = string_filter.0;
    let document = &mut string_filter.1;

    {
        let ast = workspace.ast.as_mut().unwrap();
        let ast = ast.read();
        let module = ast.downcast_ref::<Module>().unwrap();
        // string filter fn
        let function = &module.statements[0];
        let function = function.read();
        if let Statement::Compound(CompoundStatement::Function(string_builder)) = function.deref() {
            assert_eq!(string_builder.body.len(), 3);
        } else {
            panic!("Expected function statement");
        }
    }

    // Insert "    a" right after string
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 8,
                character: 0,
            },
            end: lsp_types::Position {
                line: 8,
                character: 0,
            },
        }),
        range_length: Some(0),
        text: "    a".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits[0].kind, ChangeKind::Insert);

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // Replaces the string
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Replace(0, StatementBuilder::QUERY_NAMES)
    ));

    // Inserts a
    assert!(matches!(
        workspace.changes[1],
        ChangeReport::Insert(1, StatementBuilder::QUERY_NAMES)
    ));

    // Insert another a after a
    let change = lsp_types::TextDocumentContentChangeEvent {
        range: Some(lsp_types::Range {
            start: lsp_types::Position {
                line: 8,
                character: 5,
            },
            end: lsp_types::Position {
                line: 8,
                character: 5,
            },
        }),
        range_length: Some(0),
        text: "a".into(),
    };

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    assert_eq!(edits[0].kind, ChangeKind::Insert);

    workspace.parse(Some(&edits), document);

    assert!(!workspace.changes.is_empty());

    // Replaces a with aa
    assert!(matches!(
        workspace.changes[0],
        ChangeReport::Replace(1, StatementBuilder::QUERY_NAMES)
    ));
}