use crate::generated::FunctionDefinition;
use auto_lsp::anyhow;
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::{get_ast, ParsedAst};
use auto_lsp::default::db::BaseDatabase;
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

    let ast = get_ast(db, file);
    ast.iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_inlay_hints(db, file, ast, &mut acc)
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
        ast: &ParsedAst,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        let doc = file.document(db);

        let range = self.get_range();
        let name = format!(
            "[{} {}] - {}",
            range.start_byte,
            range.end_byte,
            self.name.cast(ast).get_text(doc.as_bytes())?
        );
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(name),
            position: self.name.cast(ast).get_start_position(),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None,
        });
        Ok(())
    }
}
