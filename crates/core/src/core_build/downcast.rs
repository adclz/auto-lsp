use std::sync::Arc;

use lsp_types::{Diagnostic, Url};

use crate::{builder_error, core_ast::core::AstSymbol, document::Document, parsers::Parsers};

use super::{
    buildable::Buildable,
    symbol::{MaybePendingSymbol, PendingSymbol},
};

/// Trait for converting a builder [`Buildable`] into an [`AstSymbol`],
///
/// # Methods
/// - `try_from_builder`: Attempts to create an instance of `Self` from the given builder.
///
/// # Parameters
/// - `value`: The builder instance used to construct the target type.
/// - `document`: The document context.
pub trait TryFromBuilder<T>: Sized
where
    T: Sized,
{
    type Error;

    fn try_from_builder(
        value: T,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
    ) -> Result<Self, Self::Error>;
}

pub trait TryIntoBuilder<T>: Sized {
    type Error;

    fn try_into_builder(
        self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
    ) -> Result<T, Self::Error>;
}

/// Implementation of `TryIntoBuilder` for any type `T` that satisfies the `TryFromBuilder` trait.
impl<T, U> TryIntoBuilder<U> for T
where
    U: TryFromBuilder<T>,
{
    type Error = U::Error;

    fn try_into_builder(
        self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
    ) -> Result<U, Self::Error> {
        U::try_from_builder(self, parsers, url, document)
    }
}

/// Trait for downcasting a [`Buildable`] into an [`AstSymbol`].
///
/// Unlike [`TryFromBuilder`], which builds an entire symbol with its fields, this trait focuses
/// on converting a generic [`Buildable`] into a specific type of [`AstSymbol`]. This operation is
/// typically used for field-level downcasting.
pub trait TryDownCast<
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
>
{
    type Output;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Y;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.0
            .borrow()
            .downcast_ref::<T>()
            .ok_or(builder_error!(
                field_range,
                format!(
                    "Invalid {:?} for {:?}: received: {:?}",
                    field_name,
                    input_name,
                    parsers.tree_sitter.queries.core.capture_names()[self.get_query_index()]
                )
            ))?
            .try_into_builder(parsers, url, document)
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFromBuilder<&'a T, Error = lsp_types::Diagnostic>,
{
    type Output = Option<Y>;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.as_ref().as_ref().map_or(Ok(None), |pending| {
            pending
                .try_downcast(parsers, url, document, field_name, field_range, input_name)
                .map(Some)
        })
    }
}

impl<T, Y, V> TryDownCast<Y, V> for Vec<T>
where
    T: TryDownCast<Y, V, Output = V>,
    Y: Buildable,
    V: AstSymbol + for<'a> TryFromBuilder<&'a Y, Error = lsp_types::Diagnostic>,
{
    type Output = Vec<V>;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        url: &Arc<Url>,
        document: &Document,
        field_name: &str,
        field_range: lsp_types::Range,
        input_name: &str,
    ) -> Result<Self::Output, Diagnostic> {
        self.iter()
            .map(|item| {
                item.try_downcast(parsers, url, document, field_name, field_range, input_name)
            })
            .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()
    }
}
