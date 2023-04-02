use proc_macro::TokenStream;
use std::convert::TryFrom;
use syn::{Field, Ident, Type};
use quote::{quote, ToTokens};
use crate::constants::{EDGEQL, OPTION, EXPECTED_ONLY_TAGS};
use crate::utils::attributes_utils::has_only_attributes;
use crate::utils::derive_utils::{element_shape, element_value};
use crate::utils::type_utils::is_type_name;
use crate::builders::impl_builder::QueryImplBuilder;
use crate::tags::set_tag::{SetOption, SetTag};

pub trait Query{
    fn get_param_labels(&self) -> Vec<(Ident, String)>;

    fn check_duplicate_parameter_labels(&self) -> syn::Result<()> {

        let params = self.get_param_labels();

        check_duplicate_parameter_labels(params)

    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder>;

    fn to_token_stream(&self) -> syn::Result<TokenStream> {
        Ok(self.to_impl_builder()?.build().into())
    }
}

#[derive(Debug, Clone)]
pub struct QueryField {
    pub ident: Ident,
    pub ty: Type,
}

impl QueryField {
    pub fn struct_field_quote(&self) -> proc_macro2::TokenStream {
        let f_name = self.ident.clone();
        let f_type = self.ty.clone();

        quote! { pub #f_name: #f_type, }
    }

    pub fn field_shape_quote(&self, param: impl Into<String>) -> proc_macro2::TokenStream {
        let field_name = self.ident.clone();
        let field_type = self.ty.clone();
        element_shape(field_name, param, is_type_name(&field_type, OPTION))
    }

    pub fn field_value_quote(&self) -> proc_macro2::TokenStream {
        let field_name = self.ident.clone();
        let field_type = self.ty.clone();
        element_value(field_name, is_type_name(&field_type, OPTION))
    }

    pub fn build_nested_statement(&self,  set_tag: Option<SetTag>) -> String {
        if let Some(tag) = set_tag {
            let sign = tag.option.statement();
            let column_name = self.ident.clone();
            match tag.option {
                SetOption::Assign => format!("{column_name} {sign} ({EDGEQL}), "),
                SetOption::Concat => format!("{column_name} := .{column_name} ++ ({EDGEQL}), "),
                SetOption::Push => format!("{column_name} += ({EDGEQL}), ")
            }
            //format!("{} {} ({}), ", self.ident, sign, EDGEQL)
        } else {
            format!("{} := ({}), ", self.ident, EDGEQL)
        }

    }

    pub fn add_stmt_quote(&self, to_query: bool, set_tag: Option<SetTag>) -> proc_macro2::TokenStream {
        let f_name = self.ident.clone();

        let edge_ql = EDGEQL.to_string();

        let field_statement = self.build_nested_statement(set_tag);

        let add_quote = if to_query {
            quote!(query.push_str(p.as_str());)
        } else {
            quote!(set_stmt.push_str(p.as_str());)
        };

        quote! {
            let p = #field_statement.to_owned()
                .replace(#edge_ql, self.#f_name.to_edgeql().as_str());

            #add_quote
        }
    }

}

impl TryFrom<(&Field, Vec<&str>)> for QueryField {
    type Error = syn::Error;

    fn try_from((field, tags): (&Field, Vec<&str>)) -> syn::Result<Self> {
        if has_only_attributes(field, tags.clone()) {
            Ok(Self {
                ident: field.ident.clone().unwrap(),
                ty: field.ty.clone(),
            })
        } else {
            Err(syn::Error::new_spanned(field.to_token_stream(), format!("{EXPECTED_ONLY_TAGS} `{tags:?}`")))
        }
    }
}

pub fn check_duplicate_parameter_labels(params: Vec<(Ident, String)>) -> syn::Result<()> {


    for (ident, param) in params.clone() {
        let count = params.clone().into_iter().filter(|s| s.1 == param).count();
        if count > 1 {
            return Err(syn::Error::new_spanned(ident, format!("Duplicate parameter label `{param}`")));
        }
    }

    Ok(())
}