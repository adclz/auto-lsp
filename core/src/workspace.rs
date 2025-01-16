use crate::{
    core_ast::symbol::{DynSymbol, WeakSymbol},
    core_build::main_builder::MainBuilder,
};
use lsp_types::Diagnostic;
use parking_lot::RwLock;
use texter::core::text::Text;
use tree_sitter::{Language, Parser, Point, Query, Tree};

pub struct Queries {
    pub core: Query,
    pub comments: Option<Query>,
    pub fold: Option<Query>,
    pub highlights: Option<Query>,
}

pub struct CstParser {
    pub parser: RwLock<Parser>,
    pub language: Language,
    pub queries: Queries,
}

pub type StaticBuildableFn = fn(
    &mut MainBuilder,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

pub struct Parsers {
    pub cst_parser: CstParser,
    pub ast_parser: StaticBuildableFn,
}

pub struct Document {
    pub document: Text,
    pub cst: Tree,
}

impl Document {
    pub fn new(document: Text, cst: Tree) -> Self {
        Self { document, cst }
    }

    /// Get the smallest node within the root node that spans the given range.
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

    /// Get the smallest node within root node that spans the given range.
    pub fn position_at(&self, offset: usize) -> Option<lsp_types::Position> {
        self.cst
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|pos| lsp_types::Position {
                line: pos.start_position().row as u32,
                character: pos.start_position().column as u32,
            })
    }

    /// Get the smallest node's range within root node that spans the given range.
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

    /// Get the byte offset of the given position.
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
