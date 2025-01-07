use crate::{session::workspace::Workspace, Session};
use lsp_types::DidChangeTextDocumentParams;
use tree_sitter::{InputEdit, Point, Tree};

pub fn edit_tree(
    workspace: &mut Workspace,
    event: &DidChangeTextDocumentParams,
) -> anyhow::Result<Tree> {
    let provider = &workspace.parsers.cst_parser;
    let doc = &workspace.document;
    let tree = &mut workspace.cst;

    event.content_changes.iter().for_each(|edit| {
        let edit_range = edit.range.unwrap();

        let range_offset = doc.offset_at(edit_range.start) as usize;
        let start_byte = range_offset;
        let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
        let new_end_byte = range_offset + edit.text.len();

        let start_position = doc.position_at(start_byte as u32);
        let old_end_position = doc.position_at(old_end_byte as u32);
        let new_end_position = doc.position_at(new_end_byte as u32);

        tree.edit(&InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: Point {
                row: start_position.line as usize,
                column: start_position.character as usize,
            },
            old_end_position: Point {
                row: old_end_position.line as usize,
                column: old_end_position.character as usize,
            },
            new_end_position: Point {
                row: new_end_position.line as usize,
                column: new_end_position.character as usize,
            },
        });
    });

    let new_tree = provider
        .parser
        .write()
        .map_err(|_| {
            anyhow::format_err!(
                "Parser lock is poisoned while editing cst of document {}",
                event.text_document.uri
            )
        })?
        .parse(doc.get_content(None), Some(&tree))
        .ok_or(anyhow::format_err!(
            "Tree sitter failed to edit cst of document {}",
            event.text_document.uri
        ))?;

    Ok(new_tree)
}
