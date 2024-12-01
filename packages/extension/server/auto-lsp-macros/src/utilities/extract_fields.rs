use super::super::utilities::filter::{get_raw_type_name, is_option, is_vec};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens};
use syn::Attribute;

pub struct FieldInfo {
    pub ident: Ident,
    pub attr: Vec<Attribute>,
}

pub trait FieldInfoExtract {
    fn get_field_names<'a>(&'a self) -> Vec<&'a Ident>;
}

impl FieldInfoExtract for Vec<FieldInfo> {
    fn get_field_names<'a>(&'a self) -> Vec<&'a Ident> {
        self.iter().map(|field| &field.ident).collect()
    }
}

pub struct StructFields {
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

impl StructFields {
    pub fn get_field_names<'a>(&'a self) -> Vec<&'a Ident> {
        let mut ret = vec![];
        ret.extend(self.field_names.get_field_names());
        ret.extend(self.field_vec_names.get_field_names());
        ret.extend(self.field_option_names.get_field_names());
        ret
    }

    pub fn get_field_types<'a>(&'a self) -> Vec<&'a Ident> {
        let mut ret = vec![];
        ret.extend(&self.field_types_names);
        ret.extend(&self.field_vec_types_names);
        ret.extend(&self.field_option_types_names);
        ret
    }

    pub fn get_field_builder_names<'a>(&'a self) -> Vec<&'a Ident> {
        let mut ret = vec![];
        ret.extend(&self.field_builder_names);
        ret.extend(&self.field_vec_builder_names);
        ret.extend(&self.field_option_builder_names);
        ret
    }
}

pub struct EnumFields {
    // [Name]: Type
    pub variant_names: Vec<proc_macro2::Ident>,

    // Variant: [Type] -> Ident
    pub variant_types_names: Vec<proc_macro2::Ident>,

    // Variant(Builder): Type
    pub variant_builder_names: Vec<proc_macro2::Ident>,
}

pub fn match_struct_fields(data: &syn::Data) -> StructFields {
    let mut ret_fields = StructFields {
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

    match data {
        syn::Data::Struct(ref struct_data) => match &struct_data.fields {
            syn::Fields::Named(fields) => {
                fields.named.iter().for_each(|field| {
                    if let true = is_vec(&field.ty) {
                        ret_fields.field_vec_names.push(FieldInfo {
                            ident: field.ident.as_ref().unwrap().clone(),
                            attr: field.attrs.clone(),
                        });
                        ret_fields
                            .field_vec_types_names
                            .push(format_ident!("{}", get_raw_type_name(&field.ty)));
                        ret_fields
                            .field_vec_builder_names
                            .push(format_ident!("{}Builder", get_raw_type_name(&field.ty)));
                    } else if let true = is_option(&field.ty) {
                        ret_fields.field_option_names.push(FieldInfo {
                            ident: field.ident.as_ref().unwrap().clone(),
                            attr: field.attrs.clone(),
                        });
                        ret_fields
                            .field_option_types_names
                            .push(format_ident!("{}", get_raw_type_name(&field.ty)));
                        ret_fields
                            .field_option_builder_names
                            .push(format_ident!("{}Builder", get_raw_type_name(&field.ty)));
                    } else {
                        ret_fields.field_names.push(FieldInfo {
                            ident: field.ident.as_ref().unwrap().clone(),
                            attr: field.attrs.clone(),
                        });
                        ret_fields
                            .field_types_names
                            .push(format_ident!("{}", get_raw_type_name(&field.ty)));
                        ret_fields
                            .field_builder_names
                            .push(format_ident!("{}Builder", get_raw_type_name(&field.ty)));
                    }
                });
            }
            _ => panic!("This proc macro only works with struct"),
        },
        _ => panic!("This proc macro only works with struct"),
    };
    ret_fields
}

pub fn match_enum_fields(data: &syn::Data) -> EnumFields {
    let mut ret_fields = EnumFields {
        variant_names: vec![],

        variant_types_names: vec![],

        variant_builder_names: vec![],
    };
    match data {
        syn::Data::Enum(ref enum_data) => {
            for variant in &enum_data.variants {
                let variant_name = &variant.ident;
                match &variant.fields {
                    syn::Fields::Unnamed(fields) => {
                        let first_field = fields.unnamed.first().unwrap();
                        ret_fields.variant_names.push(variant_name.clone());
                        ret_fields
                            .variant_types_names
                            .push(format_ident!("{}", get_raw_type_name(&first_field.ty)));
                        ret_fields.variant_builder_names.push(format_ident!(
                            "{}Builder",
                            get_raw_type_name(&first_field.ty)
                        ));
                    }
                    _ => panic!("This proc macro only works with enums"),
                }
            }
        }
        _ => panic!("This proc macro only works with enums"),
    }

    ret_fields
}

pub struct FieldBuilder<'a> {
    fields: &'a StructFields,
    results: Vec<TokenStream>,
}

pub enum FieldBuilderType {
    Normal,
    Vec,
    Option,
}

impl<'a> FieldBuilder<'a> {
    pub fn new(fields: &'a StructFields) -> Self {
        Self {
            fields,
            results: vec![],
        }
    }

    pub fn apply_all<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(FieldBuilderType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        if !self.fields.field_names.is_empty() {
            self.apply(&f);
        }
        if !self.fields.field_option_names.is_empty() {
            self.apply_opt(&f);
        }
        if !self.fields.field_vec_names.is_empty() {
            self.apply_vec(&f);
        }
        self
    }

    pub fn apply<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(FieldBuilderType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let results = self
            .fields
            .field_names
            .iter()
            .zip(self.fields.field_types_names.iter())
            .zip(self.fields.field_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldBuilderType::Normal,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect();
        self.results.push(results);
        self
    }

    pub fn apply_opt<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(FieldBuilderType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let results = self
            .fields
            .field_option_names
            .iter()
            .zip(self.fields.field_option_types_names.iter())
            .zip(self.fields.field_option_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldBuilderType::Option,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect();
        self.results.push(results);
        self
    }

    pub fn apply_vec<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(FieldBuilderType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let results = self
            .fields
            .field_vec_names
            .iter()
            .zip(self.fields.field_vec_types_names.iter())
            .zip(self.fields.field_vec_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldBuilderType::Vec,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect();
        self.results.push(results);
        self
    }
}

impl ToTokens for FieldBuilder<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.results.iter().for_each(|result| {
            tokens.extend(result.clone());
        });
    }
}

impl<'a> From<FieldBuilder<'a>> for Vec<TokenStream> {
    fn from(builder: FieldBuilder<'a>) -> Self {
        builder.results
    }
}
