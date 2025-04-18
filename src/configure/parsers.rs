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

use tree_sitter::{Language, Query};

/// Create the parsers with any given language and queries.
///
/// This generates a static map named  of parsers that can be used to parse source code in the root.
///
/// To determine which parser to use for a source code, the server will check the language ID against the keys in the map generated by this macro
///
/// # Example
/// ```rust
/// # use auto_lsp::seq;
/// # use auto_lsp::configure_parsers;
/// # use auto_lsp::core::ast::*;
/// static CORE_QUERY: &'static str = "
/// (module) @module
/// (function_definition
///    name: (identifier) @function.name) @function
/// ";
///
/// #[seq(query = "module")]
/// struct Module {}
///
/// configure_parsers!(
///     PARSER_LIST,
///     "python" => {
///         language: tree_sitter_python::LANGUAGE,
///         core: CORE_QUERY,
///         ast_root: Module
///     }
/// );
/// ```
#[macro_export]
macro_rules! configure_parsers {
    ($parser_list_name: ident,
        $($extension: expr => {
            language: $language: path,
            core: $core: path,
            ast_root: $root: ident
        }),*) => {
        pub static $parser_list_name: std::sync::LazyLock<std::collections::HashMap<&str, $crate::core::parsers::Parsers>> =
            std::sync::LazyLock::new(|| {
                let mut map = std::collections::HashMap::new();
                $(
                let data = $crate::configure::parsers::create_parser($language, $core);
                map.insert(
                    $extension, $crate::core::parsers::Parsers {
                        parser: data.0,
                        language: data.1,
                        core: data.2,
                        ast_parser: |
                            db: &dyn $crate::core::salsa::db::BaseDatabase,
                            parsers: &'static $crate::core::parsers::Parsers,
                            document: &$crate::core::document::Document | {
                            use $crate::core::build::InvokeParser;

                            Ok::<$crate::core::ast::DynSymbol, $crate::core::errors::AutoLspError>(
                                $crate::core::ast::Symbol::from($root::parse_symbol(db, parsers, document)?).into(),
                            )
                        },
                    }
                );
                ),*
                map
            });
    };
}

#[doc(hidden)]
pub fn create_parser(
    language: tree_sitter_language::LanguageFn,
    core: &'static str,
) -> (parking_lot::RwLock<tree_sitter::Parser>, Language, Query) {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language.into()).unwrap();
    let language = tree_sitter::Language::new(language);
    let core = tree_sitter::Query::new(&language, core).unwrap();

    (parking_lot::RwLock::new(parser), language, core)
}
