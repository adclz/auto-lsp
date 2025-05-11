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

use crate::json::{NodeType, TypeInfo};
use crate::supertypes::SUPER_TYPES;
use crate::utils::sanitize_string_to_pascal;
use crate::{NODE_ID_FOR_NAMED_NODE, NODE_ID_FOR_UNNAMED_NODE};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

impl ToTokens for NodeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(if self.is_struct() {
            self.create_struct().to_token_stream()
        } else if self.is_enum() {
            self.create_enum().to_token_stream()
        } else if self.is_token() {
            generate_struct(
                &format_ident!("Token_{}", &sanitize_string_to_pascal(&self.kind)),
                &self.kind,
                &vec![],
                &vec![],
                &vec![],
                &vec![],
            )
        } else if !self.is_supertype() {
            generate_struct(
                &format_ident!("{}", &sanitize_string_to_pascal(&self.kind)),
                &self.kind,
                &vec![],
                &vec![],
                &vec![],
                &vec![],
            )
        } else {
            TokenStream::new()
        });
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

        let (struct_fields, struct_fields_init, struct_fields_collect, struct_fields_finalize) =
            _fields
                .iter()
                .map(|field| {
                    (
                        field.generate_field(),
                        field.generate_field_init(),
                        field.generate_field_collect(),
                        field.generate_field_finalize(),
                    )
                })
                .fold(
                    (vec![], vec![], vec![], vec![]),
                    |(mut fields, mut inits, mut collects, mut finalizes),
                     (field, init, collect, finalize)| {
                        fields.push(field);
                        inits.push(init);
                        collects.push(collect);
                        finalizes.push(finalize);
                        (fields, inits, collects, finalizes)
                    },
                );

        generate_struct(
            &format_ident!("{}", sanitize_string_to_pascal(&self.kind)),
            &self.kind,
            &struct_fields,
            &struct_fields_init,
            &struct_fields_collect,
            &struct_fields_finalize,
        )
    }

    fn create_enum(&self) -> impl ToTokens {
        generate_enum(
            &format_ident!("{}", sanitize_string_to_pascal(&self.kind)),
            self.subtypes.as_ref().unwrap(),
        )
    }
}

pub(crate) fn generate_struct(
    struct_name: &Ident,
    struct_type: &String,
    struct_fields: &Vec<TokenStream>,
    struct_fields_init: &Vec<TokenStream>,
    struct_fields_collect: &Vec<TokenStream>,
    struct_fields_finalize: &Vec<TokenStream>,
) -> TokenStream {
    let of_type = match NODE_ID_FOR_NAMED_NODE.lock().unwrap().get(struct_type) {
        Some(id) => {
            quote! {
                fn contains(node: &auto_lsp::tree_sitter::Node) -> bool {
                    matches!(node.kind_id(), #id)
                }
            }
        }
        None => {
            quote! {
                fn contains(node: &auto_lsp::tree_sitter::Node) -> bool {
                    matches!(node.kind(), #struct_type)
                }
            }
        }
    };

    let struct_fields = if struct_fields.is_empty() {
        quote! { _range: auto_lsp::tree_sitter::Range, _id: usize, _parent: Option<usize>, }
    } else {
        quote! {
            #(#struct_fields),*,
             _range: auto_lsp::tree_sitter::Range,
            _id: usize,
            _parent: Option<usize>
        }
    };

    let struct_fields_finalize = if struct_fields_finalize.is_empty() {
        quote! { Ok(Self { _range: node.range(), _id: id, _parent: parent_id }) }
    } else {
        quote! {
           Ok(Self {
                #(#struct_fields_finalize),*,
                 _range: node.range(),
                _id: id,
                _parent: parent_id
            })
        }
    };

    let init_builder = if struct_fields_collect.is_empty() {
        quote! {}
    } else {
        quote! {
          builder
            .builder(db, &node, Some(id), |b| {
                b #(.#struct_fields_collect)?*
            });
        }
    };

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #struct_name {
            #struct_fields
        }

        impl auto_lsp::core::ast::AstNode for #struct_name {
            #of_type

            fn lower(&self) -> &dyn auto_lsp::core::ast::AstNode {
                self
            }

            fn get_id(&self) -> usize {
                self._id
            }

            fn get_parent_id(&self) -> Option<usize> {
                self._parent
            }

            fn get_range(&self) -> &auto_lsp::tree_sitter::Range {
                &self._range
            }
        }

        impl<'a>
            TryFrom<auto_lsp::core::ast::TryFromParams<'a>> for #struct_name {
            type Error = auto_lsp::core::errors::AstError;

            fn try_from((node, db, builder, id, parent_id): auto_lsp::core::ast::TryFromParams) -> Result<Self, Self::Error> {
                #(#struct_fields_init);*;
                #init_builder
                #struct_fields_finalize
            }
        }
    }
}

