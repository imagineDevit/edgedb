use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::DeriveInput;
use crate::utils::derive_utils::{edge_value_quote, filter_quote_, shape_element_quote};
use crate::utils::field_utils::get_struct_fields;


pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast_struct.ident;

    let fields = get_struct_fields(ast_struct.clone())?;

    if fields.len() == 0 {
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                " Filter must have at least one named field"
            )
        );
    }

    let mut i = 0;
    let fields_iter = fields.iter();
    let filters = fields_iter.clone().map(|field| {
        filter_quote_(field, &mut i)
    }).map(|r : syn::Result<_>| r.unwrap_or_else(|e| e.to_compile_error().into()));
    
    let shapes = fields_iter.clone().map(|field| {
        shape_element_quote(field)
    });

    let field_values = fields_iter.clone().map(|field| {
        edge_value_quote(field)
    });

    let tokens = quote! {
          impl edgedb_query::queries::filter::Filter for #struct_name {
            fn to_edgeql(&self, table_name: &str) -> String {
                use edgedb_query::{ToEdgeValue, ToEdgeScalar};
                use edgedb_query::queries::filter::Filter;

                let mut query = "filter".to_owned();
                #(#filters)*
                query
            }


            fn to_edge_value(&self) -> edgedb_protocol::value::Value {

                use edgedb_query::ToEdgeValue;
                use edgedb_query::queries::filter::Filter;

                let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];

                let mut element_names: Vec<String> = vec![];

                 let mut elmt_nb: i16 = -1;

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

    Ok(tokens.into())
}