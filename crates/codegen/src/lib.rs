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

//! # Auto LSP Codegen
//!
//! To generate an AST, simply provide a Tree-sitter [node-types.json](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#static-node-types) and [LanguageFn](https://docs.rs/tree-sitter/latest/tree_sitter/struct.Language.html) of any language to the `generate` function of the `auto_lsp_codegen` crate.
//!
//! ```sh
//! cargo add auto_lsp_codegen
//! ```
//!
//! Although `auto_lsp_codegen` is a standalone crate, the generated code depends on the main `auto_lsp` crate.
//!
//! ## Usage
//!
//! The `auto_lsp_codegen` crate exposes a single `generate` function, which takes:
//!
//! - A [`node-types.json`](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html),
//! - A [`LanguageFn`](https://docs.rs/tree-sitter-language/0.1.5/tree_sitter_language/struct.LanguageFn.html)
//! - A `HashMap<&str, &str>` to rename tokens (see [Custom Tokens](#custom-tokens))
//! - And returns a **TokenStream**.
//!
//! How you choose to use the `TokenStream` is up to you.
//!
//! The most common setup is to call it from a **build.rs** script and write the generated code to a Rust file.
//!
//! Note, however, that the output can be quite large—for example, Python’s AST results in ~11,000 lines of code.
//!
//! ```rust, ignore
//! use auto_lsp_codegen::generate;
//! use std::{fs, path::PathBuf};
//!
//! fn main() {
//!    if std::env::var("AST_GEN").unwrap_or("0".to_string()) == "0" {
//!        return;
//!    }
//!
//!    let output_path = PathBuf::from("./src/generated.rs");
//!
//!    fs::write(
//!        output_path,
//!        generate(
//!            tree_sitter_python::NODE_TYPES,
//!            &tree_sitter_python::LANGUAGE.into(),
//!            None,
//!        )
//!        .to_string(),
//!    )
//!    .unwrap();
//!}
//! ```
//!
//! You can also invoke it from your own CLI or tool if needed.
//!
//! ## How Codegen Works
//!
//! The generated code structure depends on the Tree-sitter grammar.
//!
//! ## Structs for Rules
//!
//! Each rule in `node-types.json` becomes a dedicated Rust struct. For example, given the rule:
//!
//! ```js
//! function_definition: $ => seq(
//!      optional('async'),
//!      'def',
//!      field('name', $.identifier),
//!      field('type_parameters', optional($.type_parameter)),
//!      field('parameters', $.parameters),
//!      optional(
//!        seq(
//!          '->',
//!          field('return_type', $.type),
//!        ),
//!      ),
//!      ':',
//!      field('body', $._suite),
//!    ),
//! ```
//!
//! The generated struct would look like this:
//!
//! ```rust, ignore
//!#[derive(Debug, Clone, PartialEq)]
//!pub struct FunctionDefinition {
//!    pub name: std::sync::Arc<Identifier>,
//!    pub body: std::sync::Arc<Block>,
//!    pub type_parameters: Option<std::sync::Arc<TypeParameter>>,
//!    pub parameters: std::sync::Arc<Parameters>,
//!    pub return_type: Option<std::sync::Arc<Type>>,
//!    /* ... */
//!}
//! ```
//!
//! ## Field Matching
//!
//! To match fields, codegen uses the `field_id()` method from the Tree-sitter cursor.
//!
//! From the above example, the generated builder might look like this:
//!
//! ```rust, ignore
//!builder.builder(db, &node, Some(id), |b| {
//!  b.on_field_id::<Identifier, 19u16>(&mut name)?
//!    .on_field_id::<Block, 6u16>(&mut body)?
//!    .on_field_id::<TypeParameter, 31u16>(&mut type_parameters)?
//!    .on_field_id::<Parameters, 23u16>(&mut parameters)?
//!    .on_field_id::<Type, 24u16>(&mut return_type)
//!});
//! ```
//!
//! Each **u16** represents the unique field ID assigned by the Tree-sitter language parser.
//!
//! ## Handling Children
//!
//! If a node has no named fields, a children enum is generated to represent all possible variants.
//!
//! - If the children are **unnamed**, a generic "Operator_" enum is generated
//! - If the children are **named**, the enum will be a concatenation of all possible child node types with underscores, using sanitized Rust-friendly names.
//!
//! For example, given the rule:
//!
//! ```js
//!  _statement: $ => choice(
//!      $._simple_statement,
//!      $._compound_statement,
//!    ),
//! ```
//!
//! The generated enum would look like this:
//!
//! ```rust, ignore
//! pub enum SimpleStatement_CompoundStatement {
//!    SimpleStatement(SimpleStatement),
//!    CompoundStatement(CompoundStatement),
//! }
//! ```
//!
//! If the generated enum name becomes too long, consider using a Tree-sitter
//! <a href="https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#supertype-nodes">supertype</a> to group nodes together.
//!
//! The `kind_id()` method is used to determine child kinds during traversal.
//!
//! The `AstNode::contains` method relies on this to check whether a node kind belongs to a specific struct or enum variant.
//!
//! ## Vec and Option Fields
//!
//! `repeat` and `repeat1` in the grammar will generate a `Vec` field.
//!
//! `optional(...)` will generate an `Option<T>` field.
//!
//! ## Token Naming
//!
//! Unnamed tokens are mapped to Rust enums using a built-in token map. For instance:
//!
//! ```json
//!  { "type": "+", "named": false },
//!  { "type": "+=", "named": false },
//!  { "type": ",", "named": false },
//!  { "type": "-", "named": false },
//!  { "type": "-=", "named": false },
//! ```
//!
//! Generates:
//!
//! ```rust, ignore
//! pub enum Token_Plus {}
//! pub enum Token_PlusEqual {}
//! pub enum Token_Comma {}
//! pub enum Token_Minus {}
//! pub enum Token_MinusEqual {}
//! ```
//!
//! Tokens with regular identifiers are converted to PascalCase.
//!
//! ## Custom Tokens
//!
//! If your grammar defines additional unnamed tokens not covered by the default map, you can provide a custom token mapping to generate appropriate Rust enum names.
//!
//! ```rust, ignore
//!use auto_lsp_codegen::generate;
//!
//!let _result = generate(
//!        &tree_sitter_python::NODE_TYPES,
//!        &tree_sitter_python::LANGUAGE.into(),
//!        Some(HashMap::from([
//!            ("+", "Plus"),
//!            ("+=", "PlusEqual"),
//!            (",", "Comma"),
//!            ("-", "Minus"),
//!            ("-=", "MinusEqual"),
//!        ])),
//!    );
//! ```
//!
//! Tokens that are not in the map will be added, and tokens that already exist in the map will be overwritten.
//!
//! ## Super Types
//!
//! Tree-sitter supports [supertypes](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#supertype-nodes), which allow grouping related nodes under a common type.
//!
//! For example, in the Python grammar:
//!
//! ```json
//!  {
//!    "type": "_compound_statement",
//!    "named": true,
//!    "subtypes": [
//!      {
//!        "type": "class_definition",
//!        "named": true
//!      },
//!      {
//!        "type": "decorated_definition",
//!        "named": true
//!      },
//!      /* ... */
//!      {
//!        "type": "with_statement",
//!        "named": true
//!      }
//!    ]
//!  },
//! ```
//!
//! This becomes a Rust enum:
//!
//! ```rust, ignore
//! pub enum CompoundStatement {
//!    ClassDefinition(ClassDefinition),
//!    DecoratedDefinition(DecoratedDefinition),
//!    /* ... */
//!    WithStatement(WithStatement),
//! }
//! ```
//!
//! Some super types might contain other super types, in which case, the generated enum will flatten the hierarchy.

