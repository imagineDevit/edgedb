use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use crate::constants::{EDGEQL, NESTED, OPTION, SCALAR_TYPE, VEC};
use crate::helpers::attributes::{EdgeDbType, SetField};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::{edge_value_quote, format_scalar, shape_element_quote};
use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use crate::utils::type_utils::{get_wrapped_type, is_type_name};

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {

    let struct_name = &ast_struct.ident;

    let fields = get_struct_fields(ast_struct.clone());

    if fields.len() == 0 {
        panic!(r#"
            Set must have at least one named field
        "#)
    }

    let field_iter = fields.iter();

    let assign = field_iter.clone().map(|field| {

        let field_is_option = is_type_name(&field.ty, OPTION);

        let field_is_vec = is_type_name(&field.ty, VEC);

        let f_name = get_field_ident(field);

        let f_ty = &field.ty;

        let tty = if field_is_option {
            get_wrapped_type(f_ty, OPTION)
        } else {
            if field_is_vec {
                get_wrapped_type(f_ty, VEC)
            } else {
                f_ty.clone()
            }
        };

        let is_nested = has_attribute(field, NESTED);

        let assignment =  if is_nested {
            format!("{} := ({}), ", f_name.to_string(), EDGEQL)
        } else {
            SetField::build_field_assignment(field)
        };

        let dd_sign = SCALAR_TYPE.to_string();
        let edgeql = EDGEQL.to_string();

        let format_scalar = format_scalar();

        if field_is_option {
            quote! {
                    if let Some(v) = &self.#f_name {
                        let mut scalar: String = #tty::scalar();
                        #format_scalar;
                        let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, v.to_edgeql().as_str());
                        query.push_str(p.as_str());
                    }
                }
        } else if  field_is_vec {
            quote! {
                    let mut scalar: String = format!("<array{}>", #tty::scalar());
                    #format_scalar;
                    let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, self.#f_name.to_edgeql().as_str());
                    query.push_str(p.as_str());
                }
        } else {
            quote! {
                    let mut scalar: String = #tty::scalar();
                    #format_scalar;
                    let p = #assignment.to_owned()
                        .replace(#dd_sign, scalar.as_str())
                        .replace(#edgeql, self.#f_name.to_edgeql().as_str());
                    query.push_str(p.as_str());
                }

        }
    });


    let mut i: i16 = -1;

    let shapes = field_iter.clone().map(|field| {
        shape_element_quote(field, &mut i)
    });

    let field_values = field_iter.map(|field| {
        edge_value_quote(field)
    });

    let tokens = quote!{
        impl edgedb_query::ToEdgeQl for #struct_name {
            fn to_edgeql(&self) -> String {
                let mut query = "set { ".to_owned();

                #(#assign)*

                query.push_str("}");

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
    };

    tokens.into()
}