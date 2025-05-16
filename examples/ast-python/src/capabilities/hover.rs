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
use crate::generated::{
    CompoundStatement_SimpleStatement, Identifier, PassStatement, SimpleStatement,
};
use auto_lsp::core::ast::{AstNode};
use auto_lsp::core::document::Document;
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb, File};
use auto_lsp::{anyhow, lsp_types};
use auto_lsp::core::dispatch;

pub fn dispatch_hover(db: &impl BaseDatabase, file: File, node: &dyn AstNode) -> anyhow::Result<Option<lsp_types::Hover>> {
    dispatch!(node, [
        PassStatement => get_hover(db, file),
        Identifier => get_hover(db, file)
    ]);
    Ok(None)
}

impl PassStatement {
    fn get_hover(&self, db: &impl BaseDatabase, file: File) -> anyhow::Result<Option<lsp_types::Hover>> {
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#
                    .into(),
            }),
            range: None,
        }))
    }
}

impl Identifier {
    fn get_hover(&self, db: &impl BaseDatabase, file: File) -> anyhow::Result<Option<lsp_types::Hover>> {
        let doc = file.document(db).read();
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("hover {}", self.get_text(doc.texter.text.as_bytes())?),
            }),
            range: None,
        }))
    }
}
