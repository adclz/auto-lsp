use super::capabilities::{
    BuildCodeActions, BuildCodeLenses, BuildCompletionItems, BuildDocumentSymbols, BuildInlayHints,
    BuildSemanticTokens, BuildTriggeredCompletionItems, GetGoToDeclaration, GetGoToDefinition,
    GetHover,
};
use crate::errors::PositionError;
use downcast_rs::{impl_downcast, DowncastSync};
use std::cmp::Ordering;
use std::sync::Arc;
use tree_sitter::Node;

pub trait AstNode:
    std::fmt::Debug
    + Send
    + Sync
    + DowncastSync
    + GetGoToDeclaration
    + GetGoToDefinition
    + GetHover
    + BuildDocumentSymbols
    + BuildCodeLenses
    + BuildCompletionItems
    + BuildTriggeredCompletionItems
    + BuildInlayHints
    + BuildSemanticTokens
    + BuildCodeActions
{
    fn contains(node: &Node) -> bool
    where
        Self: Sized;

    fn lower(&self) -> &dyn AstNode;

    fn get_id(&self) -> usize;

    fn get_parent_id(&self) -> Option<usize>;

    fn get_range(&self) -> &tree_sitter::Range;

    fn get_lsp_range(&self) -> lsp_types::Range {
        let range = self.get_range();
        lsp_types::Range {
            start: lsp_types::Position {
                line: range.start_point.row as u32,
                character: range.start_point.column as u32,
            },
            end: lsp_types::Position {
                line: range.end_point.row as u32,
                character: range.end_point.column as u32,
            },
        }
    }

    fn get_start_position(&self) -> lsp_types::Position {
        let range = self.get_range();
        lsp_types::Position {
            line: range.start_point.row as u32,
            character: range.start_point.column as u32,
        }
    }

    fn get_end_position(&self) -> lsp_types::Position {
        let range = self.get_range();
        lsp_types::Position {
            line: range.end_point.row as u32,
            character: range.end_point.column as u32,
        }
    }

    fn get_text<'a>(&self, source_code: &'a [u8]) -> Result<&'a str, PositionError> {
        let range = self.get_range();
        let range = range.start_byte..range.end_byte;
        match source_code.get(range.start..range.end) {
            Some(text) => match std::str::from_utf8(text) {
                Ok(text) => Ok(text),
                Err(utf8_error) => Err(PositionError::UTF8Error { range, utf8_error }),
            },
            None => Err(PositionError::WrongTextRange { range }),
        }
    }

    fn get_parent<'a>(&'a self, nodes: &'a Vec<Arc<dyn AstNode>>) -> Option<&'a Arc<dyn AstNode>> {
        nodes.get(self.get_parent_id()?)
    }
}

impl_downcast!(AstNode);

impl PartialEq for dyn AstNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_range().eq(other.get_range())
    }
}

impl Eq for dyn AstNode {}

impl PartialOrd for dyn AstNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_id().cmp(&other.get_id()))
    }
}

impl Ord for dyn AstNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_id().cmp(&other.get_id())
    }
}
