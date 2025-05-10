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
use auto_lsp::anyhow;
use auto_lsp::core::ast::{AstNode, BuildInlayHints};
use auto_lsp::core::document::Document;
use crate::generated::{CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module};

impl BuildInlayHints for Module {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        self.children.build_inlay_hints(doc, acc)
    }
}

impl BuildInlayHints for CompoundStatement_SimpleStatement {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        match self {
            CompoundStatement_SimpleStatement::CompoundStatement(
                CompoundStatement::FunctionDefinition(f),
            ) => f.build_inlay_hints(doc, acc),
            _ => Ok(()),
        }
    }
}

impl BuildInlayHints for FunctionDefinition {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        let range = self.get_range();
        let name = format!(
            "[{} {}] - {}",
            range.start_byte,
            range.end_byte,
            self.name.get_text(doc.texter.text.as_bytes())?
        );
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(name),
            position: self.name.get_start_position(),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None,
        });
        Ok(())
    }
}