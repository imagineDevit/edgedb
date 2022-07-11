use crate::constants::ENUM;
use crate::utils::constants::edgedb_attributes;
use crate::utils::path_utils::path_ident_equals;
use proc_macro2::Ident;

use syn::{punctuated::Iter, Field, MetaNameValue, NestedMeta, Variant};

pub fn get_field_attribute(
    field: &Field,
    attribute_derive_name: &str,
    attribute_name: &str,
) -> Option<Ident> {
    for att in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref mut nested,
            ..
        })) = att.parse_meta()
        {
            if let Some((true, _)) = path_ident_equals(path, attribute_derive_name) {
                let cloned_nested = nested.clone();
                let mut nested_iter = cloned_nested.iter();

                if let Some(v) = edgedb_attributes().get(attribute_name) {
                    return do_get_field_attribute(&mut nested_iter, attribute_name, v);
                }
            }
        }
    }
    None
}

fn do_get_field_attribute(
    nested_iter: &mut Iter<NestedMeta>,
    attribute_name: &str,
    value: &Option<(String, String)>,
) -> Option<Ident> {
    if let Some(NestedMeta::Meta(syn::Meta::NameValue(MetaNameValue {
        ref path,
        lit: syn::Lit::Str(attribute_value),
        ..
    }))) = nested_iter.next()
    {
        let option = path_ident_equals(path, attribute_name);
        return match option {
            Some((true, _)) => {
                if let Some((expected_value, next_attribute_name)) = value {
                    if attribute_value.value() == expected_value.as_str() {
                        if expected_value == ENUM {}
                        return do_get_field_attribute(
                            nested_iter,
                            next_attribute_name.as_str(),
                            &None,
                        );
                    }
                }

                Some(Ident::new(
                    attribute_value.value().as_str(),
                    attribute_value.span(),
                ))
            }
            Some((false, i)) => panic!(
                "Attribute named '{}' is not expected! Do you mean '{}' ?",
                i.to_string(),
                attribute_name
            ),
            None => None,
        };
    } else {
        None
    }
}

pub fn has_attribute(field: &Field, attribute_derive_name: &str) -> bool {
    for att in &field.attrs {
        if let Some((true, _)) = path_ident_equals(&att.path, attribute_derive_name) {
            return true;
        }
    }

    false
}

pub fn has_attribute_value(
    field: &Field,
    attribute_derive_name: &str,
    attribute_name: &str,
    value: &str,
) -> bool {
    for att in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref mut nested,
            ..
        })) = att.parse_meta()
        {
            if let Some((true, _)) = path_ident_equals(path, attribute_derive_name) {
                let cloned_nested = nested.clone();
                let mut nested_iter = cloned_nested.iter();

                if let Some(NestedMeta::Meta(syn::Meta::NameValue(MetaNameValue {
                    ref path,
                    lit: syn::Lit::Str(attribute_value),
                    ..
                }))) = nested_iter.next()
                {
                    if let Some((true, _)) = path_ident_equals(path, attribute_name) {
                        return attribute_value.value() == value;
                    }
                }
            }
        }
    }

    false
}