pub(crate) fn generate_enum(variant_name: &Ident, variants: &Vec<TypeInfo>) -> TokenStream {
    let mut r_variants = vec![];
    let mut r_types = vec![];

    let mut super_types_variants: Vec<_> = vec![];
    let mut super_types_types: Vec<Vec<_>> = vec![];

    for value in variants {
        let variant_name = format_ident!("{}", &sanitize_string_to_pascal(&value.kind));
        if !value.named {
            r_variants.push(format_ident!("Token_{}", variant_name.clone()).to_token_stream());

            let type_name = if value.named {
                *NODE_ID_FOR_NAMED_NODE
                    .lock()
                    .unwrap()
                    .get(&value.kind)
                    .unwrap()
            } else {
                *NODE_ID_FOR_UNNAMED_NODE
                    .lock()
                    .unwrap()
                    .get(&value.kind)
                    .unwrap()
            };

            r_types.push(type_name);
        } else if SUPER_TYPES.with(|s| s.lock().unwrap().contains_key(&value.kind)) {
            let supertype =
                SUPER_TYPES.with(|s| s.lock().unwrap().get(&value.kind).unwrap().clone());
            super_types_variants.push(variant_name.to_token_stream());
            super_types_types.push(
                supertype
                    .types
                    .iter()
                    .map(|t| {
                        *NODE_ID_FOR_NAMED_NODE
                            .lock()
                            .unwrap()
                            .get::<String>(&t.clone())
                            .unwrap()
                    })
                    .collect(),
            );
        } else {
            r_variants.push(variant_name.to_token_stream());
            r_types.push(
                *NODE_ID_FOR_NAMED_NODE
                    .lock()
                    .unwrap()
                    .get(&value.kind)
                    .unwrap(),
            );
        }
    }

    let pattern_matching = match (r_types.is_empty(), super_types_types.is_empty()) {
        (false, false) => {
            let k = quote! {
                #(#r_types => Ok(Self::#r_variants(#r_variants::try_from((node, db, builder, id, parent_id))?))),*,
                /// Super types
                #(#(#super_types_types)|* => Ok(Self::#super_types_variants(#super_types_variants::try_from((node, db, builder, id, parent_id))?))),*,
                _ => Err(auto_lsp::core::errors::AstError::UnexpectedSymbol {
                    range: node.range(),
                    symbol: node.kind(),
                    parent_name: stringify!(#variant_name),
                })
            };
            k
        }
        (true, false) => {
            let k = quote! {
                /// Super types
                #(#(#super_types_types)|* => Ok(Self::#super_types_variants(#super_types_variants::try_from((node, db, builder, id, parent_id))?))),*,
                _ => Err(auto_lsp::core::errors::AstError::UnexpectedSymbol {
                    range: node.range(),
                    symbol: node.kind(),
                    parent_name: stringify!(#variant_name),
                })
            };
            k
        }
        (false, true) => quote! {
            #(#r_types => Ok(Self::#r_variants(#r_variants::try_from((node, db, builder, id, parent_id))?))),*,
            _ => Err(auto_lsp::core::errors::AstError::UnexpectedSymbol {
                range: node.range(),
                symbol: node.kind(),
                parent_name: stringify!(#variant_name),
            })
        },
        _ => quote! {
            _ => Err(auto_lsp::core::errors::AstError::UnexpectedSymbol {
                range: node.range(),
                symbol: node.kind(),
                parent_name: stringify!(#variant_name),
            })
        },
    };

    r_variants.extend(super_types_variants);
    r_types.extend(super_types_types.into_iter().flatten());

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum #variant_name {
            #(#r_variants(#r_variants)),*
        }

        impl auto_lsp::core::ast::AstNode for #variant_name {
            fn contains(node: &auto_lsp::tree_sitter::Node) -> bool {
                matches!(node.kind_id(), #(#r_types)|*)
            }

            fn lower(&self) -> &dyn auto_lsp::core::ast::AstNode {
                match self {
                    #(Self::#r_variants(node) => node.lower()),*
                }
            }

            fn get_id(&self) -> usize {
                match self {
                    #(Self::#r_variants(node) => node.get_id()),*
                }
            }

            fn get_parent_id(&self) -> Option<usize> {
                match self {
                    #(Self::#r_variants(node) => node.get_parent_id()),*
                }
            }

            fn get_range(&self) -> &auto_lsp::tree_sitter::Range {
                match self {
                    #(Self::#r_variants(node) => node.get_range()),*
                }
            }
        }

       impl<'a>
            TryFrom<auto_lsp::core::ast::TryFromParams<'a>> for #variant_name {
            type Error = auto_lsp::core::errors::AstError;

            fn try_from((node, db, builder, id, parent_id): auto_lsp::core::ast::TryFromParams) -> Result<Self, Self::Error> {
                match node.kind_id() {
                    #pattern_matching
                }
            }
        }
    }
    .to_token_stream()
}
