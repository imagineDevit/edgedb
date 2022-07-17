use crate::constants::{DD_SIGN, FILTER, OPTION, SELECT};
use crate::helpers::attributes::Filter;
use crate::utils::derive_utils::start;
use crate::utils::field_utils::get_field_ident;
use crate::utils::type_utils::is_type_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    let struct_name = &ast_struct.ident;

    let (table_name, query_attr, _, options_field, filtered_fields) = start(&ast_struct);

    let result_type_name = query_attr.clone().to_ident(struct_name.span());

    let has_options_attribute = options_field.is_some();

    let filtered_fields = filtered_fields.iter();

    let nb_fields: u8 = filtered_fields.len() as u8;

    let mut index = 0;

    let query_filters = filtered_fields.clone().map(|field| {
        let field_is_option = is_type_name(&field.ty, OPTION);

        let p = Filter::build_filter_assignment(table_name.clone(), field, index);

        index += 1;

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
                let mut scalar: String = self.#f_name.clone().to_edge_scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            }

        }
    });

    let (complete_assignment, const_check_impl_to_select_option) = if has_options_attribute {
        let opt_f = options_field.unwrap();
        let opt_f_ident = get_field_ident(&opt_f);
        let opt_f_ty = opt_f.ty.clone();
        let is_option = is_type_name(&opt_f.ty, OPTION);

        let c = quote! {
            const _: () = {
                use std::marker::PhantomData;
                struct ImplToSelectOptions<T: edgedb_query::queries::select::Options>(PhantomData<T>);
                let _ = ImplToSelectOptions(PhantomData::<#opt_f_ty>);
            };
        };

        (
            if is_option {
                quote! {
                    if let Some(v) = self.#opt_f_ident {
                        let c_q = edgedb_query::queries::select::parse_options(&v);
                        query.push_str(c_q.as_str());
                    }
                }
            } else {
                quote! {
                    let c_q =  edgedb_query::queries::select::parse_options(&self.#opt_f_ident);
                    query.push_str(c_q.as_str());
                }
            },
            c,
        )
    } else {
        let c_q = query_attr.complete_select_query(table_name.clone());
        (
            quote! {
                query.push_str(#c_q);
            },
            quote!(),
        )
    };

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

    let field_values = filtered_fields.clone().map(|field| {
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

    let mut filter_q = String::default();


    if nb_fields > 0 {
        filter_q = format!(" {}", FILTER);
    };

    let q = format!("{} {} ", SELECT, table_name);

    let tokens = quote! {

        #const_check_impl_to_select_option

        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                let mut query = #q.to_owned();

                query.push_str(#result_type_name::shape().as_str());

                query.push_str(#filter_q);

                #(#query_filters)*

                #complete_assignment

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
