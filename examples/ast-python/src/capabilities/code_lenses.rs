use crate::generated::FunctionDefinition;
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::{ParsedAst, get_ast};
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

    let ast = get_ast(db, file);
    ast.iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_code_lenses(db, file, ast, &mut acc)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(acc))
}

impl FunctionDefinition {
    fn build_code_lenses(
        &self,
        db: &impl BaseDatabase,
        file: File,
        ast: &ParsedAst,
        acc: &mut Vec<lsp_types::CodeLens>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeLens {
            range: self.name.cast(ast).get_lsp_range(file.document(db))?,
            command: None,
            data: None,
        });
        Ok(())
    }
}
