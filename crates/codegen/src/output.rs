use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use crate::json::NodeType;
use crate::NODE_ID_FOR_NAME;
use crate::utils::sanitize_string_to_pascal;

impl ToTokens for NodeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            if self.is_struct() {
                self.create_struct().to_token_stream()
            } else if self.is_enum() {
                self.create_enum().to_token_stream()
            } else if self.is_token() {
                generate_struct(
                    &format_ident!("Token_{}", &sanitize_string_to_pascal(&self.kind)),
                    &self.kind,
                    &vec![],
                    &vec![],
                    &vec![])
            } else if !self.is_supertype() {
                generate_struct(
                    &format_ident!("{}", &sanitize_string_to_pascal(&self.kind)),
                    &self.kind,
                    &vec![],
                    &vec![],
                    &vec![])
            } else {
                TokenStream::new()
            }
        );
    }
}

impl NodeType {
    fn create_struct(&self) -> impl ToTokens {
        let mut _fields = vec![];

        if let Some(fields) = self.fields.as_ref() {
            fields.iter().for_each(|(name, info)| {
                _fields.push(info.field_code_gen(name));
            });
        }

        if let Some(children) = self.children.as_ref() {
            _fields.push(children.child_code_gen());
        }

        let (struct_fields, struct_fields_collect, struct_fields_finalize) = _fields
            .iter()
            .map(|field| {
                (
                    field.generate_field(),
                    field.generate_field_collect(),
                    field.generate_field_finalize(),
                )
            })
            .fold(
                (vec![], vec![], vec![]),
                |(mut fields, mut collects, mut finalizes), (field, collect, finalize)| {
                    fields.push(field);
                    collects.push(collect);
                    finalizes.push(finalize);
                    (fields, collects, finalizes)
                },
            );


        generate_struct(
            &format_ident!("{}", sanitize_string_to_pascal(&self.kind)),
            &self.kind,
            &struct_fields,
            &struct_fields_collect,
            &struct_fields_finalize,
        )
    }

    fn create_enum(&self) -> impl ToTokens {
        // Get enum variants
        let variants = self
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
        let types = self
            .subtypes
            .as_ref()
            .map(|subtypes| {
                subtypes
                    .iter()
                    .map(|subtype| subtype.kind.clone())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        generate_enum(&format_ident!("{}", sanitize_string_to_pascal(&self.kind)), &variants, &types)
    }
}

pub(crate) fn generate_struct(
    struct_name: &Ident,
    struct_type: &String,
    struct_fields: &Vec<TokenStream>,
    struct_fields_collect: &Vec<TokenStream>,
    struct_fields_finalize: &Vec<TokenStream>
)
    -> TokenStream {
    let cursor = if struct_fields_collect.is_empty() {
        quote! { }
    } else {
        quote! { let mut cursor = node.walk(); }
    };

    let of_type = match NODE_ID_FOR_NAME.lock().unwrap().get(struct_type) {
        Some(id) => {
            quote ! {
                impl #struct_name {
                    pub fn contains(node: &tree_sitter::Node) -> bool {
                        matches!(node.kind_id(), #id)
                    }
                }
            }
        }
        None => {
            quote ! {
                impl #struct_name {
                    pub fn contains(node: &tree_sitter::Node) -> bool {
                        matches!(node.kind(), #struct_type)
                    }
                }
            }
        }
    };

    let struct_fields = if struct_fields.is_empty() {
        quote! {  _range: auto_lsp::tree_sitter::Range }
    } else {
        quote! {
            #(#struct_fields),*,
             _range: auto_lsp::tree_sitter::Range
        }
    };

    let struct_fields_collect = if struct_fields_collect.is_empty() {
        quote! { }
    } else {
        quote! { #(#struct_fields_collect);*; }
    };

    let struct_fields_finalize = if struct_fields_finalize.is_empty() {
        quote! { Ok(Self { _range: node.range() }) }
    } else {
        quote! {
           Ok(Self {
                #(#struct_fields_finalize),*,
                 _range: node.range(),
            })
        }
    };

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #struct_name {
            #struct_fields
        }

        impl auto_lsp::core::ast::AstNode for #struct_name {
            fn range(&self) -> &auto_lsp::tree_sitter::Range {
                &self._range
            }
        }

        #of_type

        impl
            TryFrom<(
                &auto_lsp::tree_sitter::Node<'_>,
                &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstNode>>
            )> for #struct_name {
            type Error = auto_lsp::core::errors::AstError;

            fn try_from((node, index): (
                    &auto_lsp::tree_sitter::Node<'_>,
                    &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstNode>>)
            ) -> Result<Self, Self::Error> {
                #cursor
                #struct_fields_collect
                #struct_fields_finalize
            }
        }
    }
}

pub(crate) fn generate_enum(variant_name: &Ident, variants: &Vec<TokenStream>, types: &Vec<String>) -> TokenStream {
    let types = types.iter()
        .map(|n| {
            *NODE_ID_FOR_NAME.lock().unwrap().get(n).unwrap()
        }).collect::<Vec<_>>();

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum #variant_name {
            #(#variants(#variants)),*
        }

        impl auto_lsp::core::ast::AstNode for #variant_name {
            fn range(&self) -> &auto_lsp::tree_sitter::Range {
                match self {
                    #(Self::#variants(node) => node.range()),*
                }
            }
        }

        impl #variant_name {
            pub fn contains(node: &tree_sitter::Node) -> bool {
                matches!(node.kind_id(), #(#types)|*)
            }
        }

        impl
            TryFrom<(
                &auto_lsp::tree_sitter::Node<'_>,
                &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstNode>>
            )> for #variant_name {
            type Error = auto_lsp::core::errors::AstError;

            fn try_from(
                (node, index): (
                    &auto_lsp::tree_sitter::Node<'_>,
                    &mut Vec<std::sync::Arc<dyn auto_lsp::core::ast::AstNode>>)
            ) -> Result<Self, Self::Error> {
                match node.kind_id() {
                    #(#types => Ok(Self::#variants(#variants::try_from((node, &mut *index))?))),*,
                    _ => Err(auto_lsp::core::errors::AstError::UnexpectedSymbol {
                        range: node.range(),
                        symbol: node.kind(),
                        parent_name: stringify!(#variant_name),
                    })
                }
            }
        }
    }.to_token_stream()
}