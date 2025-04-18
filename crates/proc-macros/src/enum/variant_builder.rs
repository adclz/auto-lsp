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

use crate::utilities::filter2::get_type_name;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Path};

/// A container for information about enum variants.
///
/// This struct stores:
/// - `variant_names`: The names of the variants in the enum.
/// - `variant_types_names`: The types of the fields in the variants.
/// - `variant_builder_names`: The builder names derived from the variant types.
///
/// # Example
///
/// ```ignore
/// enum MyEnum {
///     Variant1(u8),
///     Variant2(String),
/// }
///
/// // Extracted as:
/// Variants {
///     variant_names: vec![Variant1, Variant2],
///     variant_types_names: vec![u8, String],
///     variant_builder_names: vec![u8Builder, StringBuilder],
/// }
/// ```
#[derive(Debug)]
pub struct Variants {
    /// Names of the enum variants (e.g., `Variant1`).
    pub variant_names: Vec<proc_macro2::Ident>,

    /// Types of the fields in the variants (e.g., `u8`).
    pub variant_types_names: Vec<proc_macro2::Ident>,

    /// Builder names derived from the variant types (e.g., `u8Builder`).
    pub variant_builder_names: Vec<proc_macro2::Ident>,
}

/// Extracts variant information from a syn::Data enum definition.
///
/// See the `Variants` struct for more information.
pub fn extract_variants(data: &syn::DataEnum) -> (Variants, Option<syn::Error>) {
    let mut ret_fields = Variants {
        variant_names: vec![],
        variant_types_names: vec![],
        variant_builder_names: vec![],
    };

    let mut errors: Vec<syn::Error> = vec![];

    for variant in &data.variants {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if let Some(first_field) = fields.unnamed.first() {
                    match get_type_name(&first_field.ty) {
                        Ok(type_name) => {
                            ret_fields.variant_names.push(variant_name.clone());
                            ret_fields
                                .variant_types_names
                                .push(format_ident!("{}", type_name));
                            ret_fields
                                .variant_builder_names
                                .push(format_ident!("{}Builder", type_name));
                        }
                        Err(err) => errors.push(err),
                    }
                } else {
                    errors.push(syn::Error::new_spanned(
                        &variant,
                        "Unnamed variant must contain exactly one field",
                    ));
                }
            }
            _ => {
                errors.push(syn::Error::new_spanned(
                    &variant,
                    "This proc macro only supports tuple (unnamed) variants",
                ));
            }
        }
    }

    let combined_error = if errors.is_empty() {
        None
    } else {
        let mut iter = errors.into_iter();
        let mut combined = iter.next().unwrap();
        for err in iter {
            combined.combine(err);
        }
        Some(combined)
    };

    (ret_fields, combined_error)
}

/// Builder for enum variants
///
/// This builder stores unstaged TokenStreams that can be staged into a final TokenStream.
#[derive(Default)]
pub struct VariantBuilder {
    staged: Vec<TokenStream>,
    unstaged: Vec<TokenStream>,
}

impl VariantBuilder {
    /// Adds an **unstaged** TokenStream to the builder
    pub fn add(&mut self, token: TokenStream) -> &mut Self {
        self.unstaged.push(token);
        self
    }

    /// Adds an **unstaged** TokenStream to the builder.
    ///
    /// This function takes a closure that will receive all variants in `variants` and return a `TokenStream`.
    ///
    /// The closure takes 3 arguments:
    /// - 1: `&Ident`: The variant name
    /// - 2: `&Ident`: The variant type name
    /// - 3: `&Ident`: The variant builder name
    pub fn add_iter<F>(&mut self, variants: &Variants, body: F) -> &mut Self
    where
        F: Fn(&Ident, &Ident, &Ident) -> TokenStream,
    {
        let variants = variants
            .variant_names
            .iter()
            .zip(variants.variant_types_names.iter())
            .zip(variants.variant_builder_names.iter())
            .map(|((name, _type), builder)| body(name, _type, builder))
            .collect::<Vec<_>>();

        self.unstaged.extend(variants);
        self
    }

