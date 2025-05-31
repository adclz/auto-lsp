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

use crate::capabilities::inlay_hints::inlay_hints;
use crate::db::create_python_db;
use auto_lsp::lsp_types;
use auto_lsp::lsp_types::Url;
use auto_lsp::{default::db::BaseDatabase, lsp_types::InlayHintParams};
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
fn foo_bar_inlay_hints(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();

    let hints = inlay_hints(
        &foo_bar,
        InlayHintParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: file.url(&foo_bar).clone(),
            },
            range: lsp_types::Range {
                start: lsp_types::Position::new(0, 0),
                end: lsp_types::Position::new(0, 0),
            },
            work_done_progress_params: Default::default(),
        },
    )
    .unwrap()
    .unwrap();

    assert_eq!(hints.len(), 2);

    assert_eq!(hints[0].kind, Some(lsp_types::InlayHintKind::TYPE));
    assert_eq!(hints[1].kind, Some(lsp_types::InlayHintKind::TYPE));
}
