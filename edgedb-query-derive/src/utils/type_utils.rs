#![allow(unused)]

use syn::{Field, Type, TypeTuple};
use syn::__private::bool;
use edgedb_query::ToEdgeScalar;
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
            "()".to_string()
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

pub fn get_type( ty: &Type) -> Type {
    if is_type_name(ty, OPTION) {
        get_wrapped_type(ty, OPTION)
    } else if is_type_name(ty, VEC) {
        get_wrapped_type(ty, VEC)
    } else {
        ty.clone()
    }
}

pub fn get_scalar(ty: &Type) -> syn::Result<String> {
    scalar_types()
        .into_iter()
        .find(|(t, _)| get_type_name(&get_type(ty)) == *t)
        .map(|(_, s)| {
            if is_type_name(ty, VEC){
                Ok(format!("<array<{s}>>"))
            }  else if s.is_empty() {
                Ok(s.to_string())
            } else {
                Ok(format!("<{s}>"))
            }
        })
        .unwrap_or_else(|| {
            Err(syn::Error::new_spanned(
                ty,
                format!("Unsupported typo: {}", get_type_name(ty)),
            ))
        })
}

pub fn match_scalar(ty: &Type, scalar: impl Into<String> ) -> syn::Result<()> {

    let sc =scalar.into().replace(['<', '>'], "") ;

    let x = scalar_types().iter().map(|t| t.1).any(|s| s == sc.as_str());

    if !x { return Ok(()) }

    let scalar_found = get_scalar(ty).map_err(|e|
        syn::Error::new_spanned(
            ty,
            format!("Rust type {} does not match with scalar  type: {}", get_type_name(ty), sc),
        )
    )?;

    let scalar = if is_type_name(ty, VEC) { format!("<array<{sc}>>") } else { sc };

    if scalar_found.replace(['<', '>'], "") == scalar.replace(['<', '>'], "") {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            ty,
            format!("Rust type {} does not match with scalar  type: {}",  get_type_name(ty), scalar),
        ))
    }
}

pub fn scalar_types() -> Vec<(&'static str, &'static str)>{
    vec![
        ("String", "str"),
        ("i8", "int16"),
        ("i16", "int16"),
        ("i32", "int32"),
        ("i64", "int64"),
        ("f32", "float32"),
        ("f64", "float64"),
        ("bool", "bool"),
        ("uuid::Uuid", "uuid"),
        ("serde_json::Value", "json"),
        ("chrono::DateTime<chrono::Utc>", "datetime"),
        ("chrono::DateTime<chrono::Local>", "cal::local_datetime"),
        ("chrono::Duration", "<duration>"),
        ("chrono::Date<chrono::Local>", "cal::local_date"),
        ("chrono::NaiveTime", "cal::local_time"),
        ("chrono::NaiveDate", "cal::local_date"),
        ("()", ""),
    ]
}
