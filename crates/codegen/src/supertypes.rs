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
use std::sync::{LazyLock, Mutex};
use crate::json::TypeInfo;
use crate::NodeType;

#[derive(Clone, Default)]
pub(crate) struct SuperType {
    pub(crate) variants: Vec<TypeInfo>,
    pub(crate) types: Vec<String>,
}

thread_local! {
        pub(crate) static SUPER_TYPES: LazyLock<Mutex<HashMap<String, SuperType>>> =
            LazyLock::new(Default::default);
}

pub(crate) fn generate_super_type(node: &NodeType) -> SuperType {
    // Get enum variants
    let variants = node
        .subtypes
        .as_ref()
        .map(|subtypes| {
            subtypes.to_vec()
        })
        .unwrap_or_default();

    // Get enum types
    let types = node
        .subtypes
        .as_ref()
        .map(|subtypes| {
            subtypes
                .iter()
                .map(|subtype| subtype.kind.clone())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    SuperType { variants, types }
}