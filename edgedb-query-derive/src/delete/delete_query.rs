
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;
use crate::constants::{DELETE, FILTER};
use crate::utils::derive_utils::{edge_value_quote, filter_quote, shape_element_quote, start, StartResult, to_edge_ql_value_impl_empty_quote};
use crate::utils::field_utils::get_field_ident;

pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {

    let struct_name = &ast_struct.ident;


    let StartResult {
        table_name,
        filters_field,
        filtered_fields ,
        ..
    } = start(&ast_struct)?;

    let nb_fields = filtered_fields.len();

    let query_str = format!("{} {}", DELETE, table_name);

    let mut index: usize = 0;

    let mut i: i16 = -1;

    let to_edgeql_value_impls = if let Some(field) = filters_field {

        if nb_fields > 0 {
            return Err(syn::Error::new_spanned(
                field.attrs[0].clone().into_token_stream(),
                "#[filters] and #[filter] attributes cannot coexist"
            ));
        }

        let f_name = get_field_ident(&field);

        quote! {
            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> String {
                    let mut query = #query_str.to_owned();

                    let filter_q = self.#f_name.to_edgeql(#table_name);

                    query.push_str(" ");

                    query.push_str(filter_q.as_str());

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
        let mut filter_q = String::default();

        if nb_fields > 0 {
            filter_q = format!(" {}", FILTER);

            let filtered_fields = filtered_fields.iter();

            let query_filters = filtered_fields.clone().map(|field| {
                filter_quote(field, table_name.clone(), &mut index)
            }).map(|r: syn::Result<_>| r.unwrap_or_else(|e|e.to_compile_error().into()));

            let shapes = filtered_fields.clone().map(|field| {
                shape_element_quote(field, &mut i)
            });

            let field_values = filtered_fields.clone().map(|field| {
                edge_value_quote(field)
            });

            quote! {
                impl edgedb_query::ToEdgeQl for #struct_name {
                    fn to_edgeql(&self) -> String {
                        let mut query = #query_str.to_owned();

                        query.push_str(#filter_q);

                        #(#query_filters)*

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
        } else {
            to_edge_ql_value_impl_empty_quote(struct_name, query_str, None)
        }
    };

    let tokens = quote! {

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

    Ok(tokens.into())
}