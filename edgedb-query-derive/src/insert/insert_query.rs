use crate::constants::{BRACKET_OPEN, DD_SIGN, INSERT, OPTION, SELECT};
use crate::helpers::attributes::EdgeDbType;
use crate::utils::{field_utils::*, type_utils::is_type_name};
use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

use crate::utils::derive_utils::start;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    // Name of struct
    let struct_name = &ast_struct.ident;

    let (table_name, query_attr, has_result_type, _, filtered_fields) = start(&ast_struct);

    let result_type_name = query_attr.to_ident(struct_name.span());

    let filtered_fields = filtered_fields.iter();

    let assign = filtered_fields.clone().map(|field| {
        let field_is_option = is_type_name(&field.ty, OPTION);

        let p = EdgeDbType::build_field_assignment(field);

        let f_name = get_field_ident(field);

        let assignment = format!("{}", p);

        let dd_sign = DD_SIGN.to_string();

        let format_scalar = quote! {
            if !scalar.is_empty() {
                if !scalar.starts_with("<") {
                    scalar = format!("{}{}", "<", scalar);
                }
                if !scalar.trim().ends_with(">") {
                    scalar = format!("{}{}", scalar, ">");
                }
            }
        };

        if field_is_option {
            quote! {
                if let Some(v) = &self.#f_name {
                    let mut scalar: String = v.to_edge_scalar();
                    #format_scalar;
                    let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                    query.push_str(p.as_str());
                }
            }
        } else {
            quote! {
                let mut scalar: String = self.#f_name.to_edge_scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            }
        }
    });

    let mut i: i16 = -1;

    let shapes = filtered_fields.clone().map(|field| {
        let f_name = format!("{}", get_field_ident(field));
        i = i + 1;
        quote! {
            edgedb_protocol::descriptors::ShapeElement {
                flag_implicit: false,
                flag_link_property: false,
                flag_link: false,
                cardinality: Some(edgedb_protocol::client_message::Cardinality::One),
                name: #f_name.to_string(),
                type_pos: edgedb_protocol::descriptors::TypePos(#i as u16),
            }
        }
    });

    let field_values = filtered_fields.map(|field| {
        let field_is_option = is_type_name(&field.ty, OPTION);
        let f_name = get_field_ident(field);

        if field_is_option {
            quote! {
                if let Some(v) = &self.#f_name {
                    fields.push(Some(v.to_edge_value()));
                }
            }
        } else {
            quote! {
                fields.push(Some(self.#f_name.to_edge_value()));
            }
        }
    });

    let mut q = format!("{} {} {} ", INSERT, table_name, BRACKET_OPEN);

    if has_result_type {
        q = format!("{} ( {}", SELECT, q)
    }

    let add_result_quote = quote! {
        if #has_result_type {
            let shape = #result_type_name::shape();
            query.push_str(")");
            query.push_str(shape.as_str());
        }
    };

    let tokens = quote! {

        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                let mut query = #q.to_owned();

                #(#assign)*

                query.push_str("}");

                #add_result_quote;

                query.push_str(" limit 1");

                query

            }
        }

        impl edgedb_query::ToEdgeScalar for #struct_name {
            fn to_edge_scalar(&self) -> String {
                String::default()
            }
        }

        impl edgedb_query::ToEdgeValue for #struct_name {

            fn to_edge_value(&self) -> edgedb_protocol::value::Value {

                let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                let shape: &[edgedb_protocol::descriptors::ShapeElement] = &[
                    #(#shapes),*
                ];

                #(#field_values)*

                edgedb_protocol::value::Value::Object {
                    shape: edgedb_protocol::codec::ObjectShape::from(shape),
                    fields,
                }
            }
        }

        impl edgedb_query::models::edge_query::ToEdgeQuery for #struct_name {}

        impl ToString for #struct_name {
            fn to_string(&self) -> String {
                self.to_edgeql()
            }
        }
    };

    tokens.into()
}
