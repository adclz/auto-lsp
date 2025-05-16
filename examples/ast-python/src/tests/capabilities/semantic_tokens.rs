/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use std::ops::Deref;
use rstest::{fixture, rstest};
use auto_lsp::core::salsa::db::BaseDatabase;
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder;
use auto_lsp::lsp_types::Url;
use crate::capabilities::code_lenses::dispatch_code_lenses;
use crate::capabilities::semantic_tokens::{dispatch_semantic_tokens, DECLARATION, FUNCTION, SUPPORTED_MODIFIERS, SUPPORTED_TYPES};
use crate::db::create_python_db;
use crate::generated::Module;

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

    let mut builder = SemanticTokensBuilder::new("".into());

    get_ast(&foo_bar, file)
        .iter()
        .for_each(|n| {
            dispatch_semantic_tokens(&foo_bar, file, n.deref(), &mut builder)
                .expect("Failed to dispatch semantic tokens");
        });

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