mod ir;
mod json;
mod output;
mod supertypes;
mod tests;
mod utils;

use crate::json::{NodeType, TypeInfo};
use crate::output::{generate_enum, generate_struct};
use crate::supertypes::{generate_super_type, SuperType};
use crate::utils::{sanitize_string, sanitize_string_to_pascal};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex, RwLock};
use utils::TOKENS;

/// List of all named rules (nodes with `named: true`)
pub(crate) static NAMED_RULES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(Default::default);

/// List of fields/children that are only composed of operators
pub(crate) struct OperatorList {
    index: usize,
    operators: Vec<TypeInfo>,
}

pub(crate) static OPERATORS_RULES: LazyLock<Mutex<HashMap<String, OperatorList>>> =
    LazyLock::new(Default::default);

/// List of fields/children that are composed of multiple rules
pub(crate) static INLINE_MULTIPLE_RULES: LazyLock<Mutex<HashMap<String, Vec<TypeInfo>>>> =
    LazyLock::new(Default::default);

/// List of anonymous rules (usually aliases created on the fly)
pub(crate) static ANONYMOUS_TYPES: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(Default::default);

/// Map of node kind to  named node id
pub(crate) static NODE_ID_FOR_NAMED_NODE: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

/// Map of node kind to unnamed node id
pub(crate) static NODE_ID_FOR_UNNAMED_NODE: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