    /// Adds an **unstaged** pattern matching TokenStream to the builder.
    ///
    /// This function takes a closure that will receive all variants in `variants` and return a `TokenStream`.
    ///
    /// The closure takes 3 arguments:
    /// - 1: `&Ident`: The variant name
    /// - 2: `&Ident`: The variant type name
    /// - 3: `&Ident`: The variant builder name
    pub fn add_pattern_match_iter(
        &mut self,
        variants: &Variants,
        sig_path: &TokenStream,
        default: &TokenStream,
    ) -> &mut Self {
        let variants = variants
            .variant_names
            .iter()
            .map(|name| {
                quote! {
                    Self::#name(inner) => inner.#default,
                }
            })
            .collect::<Vec<_>>();

        self.unstaged.push(quote! {
            #sig_path {
                match self {
                    #(#variants)*
                }
            }
        });
        self
    }

    /// Adds an **unstaged** function to the builder.
    ///
    /// This function takes a closure that will receive all variants in `variants` and return a `TokenStream`.
    ///
    /// The closure takes 3 arguments:
    /// - 1: `&Ident`: The variant name
    /// - 2: `&Ident`: The variant type name
    /// - 3: `&Ident`: The variant builder name
    pub fn add_fn_iter<F>(
        &mut self,
        variants: &Variants,
        sig_path: &TokenStream,
        before: Option<TokenStream>,
        body: F,
        after: Option<TokenStream>,
    ) -> &mut Self
    where
        F: Fn(&Ident, &Ident, &Ident) -> TokenStream,
    {
        let variants = variants
            .variant_names
            .iter()
            .zip(variants.variant_types_names.iter())
            .zip(variants.variant_builder_names.iter())
            .map(|((name, _type), builder)| {
                let body = body(name, _type, builder);
                quote! {
                    Self::#name(inner) => inner.#body,
                }
            })
            .collect::<Vec<_>>();

        let mut result = TokenStream::default();
        if let Some(before) = before {
            result.extend(before);
        }

        result.extend(variants);

        if let Some(after) = after {
            result.extend(after);
        }

        self.unstaged.push(quote! {
            #sig_path {
                match self {
                    #result
                }
            }
        });
        self
    }

    fn drain(&mut self) -> Vec<TokenStream> {
        std::mem::take(&mut self.unstaged)
    }

    /// Drains the **unstaged** TokenStream and pushes it to the **staged** TokenStream.
    ///
    /// Usually, you would call this function after you are done pushing unstaged Tokens.
    pub fn stage(&mut self) -> &mut Self {
        let drain = self.drain();
        self.staged.extend(drain);
        self
    }

    /// Stages a trait implementation for the input name.
    ///
    /// This is similar to `stage` but it encapsulates the unstaged TokenStream in a trait implementation.
    ///
    /// The final struct will also be derived with `Clone`.
    pub fn stage_trait(&mut self, input_name: &Ident, trait_path: &Path) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            impl #trait_path for #input_name {
                #(#drain)*
            }
        };
        self.staged.push(result);
        self
    }

    /// Stages a struct for the input name.
    ///
    /// It will generate a struct with the fields defined in the unstaged TokenStream.
    pub fn stage_struct(&mut self, input_name: &Ident) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            #[derive(Clone)]
            pub struct #input_name {
                #(#drain,)*
            }
        };
        self.staged.push(result);
        self
    }

    /// Stages an enum for the input name.
    ///
    /// It will generate an enum with the variants defined in the unstaged TokenStream.
    pub fn stage_enum(&mut self, input_name: &Ident) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            #[derive(Clone)]
            pub enum #input_name {
                #(#drain,)*
            }
        };
        self.staged.push(result);
        self
    }
}

impl ToTokens for VariantBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.staged.clone());
    }
}

impl From<VariantBuilder> for Vec<TokenStream> {
    fn from(builder: VariantBuilder) -> Self {
        builder.staged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_extract_variants() {
        let data = quote! {
            enum MyEnum {
                Variant1(u8),
                Variant2(String),
            }
        };

        let input: DeriveInput = syn::parse2(data).unwrap();
        let data = &input.data;
        let variants = extract_variants(data);

        assert_eq!(variants.0.variant_names.len(), 2);
        assert_eq!("Variant1", variants.0.variant_names[0].to_string());
        assert_eq!("Variant2", variants.0.variant_names[1].to_string());

        assert_eq!(variants.0.variant_types_names.len(), 2);
        assert_eq!("u8", variants.0.variant_types_names[0].to_string());
        assert_eq!("String", variants.0.variant_types_names[1].to_string());

        assert_eq!(variants.0.variant_builder_names.len(), 2);
        assert_eq!("u8Builder", variants.0.variant_builder_names[0].to_string());
        assert_eq!(
            "StringBuilder",
            variants.0.variant_builder_names[1].to_string()
        );
    }

    #[test]
    fn test_stage_trait() {
        let data = quote! {
            enum MyEnum {
                Variant1(u8),
                Variant2(String),
            }
        };

        let mut builder = VariantBuilder::default();

        let input: DeriveInput = syn::parse2(data).unwrap();
        let data = &input.data;
        let variants = extract_variants(data);

        builder.add_iter(&variants.0, |name, _type, _| {
            quote! {
                if let Self::#name = #name(#_type) {
                    true
                } else {
                    false
                };
            }
        });

        let input_name = Ident::new("MyEnum", proc_macro2::Span::call_site());
        let staged = builder.stage_trait(&input_name, &parse_quote! { Path::To::Trait });
        let result = staged.to_token_stream().to_string();

        assert_eq!(
            result,
            quote! {
                impl Path::To::Trait for MyEnum {
                    if let Self::Variant1 = Variant1(u8) {
                        true
                    } else {
                        false
                    };
                    if let Self::Variant2 = Variant2(String) {
                        true
                    } else {
                        false
                    };
                }
            }
            .to_string()
        );
    }

    #[test]
    fn stage_struct() {
        let mut builder = VariantBuilder::default();

        builder
            .add(quote! { pub unique_field: u8 })
            .stage_struct(&parse_quote!(MyStruct));

        let staged = builder.stage();
        let result = staged.to_token_stream().to_string();

        assert_eq!(
            result,
            quote! {
                #[derive(Clone)]
                pub struct MyStruct {
                    pub unique_field: u8,
                }
            }
            .to_string()
        );
    }
}
