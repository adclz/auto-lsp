/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

use crate::ast::{AstSymbol, Symbol};
use std::fmt;

/// Trait for types that can be displayed with indentation
pub trait IndentedDisplay {
    /// Same as fmt::Display::fmt, but with an additional indentation parameter
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result;
}

impl<T: AstSymbol> IndentedDisplay for Symbol<T> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        self.0.fmt_with_indent(f, indent)
    }
}

impl<T: AstSymbol> IndentedDisplay for Option<Symbol<T>> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        if let Some(value) = self {
            value.0.fmt_with_indent(f, indent)
        } else {
            Ok(())
        }
    }
}

impl<T: AstSymbol> IndentedDisplay for Vec<Symbol<T>> {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        for item in self {
            item.0.fmt_with_indent(f, indent)?;
        }
        Ok(())
    }
}
