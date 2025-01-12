use crate::symbol::{DynSymbol, WeakSymbol};
use lsp_types::Diagnostic;
use std::sync::RwLock;
use texter::core::text::Text;
use tree_sitter::{Language, Parser, Point, Query, Tree};

use crate::builders::BuilderParams;

pub struct Queries {
    pub comments: Query,
    pub fold: Query,
    pub highlights: Query,
    pub outline: Query,
}

pub struct CstParser {
    pub parser: RwLock<Parser>,
    pub language: Language,
    pub queries: Queries,
}

pub type StaticBuilderFn = fn(
    &mut BuilderParams,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub struct Parsers {
    pub cst_parser: CstParser,
    pub ast_parser: StaticBuilderFn,
}

pub struct Document {
    pub document: Text,
    pub cst: Tree,
}

impl Document {
    pub fn new(document: Text, cst: Tree) -> Self {
        Self { document, cst }
    }

    pub fn descendant_at_position(
        &self,
        position: lsp_types::Position,
    ) -> Option<tree_sitter::Node<'_>> {
        let position = Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        self.cst
            .root_node()
            .descendant_for_point_range(position, position)
    }

    pub fn position_at(&self, offset: usize) -> Option<lsp_types::Position> {
        self.cst
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Position {
                line: pos.start_position().row as u32,
                character: pos.start_position().column as u32,
            })
    }

    pub fn range_at(&self, offset: usize) -> Option<lsp_types::Range> {
        self.cst
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Range {
                start: lsp_types::Position {
                    line: pos.start_position().row as u32,
                    character: pos.start_position().column as u32,
                },
                end: lsp_types::Position {
                    line: pos.end_position().row as u32,
                    character: pos.end_position().column as u32,
                },
            })
    }

    pub fn offset_at(&self, position: lsp_types::Position) -> Option<usize> {
        match self.cst.root_node().descendant_for_point_range(
            Point {
                row: position.line as usize,
                column: position.character as usize,
            },
            Point {
                row: position.line as usize,
                column: position.character as usize,
            },
        ) {
            Some(node) => Some(node.start_byte()),
            None => None,
        }
    }
}

pub struct Workspace {
    pub parsers: &'static Parsers,
    pub document: Document,
    pub errors: Vec<Diagnostic>,
    pub ast: Option<DynSymbol>,
    pub unsolved_checks: Vec<WeakSymbol>,
    pub unsolved_references: Vec<WeakSymbol>,
}
