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

use lsp_types::{Position, TextDocumentContentChangeEvent};
use texter::change::{Change, GridIndex};

/// Simple wrapper around [`Change`].
pub struct WrapChange<'a> {
    pub change: Change<'a>,
}

impl<'a> WrapChange<'a> {
    pub fn new(change: Change<'a>) -> Self {
        Self { change }
    }
}

/// Convert a `Position` into a [`GridIndex`].
///
/// ... Since `From` can't be implemented due to orphan rules.
fn grid_into_position(value: Position) -> GridIndex {
    GridIndex {
        row: value.line as usize,
        col: value.character as usize,
    }
}

impl<'a> From<&'a TextDocumentContentChangeEvent> for WrapChange<'a> {
    fn from(value: &'a TextDocumentContentChangeEvent) -> Self {
        let Some(range) = value.range else {
            return WrapChange::new(Change::ReplaceFull((&value.text).into()));
        };

        if value.text.is_empty() {
            return WrapChange::new(Change::Delete {
                start: grid_into_position(range.start),
                end: grid_into_position(range.end),
            });
        }

        if range.start == range.end {
            return WrapChange::new(Change::Insert {
                at: grid_into_position(range.start),
                text: (&value.text).into(),
            });
        }

        WrapChange::new(Change::Replace {
            start: grid_into_position(range.start),
            end: grid_into_position(range.end),
            text: (&value.text).into(),
        })
    }
}
