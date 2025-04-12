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
use auto_lsp::core::ast::BuildCodeActions;
use auto_lsp_core::document::Document;
use lsp_types::CodeAction;

impl BuildCodeActions for Module {
    fn build_code_actions(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        for statement in &self.statements {
            statement.read().build_code_actions(doc, acc)?;
        }
        Ok(())
    }
}

impl BuildCodeActions for Function {
    fn build_code_actions(
        &self,
        _doc: &Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeActionOrCommand::CodeAction(CodeAction {
            title: "A code action".to_string(),
            kind: None,
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        }));
        Ok(())
    }
}
