use crate::core::ast::{AstSymbol, BuildInlayHints, GetSymbolData, IsComment, VecOrSymbol};
use crate::core::ast::{BuildCodeLens, GetHover};
use crate::core::document::Document;
use crate::core::workspace::Workspace;
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_workspace::*;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("python").unwrap(),
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
fn check_ast(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1.texter;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    // Both bar and foo should be found
    assert_eq!(module.functions.len(), 2);
    let function = module.functions[0].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.text.as_bytes())
            .unwrap(),
        "foo"
    );

    let function = module.functions[1].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.text.as_bytes())
            .unwrap(),
        "bar"
    );

    // Checks if bar's parent is module
    assert!(function.name.read().get_parent().is_some());
    let parent = function.name.read().get_parent().unwrap();
    assert!(parent.to_dyn().unwrap().read().is::<Function>());
}

#[rstest]
fn check_foo_parameters(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1.texter;
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    // Foo has 3 parameters
    let function = module.functions[0].read();
    assert_eq!(function.parameters.read().parameters.len(), 3);
    let parameters = &function.parameters.read().parameters;

    // param1 is untyped
    assert!(matches!(*parameters[0].read(), Parameter::Identifier(_)));

    // param2 is typed
    assert!(matches!(*parameters[1].read(), Parameter::Typed(_)));
    if let Parameter::Typed(typed) = &*parameters[1].read() {
        assert_eq!(
            typed
                .name
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "param2"
        );

        assert_eq!(
            typed
                .parameter_type
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "int"
        );
    } else {
        panic!("Expected Typed parameter");
    }

    // param3 is typed with default value
    if let Parameter::TypedDefault(typed_default) = &*parameters[2].read() {
        assert_eq!(
            typed_default
                .name
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "param3"
        );

        assert_eq!(
            typed_default
                .parameter_type
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "int"
        );

        assert!(typed_default.value.read().is_integer());

        assert_eq!(
            typed_default
                .value
                .read()
                .get_text(document.text.as_bytes())
                .unwrap(),
            "5"
        );
    } else {
        panic!("Expected TypedDefault parameter");
    }

    // param3 is typed with default value
    assert!(matches!(*parameters[2].read(), Parameter::TypedDefault(_)));
}

#[rstest]
fn check_comment(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let function = module.functions[0].read();
    assert!(function.is_comment());
    assert_eq!(
        function.get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );
}

#[rstest]
fn check_document_symbols(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let symbols = ast.read().get_document_symbols(&document).unwrap();

    // Symbols should be a Vec (boo and far)
    assert!(matches!(symbols, VecOrSymbol::Vec(_)));

    if let VecOrSymbol::Vec(symbols) = symbols {
        assert_eq!(symbols.len(), 2);

        assert_eq!(symbols[0].kind, lsp_types::SymbolKind::FUNCTION);
        assert_eq!(symbols[0].name, "foo");

        assert_eq!(symbols[1].kind, lsp_types::SymbolKind::FUNCTION);
        assert_eq!(symbols[1].name, "bar");
    } else {
        panic!("Expected VecOrSymbol::Vec");
    }
}

#[rstest]
fn check_semantic_tokens(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let mut builder = auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder::new("".into());
    ast.read().build_semantic_tokens(&document, &mut builder);

    let tokens = builder.build().data;

    // Tokens should be a Vec (boo and far)
    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens[0].token_type,
        TOKEN_TYPES.get_index("Function").unwrap() as u32
    );
    // foo is at line 1
    assert_eq!(tokens[0].delta_line, 1);
    // char 4
    assert_eq!(tokens[0].delta_start, 4);
    assert_eq!(tokens[0].length, 3); // def

    assert_eq!(
        tokens[1].token_type,
        TOKEN_TYPES.get_index("Function").unwrap() as u32
    );
    // bar is at line 3
    assert_eq!(tokens[1].delta_line, 3);
    // char 4
    assert_eq!(tokens[1].delta_start, 4);
    assert_eq!(tokens[1].length, 3); // def
}

#[rstest]
fn check_hover(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let foo = module.functions[0].read();
    let foo_name = foo.name.read();

    let foo_hover = foo_name.get_hover(&document).unwrap();

    assert_eq!(
        foo_hover.contents,
        lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
            kind: lsp_types::MarkupKind::PlainText,
            value: "# foo comment\nhover foo".into(),
        })
    );

    let bar = module.functions[1].read();
    let bar_name = bar.name.read();

    let bar_hover = bar_name.get_hover(&document).unwrap();

    assert_eq!(
        bar_hover.contents,
        lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
            kind: lsp_types::MarkupKind::PlainText,
            value: "hover bar".into(),
        })
    );
}

