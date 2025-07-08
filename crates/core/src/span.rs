use std::ops::Deref;

/// A newtype for [`tree_sitter::Range`].
///
/// Useful for storing and converting ranges from Tree-sitter to LSP types.
///
/// This is typically used when storing positions in an intermediate representation (IR) of the AST.
///
/// Tree-sitter ranges also retain byte offsets, which are not available in [`lsp_types::Range`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span(pub(crate) tree_sitter::Range);

impl Span {
    pub fn lsp(&self) -> lsp_types::Range {
        self.into()
    }

    pub fn ts(&self) -> &tree_sitter::Range {
        &self.0
    }
}

impl Deref for Span {
    type Target = tree_sitter::Range;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<tree_sitter::Range> for Span {
    fn from(range: tree_sitter::Range) -> Self {
        Span(range)
    }
}

impl<'a> From<&'a tree_sitter::Range> for Span {
    fn from(range: &'a tree_sitter::Range) -> Self {
        Span(range.clone())
    }
}

impl From<&Span> for lsp_types::Range {
    fn from(range: &Span) -> Self {
        lsp_types::Range {
            start: lsp_types::Position::new(
                range.0.start_point.row as u32,
                range.0.start_point.column as u32,
            ),
            end: lsp_types::Position::new(
                range.0.end_point.row as u32,
                range.0.end_point.column as u32,
            ),
        }
    }
}

impl From<Span> for lsp_types::Range {
    fn from(span: Span) -> Self {
        (&span).into()
    }
}

impl PartialEq<tree_sitter::Range> for Span {
    fn eq(&self, other: &tree_sitter::Range) -> bool {
        self == other
    }
}

impl PartialEq<lsp_types::Range> for Span {
    fn eq(&self, other: &lsp_types::Range) -> bool {
        lsp_types::Range::from(self) == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_equality() {
        let range = tree_sitter::Range {
            start_byte: 0,
            end_byte: 10,
            start_point: tree_sitter::Point::new(10, 0),
            end_point: tree_sitter::Point::new(0, 10),
        };

        let span = Span(range);

        assert_eq!(span, range);
        assert_eq!(
            span,
            lsp_types::Range {
                start: lsp_types::Position::new(10, 0),
                end: lsp_types::Position::new(0, 10),
            }
        );
    }
}
