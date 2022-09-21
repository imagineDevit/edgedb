use syn::DeriveInput;
use proc_macro::{TokenStream};
use quote::quote;
use crate::constants::{SET, UPDATE};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::start;
use crate::utils::field_utils::get_field_ident;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    let struct_name = &ast_struct.ident;

    let (table_name, _query_attr, has_result_type, _, filters, filtered_fields) = start(&ast_struct);

    if has_result_type {
        panic!("Result type is not allowed for update query")
    }

    if filtered_fields.len() != 1 {
        panic!("Update query struct must have only one unannotated field")
    }

    let query_str = format!("{} {}", UPDATE, table_name);

    let (filter_quote, filter_value_quote) = if let Some(field) = filters {
        let f_name = get_field_ident(&field);
        (
            quote! {
                let filter_q = self.#f_name.to_edgeql(#table_name);
                query.push_str(" ");
                query.push_str(filter_q.as_str());
            },
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
        )
    } else {
        (quote!(), quote!())
    };

    let (set_quote, set_value_quote) = if let Some(field) = filtered_fields.iter().next() {
        if !has_attribute(&field, SET) {
            panic!("")
        }
        let f_name = get_field_ident(&field);
        (
            quote! {
                let set_q = self.#f_name.to_edgeql();
                query.push_str(" ");
                query.push_str(set_q.as_str());
            },
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
        )
    } else {
        panic!("")
    };

    //let result_quote = if has_result_type {
    //    let result_type_name = query_attr.to_ident(struct_name.span());
    //    quote! {
    //        query.push_str(" ");
    //        query.push_str(#result_type_name::shape().as_str());
    //    }
    //} else {
    //    quote!()
    //};

    let tokens = quote! {

         impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                let mut query = #query_str.to_owned();
                #filter_quote
                #set_quote
                //#result_quote
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

    tokens.into()
}