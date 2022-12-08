use crate::constants::{SCALAR_TYPE, INSERT, OPTION, SELECT, VEC, NESTED, EDGEQL, CONFLICT_ON, CONFLICT_ELSE};
use crate::helpers::attributes::EdgeDbType;
use crate::utils::{field_utils::*, type_utils::is_type_name};
use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;
use crate::utils::attributes_utils::has_attribute;

use crate::utils::derive_utils::{edge_value_quote, format_scalar, shape_element_quote, start, StartResult, to_edge_ql_value_impl_empty_quote, check_and_duplicate_unless_conflict_value};
use crate::utils::type_utils::get_type;

pub fn do_derive(ast_struct: &DeriveInput) -> syn::Result<TokenStream> {
    // Name of struct
    let struct_name = &ast_struct.ident;

    let StartResult {
        table_name,
        query_result,
        result_field,
        filtered_fields,
        unless_conflict_field,
        ..
    } = start(&ast_struct)?;

    let result_type_name = query_result.to_ident(struct_name.span());

    let mut query_str = format!("{} {}", INSERT, table_name);

    let nb_fields = filtered_fields.len();

    let has_result_type = result_field.is_some();

    let to_edgeql_value_impls = if nb_fields > 0 {
        if has_result_type {
            query_str = format!("{} ( {}", SELECT, query_str)
        }

        let filtered_fields = filtered_fields.iter();

        let query_fields_name = filtered_fields.clone()
            .map(|f| get_field_ident(f).to_string())
            .collect::<Vec<String>>().join(",");


        let (const_check_impl_conflict, unless_conflict_stmt_quote, unless_conflict_else_query_value)= if let Some(ucf) =  unless_conflict_field {
            let ucf_name = get_field_ident(&ucf);

            let ucf_ty = ucf.ty;

            let c = quote! {
                const _: () = {
                    use std::marker::PhantomData;
                    struct ImplConflict<R: edgedb_query::models::edge_query::ToEdgeQuery + Clone,T: edgedb_query::queries::conflict::Conflict<R>>((PhantomData<T>, Option<R>));
                    let _ = ImplConflict((PhantomData::<#ucf_ty>, None));
                };
            };


            (c, quote! {
                let qn = #query_fields_name.split(",").collect::<Vec<&str>>();
                let c_q =  edgedb_query::queries::conflict::parse_conflict(&self.#ucf_name, qn);
                query.push_str(c_q.as_str());
            }, check_and_duplicate_unless_conflict_value(ucf_name.clone()))
        } else {
            (quote!(), quote!(), quote!())
        };

        let add_result_quote = quote! {
            if #has_result_type {
                let shape = #result_type_name::shape();
                query.push_str(")");
                query.push_str(shape.as_str());
            }
        };


        let assign = filtered_fields.clone().map(|field| {
            let field_is_option = is_type_name(&field.ty, OPTION);

            let field_is_vec = is_type_name(&field.ty, VEC);

            let f_name = get_field_ident(field);

            let f_ty = &field.ty;

            let tty = get_type(field, f_ty);

            let is_nested = has_attribute(field, NESTED);

            let assignment = if is_nested {
                format!("{} := ({}), ", f_name.to_string(), EDGEQL)
            } else {
                EdgeDbType::build_field_assignment(field)?
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
            } else if field_is_vec {
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

        let mut i: i16 = -1;

        let shapes = filtered_fields.clone().map(|field| {
            shape_element_quote(field, &mut i)
        });

        let field_values = filtered_fields.map(|field| {
            edge_value_quote(field)
        });


        quote! {

            #const_check_impl_conflict

            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> String {
                    let mut query = #query_str.to_owned();

                    query.push_str(" {");

                    #(#assign)*

                    query.push_str("}");

                    #unless_conflict_stmt_quote;

                    #add_result_quote;

                    query

                }
            }

            impl edgedb_query::ToEdgeValue for #struct_name {

                fn to_edge_value(&self) -> edgedb_protocol::value::Value {

                    let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                    let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];

                    let mut element_names: Vec<String> = vec![];

                    #(#shapes)*

                    #(#field_values)*

                    #unless_conflict_else_query_value;

                    let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();

                    edgedb_protocol::value::Value::Object {
                        shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),
                        fields,
                    }
                }
            }
        }
    } else {
        to_edge_ql_value_impl_empty_quote(struct_name, query_str, None)
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
