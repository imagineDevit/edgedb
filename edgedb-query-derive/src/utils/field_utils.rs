use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, token::Comma, Data::Struct, DataEnum, DataStruct, DeriveInput, Field,
    FieldsNamed, Variant,
};

pub fn get_field_ident(f: &Field) -> Ident {
    f.ident.clone().expect("Field must have ident")
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
