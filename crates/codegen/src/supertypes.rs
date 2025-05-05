use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::NodeType;
use crate::utils::sanitize_string_to_pascal;

#[derive(Clone)]
pub(crate) struct SuperType {
    pub(crate) variants: Vec<TokenStream>,
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
            subtypes
                .iter()
                .map(|subtype| {
                    let subtype_name =
                        format_ident!("{}", sanitize_string_to_pascal(&subtype.kind));
                    quote! {
                        #subtype_name
                    }
                })
                .collect::<Vec<TokenStream>>()
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