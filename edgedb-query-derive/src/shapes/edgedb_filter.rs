use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;
use crate::utils::derive_utils::{edge_value_quote, filter_quote_, shape_element_quote};
use crate::utils::field_utils::get_struct_fields;


pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    let struct_name = &ast_struct.ident;

    let fields = get_struct_fields(ast_struct.clone());

    if fields.len() == 0 {
        panic!(r#"
            Filter must have at least one named field
        "#)
    }

    let mut i = 0;
    let fields_iter = fields.iter();
    let filters = fields_iter.clone().map(|field| {
        filter_quote_(field, &mut i)
    });

    let mut i: i16 = -1;

    let shapes = fields_iter.clone().map(|field| {
        shape_element_quote(field, &mut i)
    });

    let field_values = fields_iter.clone().map(|field| {
        edge_value_quote(field)
    });

    let tokens = quote! {
          impl edgedb_query::queries::filter::Filter for #struct_name {
            fn to_edgeql(&self, table_name: &str) -> String {
                let mut query = "filter".to_owned();
                #(#filters)*
                query
            }


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
    };

    tokens.into()
}