use proc_macro::{TokenStream};

use quote::{quote, ToTokens};
use regex::Regex;
use syn::{DeriveInput, Field};
use crate::constants::PARAM_PATTERN;
use crate::helpers::attributes::{Param, SrcValue};
use crate::utils::derive_utils::{edge_value_quote, shape_element_quote};
use crate::utils::field_utils::{get_field_ident, get_struct_fields};

pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast_struct.ident;

    let fields = get_struct_fields(ast_struct.clone())?;

    if fields.len() == 0 {
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                " FromFileQuery must have at least one field with annotated #[src()]",
            )
        );
    }

    let srcs = fields.clone().into_iter()
        .filter(|f| SrcValue::from_field(f).value.is_some())
        .map(|f| SrcValue::from_field(&f))
        .collect::<Vec<SrcValue>>();

    let src = match srcs.len() {
        n if n < 1 =>
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                "FromFileQuery must have one field annotated #[src()]",
            )
        ),
        n if n > 1 =>
            return Err(
                syn::Error::new_spanned(
                    struct_name.into_token_stream(),
                    "FromFileQuery must have only one field annotated #[src()]",
                )
            ),
        _ => srcs.get(0).unwrap()
    };

    let file_content = src.get_content();

    let Ok(content) = file_content else {
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                file_content.err().unwrap(),
            )
        );
    };

    let param_regex = Regex::new(PARAM_PATTERN).unwrap();

    let param_matches = param_regex.find_iter(content.as_str())
        .map(|mat| mat.as_str().to_string())
        .collect::<Vec<String>>();

    let params = fields.iter()
        .filter(|f| SrcValue::from_field(f).value.is_none())
        .collect::<Vec<&Field>>();

    let params = params.iter();

    let params_values = params.clone()
        .map(|f| Param::from_field(f).value)
        .collect::<Vec<String>>();

    let struct_params_not_query = params_values.clone().into_iter()
        .filter(|s| !param_matches.contains(& format!("${}", s)))
        .collect::<Vec<String>>();

    let query_params_not_struct = param_matches.clone().into_iter()
        .filter(|s| !params_values.contains(&s.replace("$", "")))
        .collect::<Vec<String>>();

    if struct_params_not_query.len() > 0 {
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                format!(r"
                    Following struct attributes do not appear as query parameters : {:#?}
                ",struct_params_not_query),
            )
        )
    } else if query_params_not_struct.len() > 0 {
        return Err(
            syn::Error::new_spanned(
                struct_name.into_token_stream(),
                format!(r"
                    Following query parameters do not appear as struct attribute : {:#?}
                ",query_params_not_struct),
            )
        )
    }


    let mut i: i16 = -1;

    let shapes = params.clone().map(|field| {
        let param_value = Param::from_field(field).value;
        i += 1;
        quote! {
            element_names.push(#param_value.clone().to_owned());
            shapes.push(edgedb_protocol::descriptors::ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: Some(edgedb_protocol::client_message::Cardinality::One),
                    name: #param_value.to_string(),
                    type_pos: edgedb_protocol::descriptors::TypePos(#i as u16),
                });
        }
    });

    let field_values = params.map(|field| {
        edge_value_quote(field)
    });

    let tokens = quote! {

        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                #content.to_string()
            }
        }

        impl edgedb_query::ToEdgeValue for #struct_name {
            fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];
                let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];
                let mut element_names: Vec<String> = vec![];

                #(#shapes)*

                #(#field_values)*

                let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();

                edgedb_protocol::value::Value::Object {
                    shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),
                    fields,
                }
            }
        }

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