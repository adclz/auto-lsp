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
use crate::db::create_python_db;
use crate::generated::Module;
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::core::{ast::BuildCodeActions, salsa::db::BaseDatabase};
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::Url;
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
fn foo_bar_code_actions(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let document = file.document(&foo_bar).read();
    let root = get_ast(&foo_bar, file).get_root();

    let module = root.as_ref().unwrap().downcast_ref::<Module>().unwrap();

    let mut code_actions = vec![];
    module
        .build_code_actions(&document, &mut code_actions)
        .unwrap();

    assert_eq!(code_actions.len(), 2);

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &code_actions[0] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }

    if let lsp_types::CodeActionOrCommand::CodeAction(code_action) = &code_actions[1] {
        assert_eq!(code_action.title, "A code action");
    } else {
        panic!("Expected a code action");
    }
}
