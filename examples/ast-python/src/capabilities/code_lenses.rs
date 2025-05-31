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
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::{BaseDatabase, File};
use auto_lsp::lsp_types::{CodeLens, CodeLensParams};
use auto_lsp::{anyhow, lsp_types};

pub fn code_lenses(
    db: &impl BaseDatabase,
    params: CodeLensParams,
) -> anyhow::Result<Option<Vec<CodeLens>>> {
    let mut acc = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    get_ast(db, file).iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_code_lenses(db, file, &mut acc)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(acc))
}

impl FunctionDefinition {
    fn build_code_lenses(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
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
