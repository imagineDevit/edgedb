use crate::constants::{FILTER, OPTION, SELECT};
use crate::utils::derive_utils::{edge_value_quote, filter_quote, shape_element_quote, start};
use crate::utils::field_utils::get_field_ident;
use crate::utils::type_utils::is_type_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    let struct_name = &ast_struct.ident;

    let (table_name, query_attr, _, options_field, filters_field, filtered_fields) = start(&ast_struct);

    let result_type_name = query_attr.clone().to_ident(struct_name.span());

    let has_options_attribute = options_field.is_some();

    let filtered_fields = filtered_fields.iter();

    let nb_fields: u8 = filtered_fields.len() as u8;

    let mut filter_q = String::default();

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


    let q = format!("{} {} ", SELECT, table_name);

    let mut index: usize = 0;
    let mut i: i16 = -1;

    let to_edgeql_value_impls=  if let Some(field) = filters_field {
        if nb_fields > 0 {
            panic!("");
        }
        let f_name = get_field_ident(&field);


        quote! {
            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> String {
                    let mut query = #q.to_owned();

                    query.push_str(#result_type_name::shape().as_str());

                    query.push_str(#filter_q);

                    let filter_q = self.#f_name.to_edgeql(#table_name);

                    query.push_str(" ");

                    query.push_str(filter_q.as_str());

                    #complete_assignment

                    query

                }
            }


            impl edgedb_query::ToEdgeValue for #struct_name {

                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                     self.#f_name.to_edge_value()
                }
            }
        }

    } else {
        if nb_fields > 0 {
            filter_q = format!(" {}", FILTER);
        };


        let query_filters = filtered_fields.clone().map(|field| {
            filter_quote(field, table_name.clone(), &mut index)
        });

        let shapes = filtered_fields.clone().map(|field| {
            shape_element_quote(field, &mut i)
        });

        let field_values = filtered_fields.clone().map(|field| {
            edge_value_quote(field)
        });

        quote! {
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


            impl edgedb_query::ToEdgeValue for #struct_name {

                fn to_edge_value(&self) -> edgedb_protocol::value::Value {

                    let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                    let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];

                    #(#shapes)*

                    let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();

                    #(#field_values)*

                    edgedb_protocol::value::Value::Object {
                        shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),
                        fields,
                    }
                }
            }
        }
    };


    let tokens = quote! {

        #const_check_impl_to_select_option

        #to_edgeql_value_impls

        impl edgedb_query::ToEdgeScalar for #struct_name {
            fn scalar() -> String {
                String::default()
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
