use crate::ast::DynSymbol;
use crate::document::Document;
use crate::workspace::Workspace;
use crate::{
    ast::AstSymbol,
    build::{Buildable, Queryable, TryFromBuilder},
};
use cfg_if::cfg_if;

use super::stack_builder::StackBuilder;
/// Trait for invoking the stack builder
///
/// This trait is implemented for all types that implement [`Buildable`] and [`Queryable`].
pub trait InvokeParser<
    T: Buildable + Queryable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    /// Creates a symbol.
    ///
    /// This method internally initializes a stack builder to build the AST and derive a symbol
    /// of type Y.
    fn parse_symbol(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, lsp_types::Diagnostic>;

    fn parse_symbols(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Vec<Y>, lsp_types::Diagnostic>;
}

impl<T, Y> InvokeParser<T, Y> for Y
where
    T: Buildable + Queryable,
    Y: AstSymbol + for<'b> TryFromBuilder<&'b T, Error = lsp_types::Diagnostic>,
{
    fn parse_symbol(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Y, lsp_types::Diagnostic> {
        StackBuilder::<T>::new(workspace, document).create_symbol(&range)
    }
    fn parse_symbols(
        workspace: &mut Workspace,
        document: &Document,
        range: Option<std::ops::Range<usize>>,
    ) -> Result<Vec<Y>, lsp_types::Diagnostic> {
        StackBuilder::<T>::new(workspace, document).create_symbols(&range)
    }
}

/// Function signature for invoking the stack builder.
///
/// This type alias is useful for mapping language IDs to specific parsers,
/// helping avoiding ambiguity.
pub type InvokeParserFn = fn(
    &mut Workspace,
    &Document,
    Option<std::ops::Range<usize>>,
) -> Result<DynSymbol, lsp_types::Diagnostic>;

cfg_if!(
    if #[cfg(feature = "miette")] {
        use crate::workspace::Parsers;
        use lsp_types::Url;
        use miette::{diagnostic, miette, Diagnostic, Result, Severity, SourceOffset};
        use thiserror::Error;

        #[derive(Debug, Error, Diagnostic)]
        #[error("{} error(s)", .related.len())]
        struct Errors {
            #[source_code]
            src: String,
            #[related]
            related: Vec<Error>,
        }

        #[derive(Debug, Error, Diagnostic)]
        #[error("{message}")]
        #[diagnostic()]
        struct Error {
            message: String,
            #[label("{message}")]
            location: SourceOffset,
            severity: Option<Severity>,
        }

        impl From<(&String, &lsp_types::Diagnostic)> for Error {
            fn from(input: (&String, &lsp_types::Diagnostic)) -> Self {
                let diag = input.1;
                Self {
                    message: diag.message.clone(),
                    location: SourceOffset::from_location(
                        &input.0,
                        diag.range.start.line as usize,
                        diag.range.end.character as usize,
                    ),
                    severity: diag.severity.map(|f| match f {
                        lsp_types::DiagnosticSeverity::ERROR => Severity::Error,
                        lsp_types::DiagnosticSeverity::WARNING => Severity::Warning,
                        lsp_types::DiagnosticSeverity::INFORMATION => Severity::Advice,
                        lsp_types::DiagnosticSeverity::HINT => Severity::Advice,
                        _ => Severity::Error,
                    }),
                }
            }
        }

        pub trait Parse<
            T: Buildable + Queryable,
            Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
        >
        {
            /// Parses the provided test code and validates the AST symbol construction.
            ///
            /// # Arguments
            /// - `test_code`: The code to be parsed and analyzed.
            /// - `parsers`: A reference to the parsers for syntax tree generation.
            ///
            /// # Returns
            /// - `Ok(())` if the code was successfully parsed and validated.
            /// - `Err(miette::Error)` if any parsing or validation errors occurred.
            fn miette_parse(test_code: &str, parsers: &'static Parsers) -> miette::Result<()>;
        }

        impl<T, Y> Parse<T, Y> for Y
        where
            T: Buildable + Queryable,
            Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
        {
            fn miette_parse(test_code: &str, parsers: &'static Parsers) -> miette::Result<()> {
                let (mut workspace, document) = Workspace::from_utf8(
                    parsers,
                    Url::parse("file://test").unwrap(),
                    test_code.into(),
                )
                .map_err(|e| {
                    miette!(
                        severity = Severity::Error,
                        "Failed to initialize workspace: {e}"
                    )
                })?;

                let mut errors = Errors {
                    src: document.texter.text.clone(),
                    related: vec![],
                };

                let result: Result<Y, lsp_types::Diagnostic> =
                    Y::parse_symbol(&mut workspace, &document, None);

                if let Err(diag) = &result {
                    errors
                        .related
                        .push(Error::from((&document.texter.text, diag)));
                }

                for diag in &workspace.diagnostics {
                    errors
                        .related
                        .push(Error::from((&document.texter.text, diag)));
                }

                match errors.related.is_empty() {
                    false => Err(errors.into()),
                    true => Ok(()),
                }
            }
        }
    }
);
