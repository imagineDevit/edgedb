use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    // Name of struct
    let struct_name = &ast_struct.ident;

    // Struct fields
    let fields = get_struct_fields(ast_struct.clone());

    let fields_names = fields
        .iter()
        .map(|field| get_field_ident(field).to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let q = format!("{{ {} }}", fields_names);

    let tokens = quote! {
        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                #q.to_owned()
            }
        }
    };

    tokens.into()
}
