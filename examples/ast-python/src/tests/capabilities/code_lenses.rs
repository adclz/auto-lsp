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
use crate::{db::create_python_db, generated::Module};
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::core::{salsa::db::BaseDatabase};
use auto_lsp::lsp_types::Url;
use rstest::{fixture, rstest};
use crate::capabilities::code_lenses::dispatch_code_lenses;

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
fn foo_bar_code_lens(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let mut code_lenses = vec![];
    get_ast(&foo_bar, file)
        .iter()
        .for_each(|n| {
            dispatch_code_lenses(&foo_bar, file, n.deref(), &mut code_lenses)
                .expect("Failed to dispatch code lenses");
        });

    assert_eq!(code_lenses.len(), 2);

    assert_eq!(code_lenses[0].range.start.line, 1);
    assert_eq!(code_lenses[0].range.start.character, 4);
    assert_eq!(code_lenses[0].range.end.line, 1);
    assert_eq!(code_lenses[0].range.end.character, 7);

    assert_eq!(code_lenses[1].range.start.line, 4);
    assert_eq!(code_lenses[1].range.start.character, 4);
    assert_eq!(code_lenses[1].range.end.line, 4);
    assert_eq!(code_lenses[1].range.end.character, 7);
}
