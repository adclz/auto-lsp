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

#[cfg(any(feature = "html", test))]
pub mod html_utils;
#[cfg(any(feature = "html", test))]
pub mod html_workspace;
#[cfg(any(feature = "python", test))]
pub mod python_utils;
#[cfg(any(feature = "python", test))]
pub mod python_workspace;

#[cfg(test)]
pub mod ast;
#[cfg(test)]
pub mod code_actions;
#[cfg(test)]
pub mod code_lenses;
#[cfg(test)]
pub mod completion_items;
#[cfg(test)]
pub mod document_links;
#[cfg(test)]
pub mod document_symbols;
#[cfg(test)]
pub mod hover;
#[cfg(test)]
pub mod html_corpus;
#[cfg(test)]
pub mod inlay_hints;
#[cfg(test)]
pub mod iter;
#[cfg(test)]
pub mod proc_macros;
#[cfg(test)]
pub mod python_corpus;
#[cfg(test)]
pub mod salsa;
#[cfg(test)]
pub mod semantic_tokens;
#[cfg(test)]
pub mod type_errors;
