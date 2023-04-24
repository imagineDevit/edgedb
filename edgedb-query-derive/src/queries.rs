use proc_macro::TokenStream;
use std::convert::TryFrom;
use syn::{Field, Ident, Type};
use quote::{quote, ToTokens};
use crate::constants::{OPTION, EXPECTED_ONLY_TAGS};
use crate::utils::attributes_utils::has_only_attributes;
use crate::utils::derive_utils::{element_shape, element_value};
use crate::utils::type_utils::is_type_name;
use crate::builders::impl_builder::QueryImplBuilder;


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