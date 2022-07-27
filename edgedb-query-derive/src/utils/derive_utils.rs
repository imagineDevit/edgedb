use proc_macro2::TokenStream;
use quote::quote;
use crate::helpers::attributes::{EdgeDbMeta, Filter, Filters, Options, Query};
use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use syn::{DeriveInput, Field};
use crate::constants::{SCALAR, OPTION, VEC, TUPLE};
use crate::utils::type_utils::{get_wrapped_type, is_type_name};

pub fn start(ast_struct: &DeriveInput) -> (String, Query, bool, Option<Field>, Option<Field>, Vec<Field>) {
    // Struct fields
    let fields = get_struct_fields(ast_struct.clone());

    let fields_cloned = fields.clone();

    let table_name = if let Some(table_field) = fields_cloned
        .iter()
        .find(|f| EdgeDbMeta::from_field(f).is_valid())
    {
        EdgeDbMeta::from_field(table_field).value().unwrap()
    } else {
        panic!(
            r#"
            Specify the module's and the table's names 
            by adding an attribute of type () with attribute as follow:
            
            #[edgedb(module = "", table="")]
            __meta__: ()
            
        "#
        );
    };

    let fields_cloned = fields.clone();

    let (query_attr, has_result_type) = if let Some(result_field) = fields_cloned
        .iter()
        .find(|f| Query::from_field(f).has_result())
    {
        (Query::from_field(result_field), true)
    } else {
        (Query::default(), false)
    };

    let options_field = fields
        .clone()
        .into_iter()
        .find(|f| Options::from_field(f).is_some());

    let filters_field = fields
        .clone()
        .into_iter()
        .find(|f| Filters::from_field(f).is_some());

    let filtered_fields = fields
        .clone()
        .into_iter()
        .filter(|f| !EdgeDbMeta::from_field(f).is_valid()
            && Options::from_field(f).is_none()
            && Filters::from_field(f).is_none()
        )
        .collect::<Vec<Field>>();


    (
        table_name,
        query_attr,
        has_result_type,
        options_field,
        filters_field,
        filtered_fields,
    )
}

pub fn filter_quote(field: &Field, table_name: String, index: &mut usize) -> TokenStream {

    let p = Filter::build_filter_assignment(table_name.clone(), field, index.clone());

    *index += 1;

    let field_is_vec = is_type_name(&field.ty, VEC);

    let field_is_tuple = is_type_name(&field.ty, TUPLE);

    let f_ty = &field.ty;

    let tty = if field_is_vec {
        get_wrapped_type(f_ty, VEC)
    } else {
        f_ty.clone()
    };

    let assignment = format!("{}", p);

    let dd_sign = SCALAR.to_string();

    let format_scalar = format_scalar();

    if field_is_vec {
        quote! {
            let mut scalar: String = format!("<array{}>", #tty::scalar());
            #format_scalar;
            let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
            query.push_str(p.as_str());
        }
    } else {
        if field_is_tuple {
            quote! {
                let mut scalar: String = <#tty>::scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            }
        } else {
            quote! {
                let mut scalar: String = #tty::scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            }
        }

    }
}

pub fn filter_quote_(field: &Field, index: &mut usize) -> TokenStream {

    let table_name: &str = "__table_name__";

    let p = Filter::build_filter_assignment(table_name.to_string(), field, index.clone());

    *index += 1;

    let field_is_vec = is_type_name(&field.ty, VEC);

    let field_is_tuple = is_type_name(&field.ty, TUPLE);

    let f_ty = &field.ty;

    let tty = if field_is_vec {
        get_wrapped_type(f_ty, VEC)
    } else {
        f_ty.clone()
    };

    let assignment = format!("{}", p);

    let dd_sign = SCALAR.to_string();

    let format_scalar = format_scalar();

    if field_is_vec {
        quote! {
            let mut scalar: String = format!("<array{}>", #tty::scalar());
            #format_scalar;
            let p = #assignment.to_owned()
                .replace(#dd_sign, scalar.as_str())
                .replace(#table_name, table_name);
            query.push_str(p.as_str());
        }
    } else {
        if field_is_tuple {
            quote! {
                let mut scalar: String = <#tty>::scalar();
                #format_scalar;
                let p = #assignment.to_owned()
                    .replace(#dd_sign, scalar.as_str())
                    .replace(#table_name, table_name);
                query.push_str(p.as_str());
            }
        } else {
            quote! {
                let mut scalar: String = #tty::scalar();
                #format_scalar;
                let p = #assignment.to_owned()
                    .replace(#dd_sign, scalar.as_str())
                    .replace(#table_name, table_name);
                query.push_str(p.as_str());
            }
        }
    }

}

pub fn format_scalar() -> TokenStream {
    quote! {
            if !scalar.is_empty() {
                if !scalar.starts_with("<") {
                    scalar = format!("{}{}", "<", scalar);
                }
                if !scalar.trim().ends_with(">") {
                    scalar = format!("{}{}", scalar, ">");
                }
            }
        }
}

pub fn shape_element_quote(field: &Field, index: &mut i16) -> TokenStream {
    let f_ident = get_field_ident(field);
    let f_name = format!("{}", f_ident);
    let field_is_option = is_type_name(&field.ty, OPTION);

    *index += 1;

    if field_is_option {
        quote! {
            if let Some(v) = self.#f_ident.clone() {
                shapes.push(edgedb_protocol::descriptors::ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: Some(edgedb_protocol::client_message::Cardinality::One),
                    name: #f_name.to_string(),
                    type_pos: edgedb_protocol::descriptors::TypePos(#index as u16),
                });
            }
        }
    } else {
        quote! {
            shapes.push(edgedb_protocol::descriptors::ShapeElement {
                flag_implicit: false,
                flag_link_property: false,
                flag_link: false,
                cardinality: Some(edgedb_protocol::client_message::Cardinality::One),
                name: #f_name.to_string(),
                type_pos: edgedb_protocol::descriptors::TypePos(#index as u16),
            });
        }
    }

}

pub fn edge_value_quote(field: &Field) -> TokenStream {
    let field_is_option = is_type_name(&field.ty, OPTION);
    let f_name = get_field_ident(field);

    if field_is_option {
        quote! {
                if let Some(v) = &self.#f_name {
                    fields.push(Some(v.to_edge_value()));
                }
            }
    } else {
        quote! {
                fields.push(Some(self.#f_name.to_edge_value()));
            }
    }
}

pub fn to_edge_ql_value_impl_empty_quote(struct_name: &syn::Ident, query: String, result_type: Option<syn::Ident>) -> TokenStream {

    let push_result_shape = if let Some(result_type_name) = result_type {
        quote! {
            query.push_str(#result_type_name::shape().as_str());
        }
    } else {
        quote!()
    };

    quote! {
            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> String {
                    let mut query = #query.to_owned();
                    #push_result_shape
                    query
                }
            }

            impl edgedb_query::ToEdgeValue for #struct_name {
                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                    edgedb_protocol::value::Value::empty_tuple()
                }
            }
        }
}