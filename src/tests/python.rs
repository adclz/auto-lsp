use crate::core::ast::{AstSymbol, BuildInlayHints, GetSymbolData, IsComment, VecOrSymbol};
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::{BuildCodeLens, GetHover};
use auto_lsp_core::document::Document;
use lsp_types::Url;
use rstest::{fixture, rstest};

use crate::python_workspace::*;

#[fixture]
fn foo_bar() -> (Workspace, Document) {
    create_python_workspace(
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass  
"#
        .into(),
    )
}

#[fixture]
fn foo_bar_with_type_error() -> (Workspace, Document) {
    create_python_workspace(
        Url::parse("file:///test_type_error.py").unwrap(),
        r#"# foo comment
def foo(param1, param2: int = "string"):
    pass

def bar():
    pass  
"#
        .into(),
    )
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

    // Foo has 2 parameters
    let function = module.functions[0].read();
    assert_eq!(function.parameters.read().parameters.len(), 3);
    let parameters = &function.parameters.read().parameters;

    // param1 is untyped
    assert!(matches!(*parameters[0].read(), Parameter::Untyped(_)));

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

        assert!(matches!(*typed.parameter_type.read(), Type::Int(_)));
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

        assert!(matches!(*typed_default.parameter_type.read(), Type::Int(_)));

        assert_eq!(
            typed_default
                .default
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
fn check_type_checking(
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

    let mut builder = auto_lsp_core::semantic_tokens::SemanticTokensBuilder::new("".into());
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
    module.build_inlay_hint(&document, &mut hints);

    assert_eq!(hints.len(), 2);

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
