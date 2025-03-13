use crate::{core::workspace::Workspace, tests::python_utils::get_python_file};
use auto_lsp_core::ast::BuildSemanticTokens;
use rstest::{fixture, rstest};

use super::{python_utils::create_python_workspace, python_workspace::ast::Module};
use crate::python::semantic_tokens::{DECLARATION, FUNCTION, SUPPORTED_MODIFIERS, SUPPORTED_TYPES};

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
fn foo_bar_semantic_tokens(foo_bar: Workspace) {
    let (root, document) = get_python_file(&foo_bar);
    let ast = root.ast.as_ref().unwrap();

    let mut builder = auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder::new("".into());
    let module = ast.read();
    let module = module.downcast_ref::<Module>().unwrap();

    module.build_semantic_tokens(document, &mut builder);

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