/// Map of field name to field id
pub(crate) static FIELD_ID_FOR_NAME: LazyLock<Mutex<HashMap<String, u16>>> =
    LazyLock::new(Default::default);

/// List of super types
pub(crate) static SUPER_TYPES: LazyLock<RwLock<HashMap<String, SuperType>>> =
    LazyLock::new(Default::default);

/// Generates the Rust code for a given Tree-sitter grammar
///
/// # Arguments
///
/// * `source` - node-types.json
/// * `language` - tree-sitter language fn
/// * `tokens` - optional map of tokens to enum names (since tokens can't be valid rust identifiers)
///
/// # Returns
/// A TokenStream containing the generated code
///
/// # Example
///
/// ```rust
/// use auto_lsp_codegen::generate;
///
/// let _result = generate(
///        &tree_sitter_python::NODE_TYPES,
///        &tree_sitter_python::LANGUAGE.into(),
///        None,
///    );
/// ```
///
pub fn generate(
    source: &str,
    language: &tree_sitter::Language,
    tokens: Option<HashMap<&'static str, &'static str>>,
) -> TokenStream {
    if let Some(tokens) = tokens {
        // extend or overwrite the default tokens

        let mut lock = TOKENS.write().unwrap();
        for (k, v) in tokens {
            lock.insert(k, v);
        }
    }

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
            // Push the node kind to the list of named rules
            NAMED_RULES
                .lock()
                .unwrap()
                .push(sanitize_string_to_pascal(&node.kind));
            // Push the node kind to the list of ids for named nodes
            NODE_ID_FOR_NAMED_NODE.lock().unwrap().insert(
                node.kind.clone(),
                language.id_for_node_kind(&node.kind, true),
            );
            // If the node has fields, we need to add them to the list of fields
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
            // Push the node kind to the list of ids for named nodes
            NODE_ID_FOR_UNNAMED_NODE.lock().unwrap().insert(
                node.kind.clone(),
                language.id_for_node_kind(&node.kind, false),
            );
        }
        // If node is a supertype, add it to the list of super types
        if node.is_supertype() {
            SUPER_TYPES
                .write()
                .unwrap()
                .insert(node.kind.clone(), generate_super_type(node));
        }
    }

    // Super types may contains other super types
    // in this case we need to add the nested super types to the `types` field of the current super type
    let mut super_types_lock = SUPER_TYPES.write().unwrap();
    let mut new_super_types = HashMap::new();

    for (super_type_name, super_type) in super_types_lock.iter() {
        let mut new_super_type = SuperType::default();

        // Iterate over the types of this super type
        super_type.types.iter().enumerate().for_each(|(i, key)| {
            if let Some(nested_super_type) = super_types_lock.get(key) {
                // Some types are super types
                new_super_type.types.extend(nested_super_type.types.clone());
            } else {
                // Otherwise, we just clone the type
                new_super_type.types.push(key.clone());
            }
            new_super_type.variants.push(super_type.variants[i].clone())
        });
        new_super_types.insert(super_type_name.clone(), new_super_type);
    }

    // Now we need to merge the new super types with the existing ones
    new_super_types.into_iter().for_each(|(name, s)| {
        super_types_lock.insert(name.clone(), s.clone());
    });

    drop(super_types_lock);

    // Generate the structs and enums for all rules
    for node in &nodes {
        output.extend(node.to_token_stream());
    }

    // Generate the list of operators
    for operators in (*OPERATORS_RULES.lock().unwrap()).values() {
        output.extend(generate_enum(
            &format_ident!("Operators_{}", operators.index),
            &operators.operators,
        ));
    }

    // Generate the list of inline multiple rules
    for (id, values) in &*INLINE_MULTIPLE_RULES.lock().unwrap() {
        output.extend(generate_enum(
            &format_ident!("{}", sanitize_string(id)),
            values,
        ));
    }

    // Generate the list of anonymous types
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

    // Generate the list of super types
    // We need to clone because generate_enum will also check if some variants are super types
    for (super_type_name, super_type) in SUPER_TYPES.read().unwrap().iter() {
        output.extend(generate_enum(
            &format_ident!("{}", &sanitize_string_to_pascal(super_type_name)),
            &super_type.variants,
        ));
    }

    output
}
