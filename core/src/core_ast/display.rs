use crate::ast::{AstSymbol, Symbol};
use std::fmt;

/// Trait for types that can be displayed with indentation
pub trait IndentedDisplay {
    /// Same as fmt::Display::fmt, but with an additional indentation parameter
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result;
}

impl<T: AstSymbol> IndentedDisplay for Symbol<T> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        self.read().fmt_with_indent(f, indent)
    }
}

impl<T: AstSymbol> IndentedDisplay for Option<Symbol<T>> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        if let Some(value) = self {
            value.fmt_with_indent(f, indent)
        } else {
            Ok(())
        }
    }
}

impl<T: AstSymbol> IndentedDisplay for Vec<Symbol<T>> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        for item in self {
            item.read().fmt_with_indent(f, indent)?;
        }
        Ok(())
    }
}
