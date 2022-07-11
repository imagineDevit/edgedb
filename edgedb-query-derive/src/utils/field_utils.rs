use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, token::Comma, Data::Struct, DataEnum, DataStruct, DeriveInput, Field,
    FieldsNamed, Variant,
};

use crate::constants::{DD_SIGN, FILTER, INF_SIGN, SELECT, SUP_SIGN};
use crate::{constants::TYPE, utils::attributes_utils::get_field_attribute};

pub fn get_field_ident(f: &Field) -> Ident {
    f.ident.clone().expect("Field must have ident")
}

/// Build insert query assignment expression
///
/// __field__ : the field
pub fn build_filter_assignment(table_name: String, field: &Field) -> String {
    let db_type = if let Some(ident) = get_field_attribute(&field.clone(), FILTER, TYPE) {
        let mut i = ident.to_string();
        if !i.is_empty() {
            if !i.starts_with(INF_SIGN) {
                i = format!("{}{}", INF_SIGN, i);
            }
            if !i.trim().ends_with(SUP_SIGN) {
                i = format!("{}{}", i, SUP_SIGN);
            }
        }

        i
    } else {
        DD_SIGN.to_string()
    };

    format!(
        "{table}.{field_name} = ({select} {edge_type}\"${field_name}\") ",
        table = table_name,
        select = SELECT,
        edge_type = db_type,
        field_name = get_field_ident(field)
    )
}

pub fn get_struct_fields(ast_struct: DeriveInput) -> Punctuated<Field, Comma> {
    if let Struct(DataStruct {
        fields: syn::Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = ast_struct.data
    {
        named
    } else {
        panic!("Only struct data are supported");
    }
}

pub fn get_enum_variant(ast_enum: DeriveInput) -> Punctuated<Variant, Comma> {
    if let syn::Data::Enum(DataEnum { variants, .. }) = ast_enum.data {
        variants
    } else {
        panic!("Only enum data are supported")
    }
}
