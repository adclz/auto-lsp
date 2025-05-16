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
use auto_lsp::core::ast::{AstNode};
use auto_lsp::core::document::Document;
use auto_lsp::{anyhow, lsp_types};
use auto_lsp::core::salsa::db::{BaseDatabase, BaseDb, File};
use auto_lsp::lsp_types::{CodeActionOrCommand, CodeLens};
use auto_lsp::core::dispatch;

pub fn dispatch_code_lenses(
    db: &impl BaseDatabase,
    file: File,
    node: &dyn AstNode,
    builder: &mut Vec<CodeLens>,
) -> anyhow::Result<()> {
    dispatch!(
        node,
        [
            FunctionDefinition => build_code_lenses(db, file, builder)
        ]
    );
    Ok(())
}

impl FunctionDefinition {
    fn build_code_lenses(
        &self,
        db: &impl BaseDatabase,
        file: File,
        acc: &mut Vec<lsp_types::CodeLens>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeLens {
            range: self.name.get_lsp_range(),
            command: None,
            data: None,
        });
        Ok(())
    }
}
