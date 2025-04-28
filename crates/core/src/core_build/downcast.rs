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

use crate::{
    build::TryFromParams, core_ast::core::AstSymbol, document::Document, errors::AstError,
    parsers::Parsers,
};
use std::sync::Arc;

use super::{
    buildable::Buildable,
    symbol::{MaybePendingSymbol, PendingSymbol},
};

/// Trait for downcasting a [`Buildable`] into an [`AstSymbol`].
pub trait TryDownCast<
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<TryFromParams<'a, T>, Error = AstError>,
>
{
    type Output;

    fn try_downcast(
        &self,
        parent_id: &Option<usize>,
        document: &Document,
        parsers: &'static Parsers,
        all_nodes: &mut Vec<Arc<dyn AstSymbol>>,
    ) -> Result<Self::Output, AstError>;
}

impl<T, Y> TryDownCast<T, Y> for PendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<TryFromParams<'a, T>, Error = AstError>,
{
    type Output = Arc<Y>;

    fn try_downcast(
        &self,
        parent_id: &Option<usize>,
        document: &Document,
        parsers: &'static Parsers,
        all_nodes: &mut Vec<Arc<dyn AstSymbol>>,
    ) -> Result<Self::Output, AstError> {
        let mut result = Y::try_from((
            self.borrow()
                .downcast_ref::<T>()
                .ok_or(AstError::InvalidSymbol {
                    range: self.get_range().clone(),
                    query: parsers.core.capture_names()[self.get_query_index()],
                })?,
            &None,
            document,
            parsers,
            all_nodes,
        ))?;
        result.get_mut_data().parent = parent_id.clone();
        result.get_mut_data().id = self.get_id();

        let arc = Arc::new(result);
        all_nodes.push(Arc::clone(&arc) as _);
        Ok(arc)
    }
}

impl<T, Y> TryDownCast<T, Y> for MaybePendingSymbol
where
    T: Buildable,
    Y: AstSymbol + for<'a> TryFrom<TryFromParams<'a, T>, Error = AstError>,
{
    type Output = Option<Arc<Y>>;

    fn try_downcast(
        &self,
        parent_id: &Option<usize>,
        document: &Document,
        parsers: &'static Parsers,
        all_nodes: &mut Vec<Arc<dyn AstSymbol>>,
    ) -> Result<Self::Output, AstError> {
        self.as_ref().as_ref().map_or(Ok(None), |pending| {
            pending
                .try_downcast(parent_id, document, parsers, all_nodes)
                .map(Some)
        })
    }
}

impl<T, Y, V> TryDownCast<Y, V> for Vec<T>
where
    T: TryDownCast<Y, V, Output = Arc<V>>,
    Y: Buildable,
    V: AstSymbol + for<'a> TryFrom<TryFromParams<'a, Y>, Error = AstError>,
{
    type Output = Vec<Arc<V>>;

    fn try_downcast(
        &self,
        parent_id: &Option<usize>,
        document: &Document,
        parsers: &'static Parsers,
        all_nodes: &mut Vec<Arc<dyn AstSymbol>>,
    ) -> Result<Self::Output, AstError> {
        self.iter()
            .map(|item| item.try_downcast(parent_id, document, parsers, all_nodes))
            .collect::<Result<Vec<_>, AstError>>()
    }
}
