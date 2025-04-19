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

use crate::StructHelpers;
use darling::{ast, util};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::{format_ident, ToTokens};
use syn::{Attribute, Path};

pub struct FieldInfo {
    pub ident: Ident,
    pub attr: Vec<Attribute>,
}

use crate::filter::*;

pub trait FieldInfoExtract {
    fn get_field_names(&self) -> Vec<&Ident>;
}

impl FieldInfoExtract for Vec<FieldInfo> {
    fn get_field_names(&self) -> Vec<&Ident> {
        self.iter().map(|field| &field.ident).collect()
    }
}

/// A container for information about struct fields.
///
/// This struct stores:
/// - `field_names`: The names of the fields in the struct.
/// - `field_vec_names`: The names of the fields in the struct that are vectors.
/// - `field_option_names`: The names of the fields in the struct that are options.
///
/// - `field_types_names`: The types of the fields in the struct.
/// - `field_vec_types_names`: The types of the fields in the struct that are vectors.
/// - `field_option_types_names`: The types of the fields in the struct that are options.
///
/// - `field_builder_names`: The builder names derived from the field types.
/// - `field_vec_builder_names`: The builder names derived from the field types that are vectors.
/// - `field_option_builder_names`: The builder names derived from the field types that are options.
///
/// # Example
///
/// ```ignore
/// struct MyStruct {
///    field1: u8,
///    field2: Vec<u8>,
///    field3: Option<u8>,
/// }
///
/// // Extracted as:
///
/// Fields {
///     field_names: vec![field1],
///     field_vec_names: vec![field2],
///     field_option_names: vec![field3],
///
///     field_types_names: vec![u8],
///     field_vec_types_names: vec![u8],
///     field_option_types_names: vec![u8],
///
///     field_builder_names: vec![u8Builder],
///     field_vec_builder_names: vec![u8Builder],
///     field_option_builder_names: vec![u8Builder],
/// }
/// ```
///
pub struct Fields {
    // [Name]: Type
    pub field_names: Vec<FieldInfo>,
    pub field_vec_names: Vec<FieldInfo>,
    pub field_option_names: Vec<FieldInfo>,

    // Field: [Type] -> Ident
    pub field_types_names: Vec<proc_macro2::Ident>,
    pub field_vec_types_names: Vec<proc_macro2::Ident>,
    pub field_option_types_names: Vec<proc_macro2::Ident>,

    // Field(Builder): Type
    pub field_builder_names: Vec<proc_macro2::Ident>,
    pub field_vec_builder_names: Vec<proc_macro2::Ident>,
    pub field_option_builder_names: Vec<proc_macro2::Ident>,
}

impl Fields {
    /// Returns a list of field names in the struct, regardless of type.
    pub fn get_field_names(&self) -> Vec<&Ident> {
        let mut ret = vec![];
        ret.extend(self.field_names.get_field_names());
        ret.extend(self.field_vec_names.get_field_names());
        ret.extend(self.field_option_names.get_field_names());
        ret
    }

    // Returns a list of field types in the struct, regardless of type.
    pub fn get_field_types(&self) -> Vec<&Ident> {
        let mut ret = vec![];
        ret.extend(&self.field_types_names);
        ret.extend(&self.field_vec_types_names);
        ret.extend(&self.field_option_types_names);
        ret
    }

    // Returns a list of field builder names in the struct, regardless of type.
    pub fn get_field_builder_names(&self) -> Vec<&Ident> {
        let mut ret = vec![];
        ret.extend(&self.field_builder_names);
        ret.extend(&self.field_vec_builder_names);
        ret.extend(&self.field_option_builder_names);
        ret
    }
}

