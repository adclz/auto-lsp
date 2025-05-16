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
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::Url;
use crate::capabilities::code_lenses::dispatch_code_lenses;
use crate::capabilities::inlay_hints::dispatch_inlay_hints;
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
fn foo_bar_inlay_hints(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let mut hints = vec![];
    get_ast(&foo_bar, file)
        .iter()
        .for_each(|n| {
            dispatch_inlay_hints(&foo_bar, file, n.deref(), &mut hints)
                .expect("Failed to dispatch inlay hints");
        });
    assert_eq!(hints.len(), 2);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}
