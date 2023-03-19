#![allow(dead_code)]

use crate::utils::path_utils::path_ident_equals;
use syn::Field;

pub fn has_attribute(field: &Field, attribute_derive_name: &str) -> bool {
    let mut result = false;
    for att in &field.attrs {
        if let Some((true, _)) = path_ident_equals(&att.path, attribute_derive_name) {
            result = true;
            break;
        }
    }

    result
}

pub fn has_any_attribute(field: &Field, attribute_derive_names: Vec<&str>) -> bool {
    let mut result = false;
    for att in &field.attrs {
        if attribute_derive_names.contains(&att.path.get_ident().unwrap().to_string().as_str()) {
            result = true;
            break;
        }
    }

    result
}

pub fn has_only_attributes(field: &Field, attribute_derive_names: Vec<&str>) -> bool {
    let mut result = true;
    for att in &field.attrs {
        if !attribute_derive_names.contains(&att.path.get_ident().unwrap().to_string().as_str()) {
            result = false;
            break;
        }
    }

    result
}

pub fn has_none_attribute(field: &Field, attribute_derive_names: Vec<&str>) -> bool {
    !has_any_attribute(field, attribute_derive_names)
}