/// Extracts field information from a syn::Data struct definition.
///
/// See the `Fields` struct for more information.
pub fn extract_fields(
    data: &ast::Data<util::Ignored, StructHelpers>,
) -> (Fields, Option<syn::Error>) {
    let mut ret_fields = Fields {
        field_names: vec![],
        field_vec_names: vec![],
        field_option_names: vec![],

        field_types_names: vec![],
        field_vec_types_names: vec![],
        field_option_types_names: vec![],

        field_builder_names: vec![],
        field_vec_builder_names: vec![],
        field_option_builder_names: vec![],
    };

    let mut errors: Vec<syn::Error> = vec![];

    data.as_ref()
        .take_struct()
        .unwrap()
        .fields
        .iter()
        .for_each(|field| {
            let result = if is_vec(&field.ty) {
                get_vec_type_name(&field.ty).map(|name| {
                    ret_fields.field_vec_names.push(FieldInfo {
                        ident: field.ident.as_ref().unwrap().clone(),
                        attr: vec![],
                    });
                    ret_fields
                        .field_vec_types_names
                        .push(format_ident!("{}", name));
                    ret_fields
                        .field_vec_builder_names
                        .push(format_ident!("{}Builder", name));
                })
            } else if is_option(&field.ty) {
                get_option_type_name(&field.ty).map(|name| {
                    ret_fields.field_option_names.push(FieldInfo {
                        ident: field.ident.as_ref().unwrap().clone(),
                        attr: vec![],
                    });
                    ret_fields
                        .field_option_types_names
                        .push(format_ident!("{}", name));
                    ret_fields
                        .field_option_builder_names
                        .push(format_ident!("{}Builder", name));
                })
            } else {
                get_type_name(&field.ty).map(|name| {
                    ret_fields.field_names.push(FieldInfo {
                        ident: field.ident.as_ref().unwrap().clone(),
                        attr: vec![],
                    });
                    ret_fields.field_types_names.push(format_ident!("{}", name));
                    ret_fields
                        .field_builder_names
                        .push(format_ident!("{}Builder", name));
                })
            };

            if let Err(err) = result {
                errors.push(err);
            }
        });

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

/// Builder for struct fields.
///
/// This builder stores unstaged TokenStreams that can be staged into a final TokenStream.
#[derive(Default)]
pub struct FieldBuilder {
    staged: Vec<TokenStream>,
    unstaged: Vec<TokenStream>,
}

/// The type of field beign passed to closures.
pub enum FieldType {
    Normal,
    Vec,
    Option,
}

impl FieldBuilder {
    /// Adds an **unstaged** TokenStream to the builder.
    pub fn add(&mut self, field: TokenStream) -> &mut Self {
        self.unstaged.push(field);
        self
    }

    /// Adds an **unstaged** TokenStream to the builder.
    ///
    /// This function takes a closure that will receive all fields in `fields` and return a `TokenStream`.
    ///
    /// The closure takes 5 arguments:
    /// - `FieldType`: The type of the field.
    /// - `Vec<Attribute>`: The attributes of the field.
    /// - `&Ident`: The name of the field.
    /// - `&Ident`: The type of the field.
    /// - `&Ident`: The builder name of the field.
    pub fn add_iter<F>(&mut self, fields: &Fields, f: F) -> &mut Self
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let mut _fields: Vec<TokenStream> = vec![];

        if !fields.field_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply(fields, &f) as _);
        }
        if !fields.field_option_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply_opt(fields, &f) as _);
        }
        if !fields.field_vec_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply_vec(fields, &f) as _);
        }
        self.unstaged.extend(_fields);
        self
    }

    /// Adds an **unstaged** TokenStream to the builder.
    ///
    /// This function takes a closure that will receive all fields in `fields` and return a `TokenStream`.
    ///
    /// The closure takes 5 arguments:
    /// - `FieldType`: The type of the field.
    /// - `Vec<Attribute>`: The attributes of the field.
    /// - `&Ident`: The name of the field.
    /// - `&Ident`: The type of the field.
    /// - `&Ident`: The builder name of the field.
    ///
    /// `before` and `after` are optional TokenStreams that will be added before and after the body, respectively.
    pub fn add_fn_iter<F>(
        &mut self,
        fields: &Fields,
        sig_path: &TokenStream,
        before: Option<TokenStream>,
        body: F,
        after: Option<TokenStream>,
    ) -> &mut Self
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let mut _body: Vec<TokenStream> = vec![];
        if !fields.field_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply(fields, &body) as _);
        }
        if !fields.field_option_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply_opt(fields, &body) as _);
        }
        if !fields.field_vec_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply_vec(fields, &body) as _);
        }

        let mut result = TokenStream::default();
        if let Some(before) = before {
            result.extend(before);
        }

        result.extend(_body);

        if let Some(after) = after {
            result.extend(after);
        }

        self.unstaged.push(quote! {
            #sig_path {
                #result
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

    /// Drains the **unstaged** TokenStream and pushes it to the **staged** TokenStream.
    ///
    /// This function will take all **unstaged** changes and sort them with a comma separator.  
    pub fn stage_fields(&mut self) -> &mut Self {
        let fields = self.drain();
        self.staged.push(quote! { #(#fields,)* });
        self
    }

    /// Stages a trait implementation for the input name.
    ///
    /// This is similar to `stage` but it encapsulates the unstaged TokenStream in a trait implementation.
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
    ///
    /// The final struct will also be derived with `Clone`.
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

    /// Inner fn for applying a closure to all fields.
    fn apply<F>(&mut self, fields: &Fields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_names
            .iter()
            .zip(fields.field_types_names.iter())
            .zip(fields.field_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Normal,
                    &field.attr,
                    &field.ident,
                    field_type,
                    field_builder,
                )
            })
            .collect::<Vec<_>>()
    }

    /// Inner fn for applying a closure to all fields that are options.
    fn apply_opt<F>(&mut self, fields: &Fields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_option_names
            .iter()
            .zip(fields.field_option_types_names.iter())
            .zip(fields.field_option_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Option,
                    &field.attr,
                    &field.ident,
                    field_type,
                    field_builder,
                )
            })
            .collect::<Vec<_>>()
    }

    /// Inner fn for applying a closure to all fields that are vectors.
    fn apply_vec<F>(&mut self, fields: &Fields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_vec_names
            .iter()
            .zip(fields.field_vec_types_names.iter())
            .zip(fields.field_vec_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Vec,
                    &field.attr,
                    &field.ident,
                    field_type,
                    field_builder,
                )
            })
            .collect::<Vec<_>>()
    }
}

