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
use auto_lsp::core::ast::AstNode;
use auto_lsp::lsp_types::HoverParams;
use auto_lsp::{
    default::{db::tracked::get_ast, db::BaseDatabase},
    lsp_types::{self, Url},
};
use rstest::{fixture, rstest};

use crate::capabilities::hover::hover;
use crate::{
    db::create_python_db,
    generated::{CompoundStatement, CompoundStatement_SimpleStatement, Module},
};

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
fn foo_bar_hover(foo_bar: impl BaseDatabase) {
    let file = foo_bar
        .get_file(&Url::parse("file:///test0.py").unwrap())
        .unwrap();
    let ast = get_ast(&foo_bar, file);
    let root = ast.get_root().unwrap();
    let module = root.downcast_ref::<Module>().unwrap();

    let foo = &module.children[0];
    if let CompoundStatement_SimpleStatement::CompoundStatement(
        CompoundStatement::FunctionDefinition(foo),
    ) = foo.cast(ast)
    {
        let foo_name = &foo.name;

        let foo_hover = hover(
            &foo_bar,
            HoverParams {
                text_document_position_params: lsp_types::TextDocumentPositionParams {
                    text_document: lsp_types::TextDocumentIdentifier {
                        uri: file.url(&foo_bar).clone(),
                    },
                    position: foo_name.cast(ast).get_start_position(),
                },
                work_done_progress_params: Default::default(),
            },
        )
        .expect("Failed to dispatch hover");

        assert_eq!(
            foo_hover.unwrap().contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "hover foo".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }

    let bar = &module.children[1];

    if let CompoundStatement_SimpleStatement::CompoundStatement(
        CompoundStatement::FunctionDefinition(foo),
    ) = bar.cast(ast)
    {
        let bar_name = &foo.name;

        let bar_hover = hover(
            &foo_bar,
            HoverParams {
                text_document_position_params: lsp_types::TextDocumentPositionParams {
                    text_document: lsp_types::TextDocumentIdentifier {
                        uri: file.url(&foo_bar).clone(),
                    },
                    position: bar_name.cast(ast).get_start_position(),
                },
                work_done_progress_params: Default::default(),
            },
        )
        .expect("Failed to dispatch hover");

        assert_eq!(
            bar_hover.unwrap().contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: "hover bar".into(),
            })
        );
    } else {
        panic!("Expected function statement");
    }
}
