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
use auto_lsp::core::dispatch;
use crate::generated::{
    CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module,
};
use auto_lsp::core::ast::{AstNode};
use auto_lsp::lsp_types::CodeActionOrCommand;
use auto_lsp::{anyhow, lsp_types};
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb, File};

pub fn dispatch_code_actions(
    db: &impl BaseDatabase,
    file: File,
    node: &dyn AstNode,
    builder: &mut Vec<CodeActionOrCommand>,
) -> anyhow::Result<()> {
    dispatch!(
        node,
        [
            FunctionDefinition => build_code_actions(db, file, builder)
        ]
    );
    Ok(())
}

impl FunctionDefinition {
    fn build_code_actions(
        &self,
        db: &impl BaseDatabase,
        file: File,
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
