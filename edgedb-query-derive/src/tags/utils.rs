use syn::Field;
use crate::constants::AT;
use crate::utils::field_utils::get_field_name;

pub fn validate_link_property(column_name: Option<String>, link_property: Option<bool>, field: &Field) -> syn::Result<()> {
    match (column_name.clone(), link_property) {
        (Some(c), Some(false)) =>{

            if c.starts_with(AT) {
                Err(syn::Error::new_spanned(
                    field, "Only link property column can have name starting with @"))
            } else {
                Ok(())
            }
        }
        _ => {
            Ok(())
        }
    }
}

pub fn get_column_name(column_name: Option<String>, link_property: Option<bool>, field: &Field) -> String {
    let c = column_name.clone().unwrap_or(get_field_name(field));

    if let Some(true) = link_property {
        if c.starts_with(AT) {
            c
        } else {
            format!("{AT}{c}")
        }
    } else {
        c
    }
}