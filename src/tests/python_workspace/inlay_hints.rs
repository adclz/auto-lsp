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

use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, BuildInlayHints, GetSymbolData};
use auto_lsp_core::document::Document;

impl BuildInlayHints for Module {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        self.statements.build_inlay_hints(doc, acc)
    }
}

impl BuildInlayHints for Function {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        let range = self.get_range();
        let read = self.name.read();
        let name = format!(
            "[{} {}] - {}",
            range.start,
            range.end,
            self.name.read().get_text(doc.texter.text.as_bytes())?
        );
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(name),
            position: read.get_start_position(doc)?,
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None,
        });
        Ok(())
    }
}
