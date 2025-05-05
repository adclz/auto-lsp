use crate::ir::{Child, Field, FieldOrChildren, Kind};
use crate::utils::sanitize_string_to_pascal;
use crate::{OperatorList, ANONYMOUS_TYPES, INLINE_MULTIPLE_RULES, NAMED_RULES, OPERATORS_RULES};
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NodeType {
    #[serde(rename = "type")]
    pub(crate) kind: String,
    pub(crate) named: bool,
    pub(crate) fields: Option<HashMap<String, FieldInfo>>,
    #[serde(default)]
    pub(crate) children: Option<ChildInfo>,
    #[serde(default)]
    pub(crate) subtypes: Option<Vec<NodeType>>,
}

impl NodeType {
    pub(crate) fn is_struct(&self) -> bool {
        self.named && self.fields.is_some()
    }

    pub(crate) fn is_enum(&self) -> bool {
        self.named && self.subtypes.is_some() && !self.is_supertype()
    }

    pub(crate) fn is_token(&self) -> bool {
        (!self.named)
            && (self.fields.is_none() || self.fields.as_ref().is_some_and(|f| f.is_empty()))
            && self.subtypes.is_none()
            && self.children.is_none()
    }

    pub(crate) fn is_supertype(&self) -> bool {
        self.named && self.subtypes.is_some() && self.fields.is_none() && self.children.is_none()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FieldInfo {
    multiple: Option<bool>,
    required: Option<bool>,
    types: Vec<TypeInfo>,
}

impl FieldInfo {
    fn field_gen_type(&self) -> Kind {
        let optional = self.required.unwrap_or(false);
        let multiple = self.multiple.unwrap_or(false);
        match (optional, multiple) {
            (true, true) => Kind::Vec,
            (false, true) => Kind::Vec,
            (true, false) => Kind::Base,
            (false, false) => Kind::Option,
        }
    }

    pub(crate) fn field_code_gen(&self, field_name: &str) -> FieldOrChildren {
        // If there's only one type, we can use it directly
        let base_type = if self.types.len() == 1 {
            if !NAMED_RULES
                .lock()
                .unwrap()
                .contains(&sanitize_string_to_pascal(&self.types[0].kind))
            {
                ANONYMOUS_TYPES
                    .lock()
                    .unwrap()
                    .insert(self.types[0].kind.clone());
            }
            format_ident!("{}", sanitize_string_to_pascal(&self.types[0].kind))

        // If all types are unnamed, we generate an operator list
        } else if self.types.iter().all(|t| !t.named) {
            let mut lock = OPERATORS_RULES.lock().unwrap();

            let rule: String = self.types.iter().map(|n| n.kind.clone()).collect();

            let len = lock.len();
            let op = lock.entry(rule.clone()).or_insert(OperatorList {
                index: len,
                operators: self.types.iter().map(|n| n.kind.clone()).collect(),
            });

            format_ident!("Operators_{}", op.index)
        // Types are mixed, so we generate an enum
        } else {
            let list: String = self
                .types
                .iter()
                .map(|t| sanitize_string_to_pascal(&t.kind).to_string())
                .collect::<Vec<_>>()
                .join("_");

            let variants = self.types.to_vec();

            INLINE_MULTIPLE_RULES
                .lock()
                .unwrap()
                .entry(list.clone())
                .or_insert(variants.clone());

            format_ident!("{}", list)
        };

        FieldOrChildren::Field(Field {
            kind: self.field_gen_type(),
            tree_sitter_type: field_name.to_string(),
            field_name: quote! { #base_type },
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChildInfo {
    multiple: Option<bool>,
    required: Option<bool>,
    types: Vec<TypeInfo>,
}

impl ChildInfo {
    fn child_gen_type(&self) -> Kind {
        let optional = self.required.unwrap_or(false);
        let multiple = self.multiple.unwrap_or(false);
        match (optional, multiple) {
            (true, true) => Kind::Vec,
            (false, true) => Kind::Vec,
            (true, false) => Kind::Option,
            (false, false) => Kind::Base,
        }
    }

    pub(crate) fn child_code_gen(&self) -> FieldOrChildren {
        // If there's only one type, we can use it directly
        let base_type = if self.types.len() == 1 {
            if !NAMED_RULES
                .lock()
                .unwrap()
                .contains(&sanitize_string_to_pascal(&self.types[0].kind))
            {
                ANONYMOUS_TYPES
                    .lock()
                    .unwrap()
                    .insert(self.types[0].kind.clone());
            }
            format_ident!("{}", sanitize_string_to_pascal(&self.types[0].kind))

        // If all types are unnamed, we generate an operator list
        } else if self.types.iter().all(|t| !t.named) {
            let mut lock = OPERATORS_RULES.lock().unwrap();

            let rule: String = self.types.iter().map(|n| n.kind.clone()).collect();

            let len = lock.len();
            let op = lock.entry(rule.clone()).or_insert(OperatorList {
                index: len,
                operators: self.types.iter().map(|n| n.kind.clone()).collect(),
            });

            format_ident!("Operators_{}", op.index)
        // Types are mixed, so we generate an enum
        } else {
            let list: String = self
                .types
                .iter()
                .map(|t| sanitize_string_to_pascal(&t.kind).to_string())
                .collect::<Vec<_>>()
                .join("_");

            let variants = self.types.to_vec();

            INLINE_MULTIPLE_RULES
                .lock()
                .unwrap()
                .entry(list.clone())
                .or_insert(variants.clone());

            format_ident!("{}", list)
        };

        FieldOrChildren::Child(Child {
            kind: self.child_gen_type(),
            field_name: quote! { #base_type },
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TypeInfo {
    #[serde(rename = "type")]
    pub(crate) kind: String,
    pub(crate) named: bool,
}
