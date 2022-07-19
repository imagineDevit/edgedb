use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;
use syn::spanned::Spanned;
use crate::constants::QUERY_SHAPE;
use crate::helpers::attributes::QueryShape;
use crate::utils::attributes_utils::has_attribute;
use crate::utils::type_utils::is_type_name;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    // Name of struct
    let struct_name = &ast_struct.ident;

    // Struct fields
    let fields = get_struct_fields(ast_struct.clone());

    let fields_names = fields
        .iter()
        .map(|field| {
            let f_ident = get_field_ident(field);
            let f_ty = &field.ty;
            let f_name = format!("{}", f_ident.to_string());

            let is_query_shape = has_attribute(field, QUERY_SHAPE);

            if is_query_shape {
                let (ql, result_type_name) = QueryShape::build_assignment(field);
                let result_type = Ident::new(result_type_name.as_str(), field.span());
                let is_vec = is_type_name(f_ty, "Vec");

                let limit = if is_vec {
                    ""
                } else {
                   " limit 1"
                };

                quote! {
                    let rs = #result_type::shape();
                    let s = format!("{} := ({}{}{})", #f_name, #ql, rs, #limit);
                    query.push_str(s.as_str());
                    query.push_str(",");
                }
            } else {
                quote! {
                    let shape = #f_ty::shape();
                    if shape.is_empty() {
                        query.push_str(#f_name);
                     query.push_str(",");
                    } else {
                        let s = format!("{} : {}", #f_name, shape);
                        query.push_str(s.as_str());
                        query.push_str(",");
                    }
                }
            }

        });


    let tokens = quote! {

        impl edgedb_query::ToEdgeShape for #struct_name {
            fn shape() -> String {
                let mut query = "{".to_string();
                #(#fields_names)*
                query.pop();
                query.push_str("}");
                query
            }
        }
    };

    tokens.into()
}
