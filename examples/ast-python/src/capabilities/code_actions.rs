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
    CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module,
};
use auto_lsp::core::ast::BuildCodeActions;
use auto_lsp::core::document::Document;
use auto_lsp::lsp_types::CodeActionOrCommand;
use auto_lsp::{anyhow, lsp_types};

impl BuildCodeActions for Module {
    fn build_code_actions(
        &self,
        doc: &auto_lsp::core::document::Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        self.children.build_code_actions(doc, acc)
    }
}

impl BuildCodeActions for CompoundStatement_SimpleStatement {
    fn build_code_actions(
        &self,
        doc: &auto_lsp::core::document::Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        match self {
            CompoundStatement_SimpleStatement::CompoundStatement(
                CompoundStatement::FunctionDefinition(f),
            ) => f.build_code_actions(doc, acc),
            _ => Ok(()),
        }
    }
}

impl BuildCodeActions for FunctionDefinition {
    fn build_code_actions(
        &self,
        _doc: &Document,
        acc: &mut Vec<CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeActionOrCommand::CodeAction(
            lsp_types::CodeAction {
                title: "A code action".to_string(),
                kind: None,
                diagnostics: None,
                edit: None,
                command: None,
                is_preferred: None,
                disabled: None,
                data: None,
            },
        ));
        Ok(())
    }
}
