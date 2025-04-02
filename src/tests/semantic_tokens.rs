use crate::python::ast::Module;
use auto_lsp_core::{
    ast::BuildSemanticTokens,
    salsa::{db::BaseDatabase, tracked::get_ast},
};
use lsp_types::Url;
use rstest::{fixture, rstest};

use super::python_utils::create_python_db;
use crate::python::semantic_tokens::{DECLARATION, FUNCTION, SUPPORTED_MODIFIERS, SUPPORTED_TYPES};

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
    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).to_symbol();

    let ast = root.unwrap();

    let mut builder = auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder::new("".into());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    module.build_semantic_tokens(&document, &mut builder);

    let tokens = builder.build().data;

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
