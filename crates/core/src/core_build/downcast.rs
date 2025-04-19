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

use crate::{core_ast::core::AstSymbol, document::Document, errors::AstError, parsers::Parsers};

use super::{
    buildable::Buildable,
    symbol::{MaybePendingSymbol, PendingSymbol},
};

/// Trait for downcasting a [`Buildable`] into an [`AstSymbol`].
pub trait TryDownCast<
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<(&'a T, &'a Document, &'static Parsers), Error = AstError>,
>
{
    type Output;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        document: &Document,
        field_name: &str,
        field_range: &std::ops::Range<usize>,
        input_name: &str,
    ) -> Result<Self::Output, AstError>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<(&'a T, &'a Document, &'static Parsers), Error = AstError>,
{
    type Output = Y;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        document: &Document,
        field_name: &str,
        field_range: &std::ops::Range<usize>,
        input_name: &str,
    ) -> Result<Self::Output, AstError> {
        Y::try_from((
            self.0
                .borrow()
                .downcast_ref::<T>()
                .ok_or(AstError::InvalidSymbol {
                    range: field_range.clone(),
                    field_name: field_name.to_string(),
                    parent_name: input_name.to_string(),
                    query: parsers.core.capture_names()[self.get_query_index()],
                })?,
            document,
            parsers,
        ))
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<(&'a T, &'a Document, &'static Parsers), Error = AstError>,
{
    type Output = Option<Y>;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        document: &Document,
        field_name: &str,
        field_range: &std::ops::Range<usize>,
        input_name: &str,
    ) -> Result<Self::Output, AstError> {
        self.as_ref().as_ref().map_or(Ok(None), |pending| {
            pending
                .try_downcast(parsers, document, field_name, field_range, input_name)
                .map(Some)
        })
    }
}

impl<T, Y, V> TryDownCast<Y, V> for Vec<T>
where
    T: TryDownCast<Y, V, Output = V>,
    Y: Buildable,
    V: AstSymbol + for<'a> TryFrom<(&'a Y, &'a Document, &'static Parsers), Error = AstError>,
{
    type Output = Vec<V>;

    fn try_downcast(
        &self,
        parsers: &'static Parsers,
        document: &Document,
        field_name: &str,
        field_range: &std::ops::Range<usize>,
        input_name: &str,
    ) -> Result<Self::Output, AstError> {
        self.iter()
            .map(|item| item.try_downcast(parsers, document, field_name, field_range, input_name))
            .collect::<Result<Vec<_>, AstError>>()
    }
}
