use std::ops::Range;

use auto_lsp::builders::ast_item::builder;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;
use auto_lsp::{builders::ast_item::localized_builder, traits::ast_item::AstItem};
use lsp_types::{DidChangeTextDocumentParams, Uri};
use tree_sitter::{InputEdit, Parser, Point, Tree};

use crate::{
    globals::{Session, Workspace},
    symbols::symbols::Symbol,
};

pub fn edit_tree(event: &DidChangeTextDocumentParams, uri: &str, session: &mut Session) {
    let workspace = session.workspaces.get_mut(uri).unwrap();
    let doc = &workspace.document;
    let tree = &mut workspace.cst;
    let parser = &mut session.parser;

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

    let new_tree = parser.parse(doc.get_content(None), Some(&tree)).unwrap();
    let ast = builder(
        &session.queries.outline,
        Symbol::query_binder,
        Symbol::builder_binder,
        new_tree.root_node(),
        doc.get_content(None).as_bytes(),
    );

    workspace.cst = new_tree;
    workspace.ast = ast;

    event.content_changes.iter().for_each(|edit| {
        let edit_range = edit.range.unwrap();

        let range_offset = doc.offset_at(edit_range.start) as usize;
        let start_byte = range_offset;
        let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
        let new_end_byte = range_offset + edit.text.len();

        let ast_node = workspace
            .ast
            .iter()
            .find_map(|symbol| symbol.find_at_offset(&range_offset));

        if let Some(node) = ast_node {
            let ast = localized_builder(
                &session.queries.outline,
                Symbol::query_binder,
                workspace.cst.root_node(),
                doc.get_content(None).as_bytes(),
                Range {
                    start: start_byte,
                    end: new_end_byte,
                },
            );

            node.write()
                .unwrap()
                .swap_at_offset(&range_offset, &ast.unwrap());
        }

        let shift = if old_end_byte > start_byte {
            (old_end_byte - start_byte) as i32
        } else {
            (new_end_byte - start_byte) as i32
        };

        /*workspace
        .ast
        .iter_mut()
        .filter(|f| f.is_inside_offset(offset))
        .for_each(|ast| {
            ast.edit_range(shift);
        });*/
    });
}
