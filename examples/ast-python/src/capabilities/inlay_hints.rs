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
use crate::generated::FunctionDefinition;
use auto_lsp::anyhow;
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::core::salsa::db::{BaseDatabase, File};
use auto_lsp::core::salsa::tracked::get_ast;
use auto_lsp::lsp_types::{InlayHint, InlayHintParams};

pub fn inlay_hints(
    db: &impl BaseDatabase,
    params: InlayHintParams,
) -> anyhow::Result<Option<Vec<InlayHint>>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut acc = vec![];

    get_ast(db, file).iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_inlay_hints(db, file, &mut acc)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(acc))
}
impl FunctionDefinition {
    fn build_inlay_hints(
        &self,
        db: &impl BaseDatabase,
        file: File,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        let doc = file.document(db);

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
