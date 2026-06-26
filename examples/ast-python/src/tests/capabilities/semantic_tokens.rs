use crate::capabilities::semantic_tokens::{
    semantic_tokens_full, DECLARATION, FUNCTION, SUPPORTED_MODIFIERS, SUPPORTED_TYPES,
};
use crate::db::create_python_db;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{
    SemanticTokensParams, SemanticTokensResult, TextDocumentIdentifier, Url,
};
use rstest::{fixture, rstest};

#[fixture]
fn foo_bar() -> impl BaseDatabase {
    create_python_db(&[r#"# foo comment
def foo(param1, param2: int, param3: int = 5):
    pass

def bar():
    pass
"#])
}

#[rstest]
fn foo_bar_semantic_tokens(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let tokens = semantic_tokens_full(
        &foo_bar,
        SemanticTokensParams {
            text_document: TextDocumentIdentifier {
                uri: file.url(&foo_bar).clone(),
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        },
    )
    .unwrap()
    .unwrap();

    let tokens = if let SemanticTokensResult::Tokens(tokens) = tokens {
        tokens.data
    } else {
        panic!("Expected tokens");
    };

    // Tokens should be a Vec (boo and far)
    assert_eq!(tokens.len(), 2);

    assert_eq!(
        tokens[0].token_type,
        SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
    );

    assert_eq!(
        tokens[0].token_modifiers_bitset,
        SUPPORTED_MODIFIERS
            .iter()
            .position(|x| *x == DECLARATION)
            .unwrap() as u32,
    );

    // foo is at line 1
    assert_eq!(tokens[0].delta_line, 1);
    // char 4
    assert_eq!(tokens[0].delta_start, 4);
    assert_eq!(tokens[0].length, 3); // def

    assert_eq!(
        tokens[1].token_type,
        SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
    );

    assert_eq!(
        tokens[1].token_modifiers_bitset,
        SUPPORTED_MODIFIERS
            .iter()
            .position(|x| *x == DECLARATION)
            .unwrap() as u32,
    );
    // bar is at line 3
    assert_eq!(tokens[1].delta_line, 3);
    // char 4
    assert_eq!(tokens[1].delta_start, 4);
    assert_eq!(tokens[1].length, 3); // def
}
