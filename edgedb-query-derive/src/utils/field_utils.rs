use proc_macro2::Ident;
use syn::Field;

pub fn get_field_ident(f: &Field) -> Ident {
    f.ident.clone().expect("Field must have ident")
}

pub fn get_field_name(f: &Field) -> String {
    f.ident.as_ref().unwrap().to_string()
}