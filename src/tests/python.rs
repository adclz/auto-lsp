use crate::core::ast::{AstSymbol, BuildInlayHints, GetSymbolData, IsComment, VecOrSymbol};
use crate::core::workspace::Workspace;
use auto_lsp_core::ast::{BuildCodeLens, GetHoverInfo};
use lsp_types::Url;
use std::sync::LazyLock;

use crate::python_workspace::*;

static TEST_FILE: LazyLock<Workspace> = LazyLock::new(|| {
    create_python_workspace(
        Url::parse("file:///test.py").unwrap(),
        r#"# foo comment
def foo():
    pass

def bar():
    pass  
"#
        .into(),
    )
});

#[test]
fn check_ast() {
    let workspace = &TEST_FILE;
    let ast = workspace.ast.as_ref().unwrap();
    let document = &workspace.document;

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
            .get_text(document.document.text.as_bytes())
            .unwrap(),
        "foo"
    );

    let function = module.functions[1].read();
    assert_eq!(
        function
            .name
            .read()
            .get_text(document.document.text.as_bytes())
            .unwrap(),
        "bar"
    );

    // Checks if bar's parent is module
    assert!(function.name.read().get_parent().is_some());
    let parent = function.name.read().get_parent().unwrap();
    assert!(parent.to_dyn().unwrap().read().is::<Function>());
}

#[test]
fn check_comment() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();
    let document = &test_file.document;

    // Root node should be module

    assert!(ast.read().is::<Module>());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let function = module.functions[0].read();
    assert!(function.is_comment());
    assert_eq!(
        function.get_comment(document.document.text.as_bytes()),
        Some("# foo comment")
    );
}

#[test]
fn check_document_symbols() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let symbols = ast
        .read()
        .get_document_symbols(&test_file.document)
        .unwrap();

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

#[test]
fn check_semantic_tokens() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let mut builder = auto_lsp_core::semantic_tokens::SemanticTokensBuilder::new("".into());
    ast.read()
        .build_semantic_tokens(&test_file.document, &mut builder);

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

#[test]
fn check_hover() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let foo = module.functions[0].read();
    let foo_name = foo.name.read();

    let foo_hover = foo_name.get_hover(&test_file.document).unwrap();

    assert_eq!(
        foo_hover.contents,
        lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
            kind: lsp_types::MarkupKind::PlainText,
            value: "# foo comment\nhover foo".into(),
        })
    );

    let bar = module.functions[1].read();
    let bar_name = bar.name.read();

    let bar_hover = bar_name.get_hover(&test_file.document).unwrap();

    assert_eq!(
        bar_hover.contents,
        lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
            kind: lsp_types::MarkupKind::PlainText,
            value: "hover bar".into(),
        })
    );
}

#[test]
fn check_inlay_hints() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut hints = vec![];
    module.build_inlay_hint(&test_file.document, &mut hints);

    assert_eq!(hints.len(), 2);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}

#[test]
fn check_code_lens() {
    let test_file = &TEST_FILE;
    let ast = test_file.ast.as_ref().unwrap();

    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    let mut code_lens = vec![];
    module.build_code_lens(&test_file.document, &mut code_lens);

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