impl ToTokens for FieldBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.staged.clone());
    }
}

impl From<FieldBuilder> for Vec<TokenStream> {
    fn from(builder: FieldBuilder) -> Self {
        builder.staged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StructInput;
    use darling::FromDeriveInput;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn text_extract_fields() {
        let data = quote! {
            struct MyStruct {
                field1: u8,
                field2: Vec<u8>,
                field3: Option<u8>,
            }
        };

        let input: DeriveInput = syn::parse2(data).unwrap();
        let derive_input = StructInput::from_derive_input(&input).unwrap();

        let fields = extract_fields(&derive_input.data).0;
        assert_eq!(fields.field_names.len(), 1);
        assert_eq!(fields.field_vec_names.len(), 1);
        assert_eq!(fields.field_option_names.len(), 1);

        assert_eq!(fields.field_types_names.len(), 1);
        assert_eq!(fields.field_vec_types_names.len(), 1);
        assert_eq!(fields.field_option_types_names.len(), 1);

        assert_eq!(fields.field_builder_names.len(), 1);
        assert_eq!(fields.field_vec_builder_names.len(), 1);
        assert_eq!(fields.field_option_builder_names.len(), 1);
    }

    #[test]
    fn stage_fields() {
        let data = quote! {
            struct MyStruct {
                field1: u8,
                field2: Vec<u8>,
                field3: Option<u8>,
            }
        };

        let input: DeriveInput = syn::parse2(data).unwrap();
        let derive_input = StructInput::from_derive_input(&input).unwrap();

        let fields = extract_fields(&derive_input.data);

        // Transform fields into a Rc<RefCell<**field**>> for testing
        let mut builder = FieldBuilder::default();
        builder.add_iter(&fields.0, |_, _, name, _type, _| {
            quote! {
                #name: Rc<RefCell<#_type>>
            }
        });

        let staged = builder.stage_fields();
        let result = staged.to_token_stream().to_string();

        // Since get_raw_type_name only returns the lowest type, Vec and Option are ommited
        assert_eq!(
            result,
            quote! {
                field1: Rc<RefCell<u8>>,
                field3: Rc<RefCell<u8>>,
                field2: Rc<RefCell<u8>>,
            }
            .to_string()
        );
    }

    #[test]
    fn stage_trait() {
        let data = quote! {
            struct MyStruct {
                field1: u8,
                field2: Vec<u8>,
                field3: Option<u8>,
            }
        };

        let input: DeriveInput = syn::parse2(data).unwrap();
        let derive_input = StructInput::from_derive_input(&input).unwrap();

        let fields = extract_fields(&derive_input.data);

        let mut builder = FieldBuilder::default();
        builder.add_iter(&fields.0, |_, _, name, _type, _| {
            quote! {
                #name.do_stuff();
            }
        });

        let staged = builder.stage_trait(&input.ident, &parse_quote! { Path::To::Trait });
        let result = staged.to_token_stream().to_string();

        assert_eq!(
            result,
            quote! {
                impl Path::To::Trait for MyStruct {
                    field1.do_stuff();
                    field3.do_stuff();
                    field2.do_stuff();
                }
            }
            .to_string()
        );
    }

    #[test]
    fn stage_struct() {
        use super::*;
        use crate::StructInput;
        use darling::FromDeriveInput;
        use syn::DeriveInput;

        let data = quote! {
            struct MyStruct {
                field1: u8,
                field2: Vec<u8>,
                field3: Option<u8>,
            }
        };

        let input: DeriveInput = syn::parse2(data).unwrap();
        let derive_input = StructInput::from_derive_input(&input).unwrap();

        let fields = extract_fields(&derive_input.data);

        let mut builder = FieldBuilder::default();
        builder.add_iter(&fields.0, |_, _, name, _type, _| {
            quote! {
                #name: Rc<RefCell<#_type>>
            }
        });

        let staged = builder.stage_struct(&input.ident);
        let result = staged.to_token_stream().to_string();

        // Since get_raw_type_name only returns the lowest type, Vec and Option are ommited
        assert_eq!(
            result,
            quote! {
                #[derive(Clone)]
                pub struct MyStruct {
                    field1: Rc<RefCell<u8>>,
                    field3: Rc<RefCell<u8>>,
                    field2: Rc<RefCell<u8>>,
                }
            }
            .to_string()
        );
    }
}
