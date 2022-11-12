use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, token::Comma, Data::Struct, DataEnum, DataStruct, DeriveInput, Field,
    FieldsNamed, Variant,
};

pub fn get_field_ident(f: &Field) -> Ident {
    f.ident.clone().expect("Field must have ident")
}

pub fn get_struct_fields(ast_struct: DeriveInput) -> syn::Result<Punctuated<Field, Comma>> {
    if let Struct(DataStruct {
        fields: syn::Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = ast_struct.data
    {
        Ok(named)
    } else {
        Err(
            syn::Error::new_spanned(
                ast_struct.ident.to_token_stream(),
                "Only struct data are supported"
            )
        )
       
    }
}

pub fn get_enum_variant(ast_enum: DeriveInput) -> syn::Result<Punctuated<Variant, Comma>> {
    if let syn::Data::Enum(DataEnum { variants, .. }) = ast_enum.data {
        Ok(variants)
    } else {
        Err(
            syn::Error::new_spanned(
                ast_enum.ident.to_token_stream(),
                "Only enum data are supported"
            )
        )
    }
}
