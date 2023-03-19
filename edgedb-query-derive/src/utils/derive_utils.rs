use proc_macro2::{Ident, TokenStream};
use quote::quote;

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

pub fn element_shape(f_ident: Ident, f_name: impl Into<String>, field_is_option: bool) -> TokenStream {
    let f_name = f_name.into();

    let q = quote! {
        elmt_nb += 1;
        element_names.push(#f_name.clone().to_owned());
        shapes.push(edgedb_protocol::descriptors::ShapeElement {
                    flag_implicit: false,
                    flag_link_property: false,
                    flag_link: false,
                    cardinality: Some(edgedb_protocol::client_message::Cardinality::One),
                    name: #f_name.to_string(),
                    type_pos: edgedb_protocol::descriptors::TypePos(elmt_nb as u16),
                });
    };

    if field_is_option {
        quote! {
            if let Some(_) = self.#f_ident.clone() { #q}
        }
    } else { q }
}

pub fn element_value(f_name: Ident, field_is_option: bool) -> TokenStream {
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

pub fn nested_element_shape(f_ident: Ident) -> TokenStream {
    quote! {
        match self.#f_ident.to_edge_value() {
            edgedb_protocol::value::Value::Object { shape, fields } => {
                let elements = &shape.elements;
                elements.iter().for_each(|e| {
                    elmt_nb += 1;
                    shapes.push(edgedb_protocol::descriptors::ShapeElement {
                        flag_implicit: false,
                        flag_link_property: false,
                        flag_link: false,
                        cardinality: e.cardinality,
                        name: e.name.clone(),
                        type_pos: edgedb_protocol::descriptors::TypePos(elmt_nb as u16),
                    });


                    if (element_names.contains(&e.name.clone())) {
                        panic!("Duplicate query parameter name found : {}", e.name.clone())
                    } else {
                        element_names.push(e.name.clone());
                    }
                });
            }
            _ => {}
        }
    }
}

pub fn nested_element_value(f_ident: Ident) -> TokenStream {
    quote! {
        match self.#f_ident.to_edge_value() {
            edgedb_protocol::value::Value::Object { shape, fields: fs } => {
                fs.iter().for_each(|f| fields.push(f.clone()));
            }
            _ => {}
        }
    }
}

pub fn conflict_element_shape(f_name: Ident) -> TokenStream {
    quote! {
        if let Some(q) = self.#f_name.else_query() {
             if let edgedb_protocol::value::Value::Object { shape, fields: f_fields } = q.to_edge_value() {

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
}

pub fn conflict_element_value(f_ident: Ident) -> TokenStream {
    quote! {
        if let Some(q) = self.#f_ident.else_query() {
            if let edgedb_protocol::value::Value::Object { shape, fields: f_fields } = q.to_edge_value() {
                f_fields.iter().for_each(|ff| fields.push(ff.clone()));
            }
        }
    }
}
