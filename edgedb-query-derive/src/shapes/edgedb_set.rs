use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::DeriveInput;
use crate::constants::{EDGEQL, NESTED_QUERY, OPTION, SCALAR_TYPE, VEC};
use crate::helpers::attributes::SetField;
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::{add_nested_query_shape, add_nested_query_value, edge_value_quote, format_scalar, shape_element_quote};
use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use crate::utils::type_utils::{get_type, is_type_name};

pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {

    let struct_name = &ast_struct.ident;

    let fields = get_struct_fields(ast_struct.clone())?;

    if fields.len() == 0 {
        return Err(syn::Error::new(
            Span::mixed_site(),
            " Set must have at least one named field"
        ));
    }

    let field_iter = fields.iter();

    let assign_result = field_iter.clone().map(|field| {

        let field_is_option = is_type_name(&field.ty, OPTION);

        let field_is_vec = is_type_name(&field.ty, VEC);

        let f_name = get_field_ident(field);

        let f_ty = &field.ty;

        let tty = get_type(field, f_ty);

        let is_nested = has_attribute(field, NESTED_QUERY);

        let assignment =  if is_nested {
            format!("{} := ({}), ", f_name.to_string(), EDGEQL)
        } else {
            SetField::build_field_assignment(field)?
        };


        let dd_sign = SCALAR_TYPE.to_string();
        let edgeql = EDGEQL.to_string();

        let format_scalar = format_scalar();

        if field_is_option {
            Ok(quote! {
                    if let Some(v) = &self.#f_name {
                        let mut scalar: String = #tty::scalar();
                        #format_scalar;
                        let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, v.to_edgeql().as_str());
                        query.push_str(p.as_str());
                    }
                })
        } else if  field_is_vec {
            Ok(quote! {
                    let mut scalar: String = format!("<array{}>", #tty::scalar());
                    #format_scalar;
                    let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, self.#f_name.to_edgeql().as_str());
                    query.push_str(p.as_str());
                })
        } else {
            Ok(quote! {
                    let mut scalar: String = #tty::scalar();
                    #format_scalar;
                    let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, self.#f_name.to_edgeql().as_str());
                    query.push_str(p.as_str());
                })
        }
    }).map(|r: syn::Result<_>| r.unwrap_or_else(|e| e.to_compile_error().into()));
    

    let shapes = field_iter.clone()
        .filter(|f| !has_attribute(f, NESTED_QUERY))
        .map(|field| {
        shape_element_quote(field)
    });

    let nested_query_shapes = field_iter.clone()
        .filter(|f| has_attribute(f, NESTED_QUERY))
        .map(|field| { add_nested_query_shape(field) });

    let field_values = field_iter.clone()
        .filter(|f| !has_attribute(f, NESTED_QUERY))
        .map(|field| {
        edge_value_quote(field)
    });

    let nested_query_values = field_iter.clone()
        .filter(|f| has_attribute(f, NESTED_QUERY))
        .map(|field|{ add_nested_query_value(field) });

    let tokens = quote!{
        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                
                use edgedb_query::ToEdgeScalar;
                
                let mut query = "set { ".to_owned();

                #(#assign_result)*

                query.pop();
                query.pop();

                query.push_str("}");

                query
            }
        }

        impl edgedb_query::ToEdgeValue for #struct_name {

            fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];
                let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];
                let mut element_names: Vec<String> = vec![];
                let mut elmt_nb: i16 = -1;
                #(#shapes)*
                #(#nested_query_shapes)*
                let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();
                #(#field_values)*
                #(#nested_query_values)*
                edgedb_protocol::value::Value::Object {
                    shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),
                    fields,
                }
            }
        }
    };

    Ok(tokens.into())
}