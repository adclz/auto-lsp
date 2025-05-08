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
use auto_lsp::core::ast::{AstNode, GetHover};
use auto_lsp::core::document::Document;
use auto_lsp::{anyhow, lsp_types};
use crate::generated::{Identifier, PassStatement};

impl GetHover for PassStatement {
    fn get_hover(&self, _doc: &Document) -> anyhow::Result<Option<lsp_types::Hover>> {
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

impl GetHover for Identifier {
    fn get_hover(&self, doc: &Document) -> anyhow::Result<Option<lsp_types::Hover>> {
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("hover {}", self.get_text(doc.texter.text.as_bytes())?),
            }),
            range: None,
        }))
    }
}