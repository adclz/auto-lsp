use super::super::utilities::filter::{get_raw_type_name, is_option, is_vec};
use quote::format_ident;
use syn::token::Enum;

pub struct StructFields {
    // [Name]: Type
    pub field_names: Vec<proc_macro2::Ident>,
    pub field_vec_names: Vec<proc_macro2::Ident>,
    pub field_option_names: Vec<proc_macro2::Ident>,

    // Field: [Type] -> Ident
    pub field_types_names: Vec<proc_macro2::Ident>,
    pub field_vec_types_names: Vec<proc_macro2::Ident>,
    pub field_option_types_names: Vec<proc_macro2::Ident>,

    // Field(Builder): Type
    pub field_builder_names: Vec<proc_macro2::Ident>,
    pub field_vec_builder_names: Vec<proc_macro2::Ident>,
    pub field_option_builder_names: Vec<proc_macro2::Ident>,

    // Optionnal comma separator between fields because rust is strict with macro interpolations
    pub commas: Vec<syn::token::Comma>,
    pub option_commas: Vec<syn::token::Comma>,
}

pub struct EnumFields {
    // [Name]: Type
    pub variant_names: Vec<proc_macro2::Ident>,
    pub variant_vec_names: Vec<proc_macro2::Ident>,
    pub variant_option_names: Vec<proc_macro2::Ident>,

    // Variant: [Type] -> Ident
    pub variant_types_names: Vec<proc_macro2::Ident>,
    pub variant_vec_types_names: Vec<proc_macro2::Ident>,
    pub variant_option_types_names: Vec<proc_macro2::Ident>,

    // Variant(Builder): Type
    pub variant_builder_names: Vec<proc_macro2::Ident>,
    pub variant_vec_builder_names: Vec<proc_macro2::Ident>,
    pub variant_option_builder_names: Vec<proc_macro2::Ident>,

    // Optionnal comma separator between fields because rust is strict with macro interpolations
    pub commas: Vec<syn::token::Comma>,
    pub option_commas: Vec<syn::token::Comma>,

    pub len: u32,
}

pub fn match_fields(data: &syn::Data) -> StructFields {
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

        option_commas: vec![],
        commas: vec![],
    };

    match data {
        syn::Data::Struct(ref struct_data) => match &struct_data.fields {
            syn::Fields::Named(fields) => {
                fields.named.iter().for_each(|field| {
                    if let true = is_vec(&field.ty) {
                        ret_fields
                            .field_vec_names
                            .push(field.ident.as_ref().unwrap().clone());
                        ret_fields
                            .field_vec_types_names
                            .push(format_ident!("{}", get_raw_type_name(&field.ty)));
                        ret_fields
                            .field_vec_builder_names
                            .push(format_ident!("{}Builder", get_raw_type_name(&field.ty)));
                    } else if let true = is_option(&field.ty) {
                        ret_fields
                            .field_option_names
                            .push(field.ident.as_ref().unwrap().clone());
                        ret_fields
                            .field_option_types_names
                            .push(format_ident!("{}", get_raw_type_name(&field.ty)));
                        ret_fields
                            .field_option_builder_names
                            .push(format_ident!("{}Builder", get_raw_type_name(&field.ty)));
                    } else {
                        ret_fields
                            .field_names
                            .push(field.ident.as_ref().unwrap().clone());
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

    if ret_fields.field_names.len() > 0
        && (ret_fields.field_vec_names.len() > 0 || ret_fields.field_option_names.len() > 0)
    {
        ret_fields.commas.push(syn::token::Comma::default());
    }

    if ret_fields.field_option_names.len() > 0 && ret_fields.field_vec_names.len() > 0 {
        ret_fields.option_commas.push(syn::token::Comma::default());
    }
    ret_fields
}

pub fn match_enum_fields(data: &syn::ItemEnum) -> EnumFields {
    let mut ret_fields = EnumFields {
        variant_names: vec![],
        variant_vec_names: vec![],
        variant_option_names: vec![],

        variant_types_names: vec![],
        variant_vec_types_names: vec![],
        variant_option_types_names: vec![],

        variant_builder_names: vec![],
        variant_vec_builder_names: vec![],
        variant_option_builder_names: vec![],

        commas: vec![],
        option_commas: vec![],

        len: 0,
    };

    ret_fields.len = data.variants.len() as u32;
    for variant in &data.variants {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let first_field = fields.unnamed.first().unwrap();
                //panic!("{:?}", variant_name);

                if let true = is_vec(&first_field.ty) {
                    ret_fields.variant_vec_names.push(variant_name.clone());
                    ret_fields
                        .variant_vec_types_names
                        .push(format_ident!("{}", get_raw_type_name(&first_field.ty)));
                    ret_fields.variant_vec_builder_names.push(format_ident!(
                        "{}Builder",
                        get_raw_type_name(&first_field.ty)
                    ));
                } else if let true = is_option(&first_field.ty) {
                    ret_fields.variant_option_names.push(variant_name.clone());
                    ret_fields
                        .variant_option_types_names
                        .push(format_ident!("{}", get_raw_type_name(&first_field.ty)));
                    ret_fields.variant_option_builder_names.push(format_ident!(
                        "{}Builder",
                        get_raw_type_name(&first_field.ty)
                    ));
                } else {
                    ret_fields.variant_names.push(variant_name.clone());
                    ret_fields
                        .variant_types_names
                        .push(format_ident!("{}", get_raw_type_name(&first_field.ty)));
                    ret_fields.variant_builder_names.push(format_ident!(
                        "{}Builder",
                        get_raw_type_name(&first_field.ty)
                    ));
                };
            }
            _ => panic!("This proc macro only works with enums"),
        }
    }

    if ret_fields.variant_names.len() > 0
        && (ret_fields.variant_vec_names.len() > 0 || ret_fields.variant_option_names.len() > 0)
    {
        ret_fields.commas.push(syn::token::Comma::default());
    }

    if ret_fields.variant_option_names.len() > 0 && ret_fields.variant_vec_names.len() > 0 {
        ret_fields.option_commas.push(syn::token::Comma::default());
    }

    ret_fields
}