#[rstest]
fn check_inlay_hints(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut hints = vec![];
    module.build_inlay_hints(&document, &mut hints);

    assert_eq!(hints.len(), 4);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}

#[rstest]
fn check_code_lens(foo_bar: (Workspace, Document)) {
    let ast = foo_bar.0.ast.as_ref().unwrap();
    let document = &foo_bar.1;

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut code_lens = vec![];
    module.build_code_lens(&document, &mut code_lens);

    assert_eq!(code_lens.len(), 2);

    assert_eq!(code_lens[0].range.start.line, 1);
    assert_eq!(code_lens[0].range.start.character, 4);
    assert_eq!(code_lens[0].range.end.line, 1);
    assert_eq!(code_lens[0].range.end.character, 7);

    assert_eq!(code_lens[1].range.start.line, 4);
    assert_eq!(code_lens[1].range.start.character, 4);
    assert_eq!(code_lens[1].range.end.line, 4);
    assert_eq!(code_lens[1].range.end.character, 7);
}

#[fixture]
fn foo_bar_no_comments() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("python").unwrap(),
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
    let foo = &ast.downcast_ref::<Module>().unwrap().functions[0];
    assert!(foo
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());

    // bar has no comment
    let bar = &ast.downcast_ref::<Module>().unwrap().functions[1];
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
    let foo = &ast.downcast_ref::<Module>().unwrap().functions[0];
    assert_eq!(
        foo.read().get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );

    let bar = &ast.downcast_ref::<Module>().unwrap().functions[1];
    assert_eq!(
        bar.read().get_comment(document.texter.text.as_bytes()),
        Some("# bar comment")
    );
}

#[fixture]
fn foo_bar_with_comments() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("python").unwrap(),
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
    let foo = &ast.downcast_ref::<Module>().unwrap().functions[0];
    assert_eq!(
        foo.read().get_comment(document.texter.text.as_bytes()),
        Some("# foo comment")
    );

    // bar has comment
    let bar = &ast.downcast_ref::<Module>().unwrap().functions[1];
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
    let foo = &ast.downcast_ref::<Module>().unwrap().functions[0];
    assert!(foo
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());

    let bar = &ast.downcast_ref::<Module>().unwrap().functions[1];
    assert!(bar
        .read()
        .get_comment(document.texter.text.as_bytes())
        .is_none());
}

#[fixture]
fn foo_bar_with_type_error() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("python").unwrap(),
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
fn foo_has_type_error(
    foo_bar: (Workspace, Document),
    foo_bar_with_type_error: (Workspace, Document),
) {
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
fn foo_with_type_error() -> (Workspace, Document) {
    Workspace::from_utf8(
        &PARSERS.get("python").unwrap(),
        Url::parse("file:///test.py").unwrap(),
        r#"def foo(p: int = "x"): pass "#.into(),
    )
    .unwrap()
}

#[rstest]
fn non_redundant_edited_type_error(mut foo_with_type_error: (Workspace, Document)) {
    // test to check if a same error is not reported twice between edits of the same error

    // foo_with_type_error has one type error
    let mut workspace = foo_with_type_error.0;
    let document = &mut foo_with_type_error.1;
    assert!(!workspace.diagnostics.is_empty());
    assert!(!workspace.unsolved_checks.is_empty());
    assert!(workspace.unsolved_references.is_empty());
    assert_eq!(
        workspace.diagnostics[0].message,
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

    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    // foo_with_type_error should have 1 error
    assert_eq!(workspace.diagnostics.len(), 1);
    assert_eq!(workspace.unsolved_checks.len(), 1);
    assert_eq!(workspace.unsolved_references.len(), 0);
    assert_eq!(
        workspace.diagnostics[0].message,
        "Invalid value \"xxxx\" for type int"
    );
}

#[rstest]
fn fix_type_error(mut foo_with_type_error: (Workspace, Document)) {
    // Replaces "x" with 1 and therefore fixes the type error

    // foo_with_type_error has one type error
    let mut workspace = foo_with_type_error.0;
    let document = &mut foo_with_type_error.1;
    assert!(!workspace.diagnostics.is_empty());
    assert!(!workspace.unsolved_checks.is_empty());
    assert!(workspace.unsolved_references.is_empty());
    assert_eq!(
        workspace.diagnostics[0].message,
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
    let edits = document
        .update(
            &mut workspace.parsers.tree_sitter.parser.write(),
            &vec![change],
        )
        .unwrap();

    workspace.parse(Some(&edits), document);

    // foo_with_type_error should have no type errors
    assert_eq!(workspace.diagnostics.len(), 0);
    assert_eq!(workspace.unsolved_checks.len(), 0);
    assert_eq!(workspace.unsolved_references.len(), 0);
}
