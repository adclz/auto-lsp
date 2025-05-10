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
use auto_lsp::core::ast::{AstNode, BuildCodeLenses};
use auto_lsp::core::document::Document;
use auto_lsp::{anyhow, lsp_types};

impl BuildCodeLenses for Module {
    fn build_code_lenses(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeLens>,
    ) -> anyhow::Result<()> {
        self.children.build_code_lenses(doc, acc)
    }
}

impl BuildCodeLenses for CompoundStatement_SimpleStatement {
    fn build_code_lenses(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeLens>,
    ) -> anyhow::Result<()> {
        match self {
            CompoundStatement_SimpleStatement::CompoundStatement(
                CompoundStatement::FunctionDefinition(f),
            ) => f.build_code_lenses(doc, acc),
            _ => Ok(()),
        }
    }
}

impl BuildCodeLenses for FunctionDefinition {
    fn build_code_lenses(
        &self,
        _doc: &Document,
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
