use syn::DeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use crate::constants::{RESULT, SET, UPDATE};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::{start, StartResult};
use crate::utils::field_utils::get_field_ident;
use crate::utils::path_utils::path_ident_equals;

pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast_struct.ident;

    let StartResult {
        table_name,
        result_field,
        filters_field,
        filtered_fields,
        ..
    } = start(&ast_struct)?;

    if let Some(f) = result_field {
        let att = f.attrs.into_iter().find(|p| {
            let op = path_ident_equals(&p.path, RESULT);

            if let Some((true, _)) = op {
                true
            } else {
                false
            }
        }).unwrap();

        return Err(
            syn::Error::new_spanned(
                att.into_token_stream(),
                "Result type is not allowed for update query"
            )
        );
    }


    if filtered_fields.len() != 1 {
        return Err(
            syn::Error::new_spanned(
                ast_struct.ident.clone().into_token_stream(),
                "Update query struct must have all his fields annotated"
            )
        );
    }

    let set_field = if let Some(f) = filtered_fields.get(0){
        if !has_attribute(f, SET) {
            return Err(
                syn::Error::new_spanned(
                    ast_struct.ident.clone().into_token_stream(),
                    "Update query struct must have all his fields annotated (one annotated #[set]) "
                )
            );
        }
        f
    } else {
        return Err(
            syn::Error::new_spanned(
                ast_struct.ident.clone().into_token_stream(),
                "Update query struct must have all his fields annotated (one annotated #[set]) "
            )
        );
    };

    let query_str = format!("{} {}", UPDATE, table_name);

    let quote_value= |f_name: Ident| -> proc_macro2::TokenStream {
        quote! {
                if let edgedb_protocol::value::Value::Object { shape, fields } = self.#f_name.to_edge_value() {
                    fields.iter().for_each(|ff| f_fields.push(ff.clone()));
                    shape.elements.iter().for_each(|e| {
                        let n = e.name.clone();
                        let c = e.cardinality.clone();
                        let el = edgedb_protocol::codec::ShapeElement {
                            flag_implicit: false,
                            flag_link_property: false,
                            flag_link: false,
                            cardinality: c,
                            name: n
                        };

                        if (element_names.contains(&e.name.clone())) {
                            panic!("Duplicate query parameter name found : {}", e.name.clone())
                        } else {
                            element_names.push(e.name.clone());
                        }

                        shapes.push(el);
                    });
                }
            }
    };


    let (filter_quote, filter_value_quote) = if let Some(field) = filters_field {
        let f_name = get_field_ident(&field);
        let qv = quote_value(f_name.clone());
        (
            quote! {
                let filter_q = self.#f_name.to_edgeql(#table_name);
                query.push_str(" ");
                query.push_str(filter_q.as_str());
            },
            qv
        )
    } else {
        (quote!(), quote!())
    };

    let (set_quote, set_value_quote) = {
        let f_name = get_field_ident(set_field);
        let qv = quote_value(f_name.clone());
        (
            quote! {
                let set_q = self.#f_name.to_edgeql();
                query.push_str(" ");
                query.push_str(set_q.as_str());
            },
            qv
        )
    };

    let tokens = quote! {

         impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                let mut query = #query_str.to_owned();
                #filter_quote
                #set_quote
                query
            }
        }
        impl edgedb_query::ToEdgeValue for #struct_name {
            fn to_edge_value(&self) -> edgedb_protocol::value::Value {

                let mut f_fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                let mut shapes:  Vec<edgedb_protocol::codec::ShapeElement> = vec![];

                let mut element_names: Vec<String> = vec![];

                #filter_value_quote
                #set_value_quote

                edgedb_protocol::value::Value::Object {
                    shape: edgedb_protocol::codec::ObjectShape::new(shapes),
                    fields: f_fields,
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

    Ok(tokens.into())
}