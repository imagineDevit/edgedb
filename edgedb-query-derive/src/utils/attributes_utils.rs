use crate::utils::path_utils::path_ident_equals;

use syn::{Field, MetaNameValue, NestedMeta};

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
