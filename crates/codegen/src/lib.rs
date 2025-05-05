mod ir;
mod json;
mod output;
mod supertypes;
mod tests;
mod utils;

use crate::json::{NodeType, TypeInfo};
use crate::output::{generate_enum, generate_struct};
use crate::supertypes::{generate_super_type, SUPER_TYPES};
use crate::utils::{sanitize_string, sanitize_string_to_pascal};
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

/// List of all named rules
pub(crate) static NAMED_RULES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(Default::default);

pub(crate) struct OperatorList {
    index: usize,
    operators: Vec<String>,
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
pub(crate) static NODE_ID_FOR_NAME: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

/// Map of field name to field id
pub(crate) static FIELD_ID_FOR_NAME: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

pub fn generate(source: &str, language: &tree_sitter::Language) -> TokenStream {
    let nodes: Vec<NodeType> = serde_json::from_str(&source).expect("Invalid JSON");

    let mut output = TokenStream::new();
    for node in &nodes {
        if node.named {
            NAMED_RULES
                .lock()
                .unwrap()
                .push(sanitize_string_to_pascal(&node.kind));
            NODE_ID_FOR_NAME.lock().unwrap().insert(
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
            NODE_ID_FOR_NAME.lock().unwrap().insert(
                node.kind.clone(),
                language.id_for_node_kind(&node.kind, false),
            );
        }
        if node.is_supertype() {
            SUPER_TYPES.with(|s| {
                s.lock()
                    .unwrap()
                    .insert(node.kind.clone(), generate_super_type(&node));
            });
        }
    }

    for node in &nodes {
        output.extend(node.to_token_stream());
    }

    for (_, operators) in &*OPERATORS_RULES.lock().unwrap() {
        let id = format_ident!("Operators_{}", operators.index);

        let sanitized_operators = operators
            .operators
            .iter()
            .map(|op| format_ident!("Token_{}", sanitize_string_to_pascal(op)).to_token_stream())
            .collect::<Vec<_>>();

        output.extend(generate_enum(
            &id,
            &sanitized_operators,
            &operators.operators,
        ));
    }

    for (id, values) in &*INLINE_MULTIPLE_RULES.lock().unwrap() {
        let id = format_ident!("{}", sanitize_string(&id));

        let mut variants = vec![];
        let mut types = vec![];

        for value in values {
            let variant_name = format_ident!("{}", &sanitize_string_to_pascal(&value.kind));
            if !value.named {
                variants.push(format_ident!("Token_{}", variant_name.clone()).to_token_stream());
            } else if SUPER_TYPES.with(|s| s.lock().unwrap().contains_key(&value.kind)) {
                let supertype =
                    SUPER_TYPES.with(|s| s.lock().unwrap().get(&value.kind).unwrap().clone());
                variants.extend(supertype.variants);
                types.extend(supertype.types);
                continue;
            } else {
                variants.push(variant_name.to_token_stream());
            }
            types.push(value.kind.clone());
        }

        output.extend(generate_enum(&id, &variants, &types));
    }

    for name in ANONYMOUS_TYPES.lock().unwrap().iter() {
        output.extend(generate_struct(
            &format_ident!("{}", &sanitize_string_to_pascal(&name)),
            &name,
            &vec![],
            &vec![],
            &vec![],
        ));
    }

    let super_types_clone = SUPER_TYPES.with(|s| s.lock().unwrap().clone());

    let output_s = SUPER_TYPES.with(|s| {
        let mut output = TokenStream::new();

        for (super_type_name, super_type) in s.lock().unwrap().iter() {
            let super_type_name = format_ident!("{}", &sanitize_string_to_pascal(&super_type_name));

            let mut variants = vec![];
            let mut types = vec![];

            super_type.types.iter().enumerate().for_each(|(i, t)| {
                if let Some(super_type) = super_types_clone.get(t) {
                    variants.extend(super_type.variants.clone());
                    types.extend(super_type.types.clone());
                } else {
                    variants.push(super_type.variants[i].clone());
                    types.push(super_type.types[i].clone());
                }
            });
            output.extend(generate_enum(&super_type_name, &variants, &types));
        }
        output
    });

    output.extend(output_s);
    output
}
