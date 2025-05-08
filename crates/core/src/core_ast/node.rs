use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use std::cmp::Ordering;
use std::sync::Arc;
use crate::{ast::BuildDocumentSymbols, errors::PositionError};
use crate::ast::{BuildCodeActions, BuildCodeLenses, BuildCompletionItems, BuildInlayHints, BuildSemanticTokens, BuildTriggeredCompletionItems, GetGoToDeclaration, GetGoToDefinition, GetHover};

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
    + BuildCodeActions {
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
        let range = range.start_byte as usize..range.end_byte as usize;
        match source_code.get(range.start..range.end) {
            Some(text) => match std::str::from_utf8(text) {
                Ok(text) => Ok(text),
                Err(utf8_error) => Err(PositionError::UTF8Error { range, utf8_error }),
            },
            None => Err(PositionError::WrongTextRange { range }),
        }
    }

    fn get_parent<'a>(&'a self, nodes: &'a Vec<Arc<dyn AstNode>>) -> Option<&'a Arc<dyn AstNode>> {
        let self_range = self.get_range();
        let mut low = 0;
        let mut high = nodes.len();
        let mut result: Option<&'a Arc<dyn AstNode>> = None;

        while low < high {
            let mid = (low + high) / 2;
            let node = &nodes[mid];
            let range = node.get_range();

            if range.start_byte <= self_range.start_byte && range.end_byte >= self_range.end_byte {
                // Potential parent found — go deeper to find tighter parent
                result = Some(node);
                low = mid + 1;
            } else {
                if range.start_byte > self_range.start_byte {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            }
        }

        result
    }

}

impl_downcast!(AstNode);

impl PartialEq for dyn AstNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_range().eq(&other.get_range())
    }
}

impl Eq for dyn AstNode {}

impl PartialOrd for dyn AstNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn AstNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.get_range();
        let b = other.get_range();

        (a.start_byte, std::cmp::Reverse(a.end_byte))
            .cmp(&(b.start_byte, std::cmp::Reverse(b.end_byte)))
    }
}
