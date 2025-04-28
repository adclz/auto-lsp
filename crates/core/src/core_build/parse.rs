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

use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::DynSymbol;
use crate::document::Document;
use crate::errors::AstError;
use crate::errors::ParseError;
use crate::errors::ParseErrorAccumulator;
use crate::parsers::Parsers;
use crate::salsa::db::BaseDatabase;
use crate::salsa::db::BaseDb;
use crate::salsa::tracked::get_ast;
use crate::{
    ast::AstSymbol,
    build::{Buildable, Queryable},
};
use ariadne::{ColorGenerator, Report, ReportKind, Source};
use lsp_types::Url;
use texter::core::text::Text;

use super::stack_builder::StackBuilder;

/// Trait for invoking the stack builder
///
/// This trait is implemented for all types that implement [`Buildable`] and [`Queryable`].
pub trait InvokeParser<
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFrom<
            (
                &'a T,
                &'a Option<usize>,
                &'a Document,
                &'static Parsers,
                &'a HashMap<usize, usize>,
                &'a mut Vec<Arc<dyn AstSymbol>>,
            ),
            Error = AstError,
        >,
>
{
    /// Creates a symbol.
    ///
    /// This method internally initializes a stack builder to build the AST and derive a symbol
    /// of type Y.
    fn parse_symbol(
        db: &dyn BaseDatabase,
        parsers: &'static Parsers,
        document: &Document,
    ) -> Result<Vec<Arc<dyn AstSymbol>>, ParseError>;
}

impl<T, Y> InvokeParser<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'b> TryFrom<
            (
                &'b T,
                &'b Option<usize>,
                &'b Document,
                &'static Parsers,
                &'b HashMap<usize, usize>,
                &'b mut Vec<Arc<dyn AstSymbol>>,
            ),
            Error = AstError,
        >,
{
    fn parse_symbol(
        db: &dyn BaseDatabase,
        parsers: &'static Parsers,
        document: &Document,
    ) -> Result<Vec<Arc<dyn AstSymbol>>, ParseError> {
        StackBuilder::<T>::new(db, document, parsers).create_symbol::<Y>()
    }
}

/// Function signature for invoking the stack builder.
///
/// This type alias is useful for mapping language IDs to specific parsers,
/// avoiding ambiguity.
pub type InvokeParserFn = fn(
    &dyn BaseDatabase,
    &'static Parsers,
    &Document,
) -> Result<Vec<Arc<dyn AstSymbol>>, ParseError>;

pub type TestParseResult<E = AriadneReport> = Result<(), Box<E>>;

pub struct AriadneReport {
    pub result: Report<'static>,
    pub cache: Source<&'static str>,
}

impl std::fmt::Debug for AriadneReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for AriadneReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = Vec::<u8>::new();
        self.result.write(self.cache.clone(), &mut output).unwrap();
        write!(f, "{}", String::from_utf8_lossy(&output).into_owned())
    }
}

pub trait TryParse<
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFrom<(
            &'a T,
            &'a Option<usize>,
            &'a Document,
            &'static Parsers,
            &'a HashMap<usize, usize>,
            &'a mut Vec<Arc<dyn AstSymbol>>,
        )>,
    Error = AstError,
>
{
    /// Parses the provided test code and validates the AST symbol construction.
    ///
    /// # Arguments
    /// - `test_code`: The code to be parsed and analyzed.
    /// - `parsers`: A reference to the parsers for syntax tree generation.
    ///
    /// # Returns [`TestParseResult`]
    fn test_parse(test_code: &'static str, parsers: &'static Parsers) -> TestParseResult;
}

impl<T, Y> TryParse<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol
        + for<'a> TryFrom<
            (
                &'a T,
                &'a Option<usize>,
                &'a Document,
                &'static Parsers,
                &'a HashMap<usize, usize>,
                &'a mut Vec<Arc<dyn AstSymbol>>,
            ),
            Error = AstError,
        >,
{
    fn test_parse(test_code: &'static str, parsers: &'static Parsers) -> TestParseResult {
        let mut db = BaseDb::default();
        let url = Url::parse("file://test.txt").unwrap();
        let text = Text::new(test_code.to_string());
        let source = Source::from(test_code);

        match db.add_file_from_texter(parsers, &url, text) {
            Ok(_) => {}
            Err(e) => {
                return Err(Box::new(AriadneReport {
                    result: Report::build(ReportKind::Error, 0..test_code.len())
                        .with_message(format!("Failed to create file: {}", e))
                        .finish(),
                    cache: source,
                }));
            }
        }

        let file = db.get_file(&url).unwrap();
        let ast = get_ast(&db, file);
        let diagnostics: Vec<&ParseErrorAccumulator> =
            get_ast::accumulated::<ParseErrorAccumulator>(&db, file);

        match diagnostics.is_empty() {
            false => {
                let mut colors = ColorGenerator::new();
                let mut report = Report::build(ReportKind::Error, 0..test_code.len())
                    .with_message(format!("Parsing failed: {} error(s)", diagnostics.len()));

                for diagnostic in &diagnostics {
                    diagnostic.to_label(&source, &mut colors, &mut report);
                }

                if let Some(ast) = ast.get(0) {
                    report.add_note(format!("{}", ast));
                }

                Err(Box::new(AriadneReport {
                    result: report.finish(),
                    cache: source,
                }))
            }
            true => Ok(()),
        }
    }
}
