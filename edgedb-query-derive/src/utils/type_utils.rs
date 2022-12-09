use syn::{Field, Type, TypeTuple};
use crate::constants::{OPTION, VEC};

/// Check if a type name is equal to the  given name
///
/// __ty__ : the type to check
///
/// __name__ : the expected name
pub fn is_type_name(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Path(ref p) => p.path.segments.len() == 1 && p.path.segments[0].ident == name,
        Type::Tuple(_) => name == "()",
        _ => false,
    }
}

/// Get the given type name
///
/// __ty__ : the type
pub fn get_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(ref p) => {
            //p.path
            //    .segments
            //    .iter()
            //    .map(|s| s.ident.to_string())
            //    .collect::<Vec<String>>()
            //    .join("::")
            p.path.segments[0].ident.to_string()
        }
        Type::Tuple(TypeTuple { elems: _e, .. }) => {
            // if e.len() == 0 {}
            panic!("Tuple types are not supported yet")
        }
        _ => panic!("Only Path and Tuple types are supported yet."),
    }
}

/// Get the type within a given wrapper
///
/// __ty__: the wrapper type
///
/// __wrapper__: the wrapper name
pub fn get_wrapped_type(ty: &Type, wrapper: &str) -> Type {
    assert!(is_type_name(ty, wrapper));

    if let Type::Path(ref p) = ty {
        if let syn::PathArguments::AngleBracketed(ref generic_args) = p.path.segments[0].arguments {
            if let syn::GenericArgument::Type(ref t) = generic_args.args[0] {
                t.clone()
            } else {
                panic!("First wrapped type must be a generic argument.")
            }
        } else {
            panic!("First path segment must be of type syn::PathArguments::AngleBracketed")
        }
    } else {
        panic!("Only Path types are supported for wrapped type field")
    }
}


pub fn get_type(field: &Field, ty: &Type) -> Type {
    if is_type_name(&field.ty, OPTION) {
        get_wrapped_type(ty, OPTION)
    } else {
        if is_type_name(&field.ty, VEC) {
            get_wrapped_type(ty, VEC)
        } else {
            ty.clone()
        }
    }
}