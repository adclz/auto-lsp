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

mod ir;
mod json;
mod output;
mod supertypes;
mod tests;
mod utils;

use crate::json::{NodeType, TypeInfo};
use crate::output::{generate_enum, generate_struct};
use crate::supertypes::{generate_super_type, SuperType, SUPER_TYPES};
use crate::utils::{sanitize_string, sanitize_string_to_pascal};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

/// List of all named rules
pub(crate) static NAMED_RULES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(Default::default);

pub(crate) struct OperatorList {
    index: usize,
    operators: Vec<TypeInfo>,
}

/// List of fields/children that are only composed of operators
pub(crate) static OPERATORS_RULES: LazyLock<Mutex<HashMap<String, OperatorList>>> =
    LazyLock::new(Default::default);

/// List of fields/children that are composed of multiple rules
pub(crate) static INLINE_MULTIPLE_RULES: LazyLock<Mutex<HashMap<String, Vec<TypeInfo>>>> =
    LazyLock::new(Default::default);

/// List of anonymous rules (usually aliases created on the fly)
pub(crate) static ANONYMOUS_TYPES: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(Default::default);

/// Map of node kind to node id
pub(crate) static NODE_ID_FOR_NAMED_NODE: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

pub(crate) static NODE_ID_FOR_UNNAMED_NODE: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

/// Map of field name to field id
pub(crate) static FIELD_ID_FOR_NAME: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

pub fn generate(source: &str, language: &tree_sitter::Language) -> TokenStream {
    let nodes: Vec<NodeType> = serde_json::from_str(source).expect("Invalid JSON");

    let mut output = quote! {
        // Auto-generated file. Do not edit manually.
        #![allow(clippy::all)]
        #![allow(unused)]
        #![allow(dead_code)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]

    };
    for node in &nodes {
        if node.named {
            NAMED_RULES
                .lock()
                .unwrap()
                .push(sanitize_string_to_pascal(&node.kind));
            NODE_ID_FOR_NAMED_NODE.lock().unwrap().insert(
                node.kind.clone(),
                language.id_for_node_kind(&node.kind, true),
            );
            if let Some(fields) = &node.fields {
                fields.iter().for_each(|(field_name, _)| {
                    let field_id = language.field_id_for_name(field_name);
                    FIELD_ID_FOR_NAME
                        .lock()
                        .unwrap()
                        .insert(field_name.clone(), field_id.unwrap().get());
                });
            }
        } else {
            NODE_ID_FOR_UNNAMED_NODE.lock().unwrap().insert(
                node.kind.clone(),
                language.id_for_node_kind(&node.kind, false),
            );
        }
        if node.is_supertype() {
            SUPER_TYPES.with(|s| {
                s.lock()
                    .unwrap()
                    .insert(node.kind.clone(), generate_super_type(node));
            });
        }
    }

    SUPER_TYPES.with(|s| {
        let mut super_types = s.lock().unwrap();
        let mut new_super_types = HashMap::new();

        for (super_type_name, super_type) in super_types.iter() {
            let mut new_super_type = SuperType::default();

            super_type.types.iter().enumerate().for_each(|(i, key)| {
                if let Some(nested_super_type) = super_types.get(key) {
                    new_super_type.types.extend(nested_super_type.types.clone());
                } else {
                    new_super_type.types.push(key.clone());
                }
                new_super_type.variants.push(super_type.variants[i].clone())
            });
            new_super_types.insert(super_type_name.clone(), new_super_type);
        }

        new_super_types.into_iter().for_each(|(name, s)| {
            super_types.insert(name.clone(), s.clone());
        })
    });

    for node in &nodes {
        output.extend(node.to_token_stream());
    }

    for operators in (*OPERATORS_RULES.lock().unwrap()).values() {
        output.extend(generate_enum(
            &format_ident!("Operators_{}", operators.index),
            &operators.operators,
        ));
    }

    for (id, values) in &*INLINE_MULTIPLE_RULES.lock().unwrap() {
        output.extend(generate_enum(
            &format_ident!("{}", sanitize_string(id)),
            values,
        ));
    }

    for name in ANONYMOUS_TYPES.lock().unwrap().iter() {
        output.extend(generate_struct(
            &format_ident!("{}", &sanitize_string_to_pascal(name)),
            name,
            &vec![],
            &vec![],
            &vec![],
            &vec![],
        ));
    }

    let super_types_clone = SUPER_TYPES.with(|s| s.lock().unwrap().clone());
    for (super_type_name, super_type) in super_types_clone.iter() {
        output.extend(generate_enum(
            &format_ident!("{}", &sanitize_string_to_pascal(super_type_name)),
            &super_type.variants,
        ));
    }

    output
}
