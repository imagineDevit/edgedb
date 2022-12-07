use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Field};

use crate::constants::{CONFLICT_ELSE, OPTION, RESULT, SCALAR_TYPE, TUPLE, VEC};
use crate::helpers::attributes::{EdgeDbMeta, Filter, Filters, Options, QueryResult};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::field_utils::{get_field_ident, get_struct_fields};
use crate::utils::type_utils::{get_wrapped_type, is_type_name};

pub struct StartResult {
    pub table_name: String,
    pub query_result: QueryResult,
    pub result_field: Option<Field>,
    pub options_field: Option<Field>,
    pub filters_field: Option<Field>,
    pub filtered_fields: Vec<Field>,
    pub conflict_else_field: Option<Field>,
}

pub fn start(ast_struct: &DeriveInput) -> syn::Result<StartResult> {
    // Struct fields
    let fields = get_struct_fields(ast_struct.clone())?;

    let fields_cloned = fields.clone();

    let table_name = if let Some(table_field) = fields_cloned
        .iter()
        .find(|f| EdgeDbMeta::from_field(f).is_valid())
    {
        EdgeDbMeta::from_field(table_field).value().unwrap()
    } else {
        return Err(syn::Error::new_spanned(
            ast_struct.ident.to_token_stream(),
            r#"
            Specify the module's and the table's names
            by adding an attribute of type () with attribute as follow:

            #[meta(module = "", table="")]
            __meta__: ()

        "#,
        ));
    };

    let fields_cloned = fields.clone();

    let (query_attr, result_f) = if let Some(result_field) = fields_cloned
        .iter()
        .find(|f| has_attribute(f, RESULT))
    {
        (QueryResult::from_field(result_field)?, Some(result_field.clone()))
    } else {
        (QueryResult::default(), None)
    };

    let options_field = fields
        .clone()
        .into_iter()
        .find(|f| Options::from_field(f).is_some());

    let filters_field = fields
        .clone()
        .into_iter()
        .find(|f| Filters::from_field(f).is_some());

    let conflict_else_field = fields
        .clone()
        .into_iter()
        .find(|f| has_attribute(f, CONFLICT_ELSE));

    let filtered_fields = fields
        .clone()
        .into_iter()
        .filter(|f| !EdgeDbMeta::from_field(f).is_valid()
            && Options::from_field(f).is_none()
            && Filters::from_field(f).is_none()
            && !has_attribute(f, CONFLICT_ELSE)
        )
        .collect::<Vec<Field>>();


    Ok(StartResult {
        table_name,
        query_result: query_attr,
        result_field: result_f,
        options_field,
        filters_field,
        filtered_fields,
        conflict_else_field,
    })
}

pub fn filter_quote(field: &Field, table_name: String, index: &mut usize) -> syn::Result<TokenStream> {
    let p = Filter::build_filter_assignment(table_name.clone(), field, index.clone())?;

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

    let dd_sign = SCALAR_TYPE.to_string();

    let format_scalar = format_scalar();

    if field_is_vec {
        Ok(quote! {
            let mut scalar: String = format!("<array{}>", #tty::scalar());
            #format_scalar;
            let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
            query.push_str(p.as_str());
        })
    } else {
        if field_is_tuple {
            Ok(quote! {
                let mut scalar: String = <#tty>::scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            })
        } else {
            Ok(quote! {
                let mut scalar: String = #tty::scalar();
                #format_scalar;
                let p = #assignment.to_owned().replace(#dd_sign, scalar.as_str());
                query.push_str(p.as_str());
            })
        }
    }
}

pub fn filter_quote_(field: &Field, index: &mut usize) -> syn::Result<TokenStream> {
    let table_name: &str = "__table_name__";

    let p = Filter::build_filter_assignment(table_name.to_string(), field, index.clone())?;

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

    let dd_sign = SCALAR_TYPE.to_string();

    let format_scalar = format_scalar();

    if field_is_vec {
        Ok(quote! {
            let mut scalar: String = format!("<array{}>", #tty::scalar());
            #format_scalar;
            let p = #assignment.to_owned()
                .replace(#dd_sign, scalar.as_str())
                .replace(#table_name, table_name);
            query.push_str(p.as_str());
        })
    } else {
        if field_is_tuple {
            Ok(quote! {
                let mut scalar: String = <#tty>::scalar();
                #format_scalar;
                let p = #assignment.to_owned()
                    .replace(#dd_sign, scalar.as_str())
                    .replace(#table_name, table_name);
                query.push_str(p.as_str());
            })
        } else {
            Ok(quote! {
                let mut scalar: String = #tty::scalar();
                #format_scalar;
                let p = #assignment.to_owned()
                    .replace(#dd_sign, scalar.as_str())
                    .replace(#table_name, table_name);
                query.push_str(p.as_str());
            })
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
            element_names.push(#f_name.clone().to_owned());
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
            element_names.push(#f_name.clone().to_owned());
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
                    edgedb_protocol::value::Value::Nothing
                }
            }
        }
}

pub fn check_and_duplicate_value(f_name: syn::Ident) -> TokenStream {

    quote! {
        if let edgedb_protocol::value::Value::Object { shape, fields: f_fields } = self.#f_name.to_edge_value() {
            f_fields.iter().for_each(|ff| fields.push(ff.clone()));

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
}

pub fn check_and_duplicate_value_2(f_name: syn::Ident) -> TokenStream {
    quote! {
        if let edgedb_protocol::value::Value::Object { shape, fields: f_fields } = self.#f_name.to_edge_value() {
            f_fields.iter().for_each(|ff| fields.push(ff.clone()));
            let mut i = shapes.len() - 1;
            shape.elements.iter().for_each(|e| {
                let n = e.name.clone();
                let c = e.cardinality.clone();

                let el = edgedb_protocol::descriptors::ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: c,
                    name: n,
                    type_pos: edgedb_protocol::descriptors::TypePos(i as u16)
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
}