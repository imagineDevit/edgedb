use crate::helpers::attributes::EdgeEnumValue;
use crate::utils::field_utils::get_enum_variant;
use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

pub fn do_derive(ast_struct: &DeriveInput) -> TokenStream {
    let enum_name = &ast_struct.ident;

    // get enum variants
    let enum_variants = get_enum_variant(ast_struct.clone());

    let v_cloned = enum_variants.clone();

    let v_idents = v_cloned.iter().map(|v| {
        let ident = &v.ident;
        let variant_name = ident.to_string();

        let i = EdgeEnumValue::from_variant(v)
            .value
            .or_else(|| Some(variant_name))
            .unwrap();

        let value = format!("{}", i);

        quote! {
            #enum_name::#ident => #value.to_owned()
        }
    });

    let name = format!("{}", enum_name);
    let tokens = quote! {

        impl edgedb_query::ToEdgeQl for #enum_name {
            fn to_edgeql(&self) -> String {
                match self {
                    #(#v_idents,)*
                }
            }
        }
        impl edgedb_query::ToEdgeScalar for #enum_name {
            fn scalar() -> String {
                #name.to_owned()
            }
        }

         impl edgedb_query::ToEdgeValue for #enum_name {
            fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                edgedb_protocol::value::Value::Enum(edgedb_protocol::codec::EnumValue::from(self.to_edgeql().as_str()))
            }
        }

        impl ToString for #enum_name {
            fn to_string(&self) -> String {
                self.to_edgeql()
            }
        }
    };
    tokens.into()
}
