
use crate::queries::QueryField;
use syn::Ident;
use quote::quote;
use proc_macro2::TokenStream;
use edgedb_query::QueryType;
use crate::utils::derive_utils::{conflict_element_shape, conflict_element_value, nested_element_shape, nested_element_value};

#[derive(Clone, PartialEq)]
pub enum FieldCat {
    Simple(String),
    Nested,
    Conflict,
    Ignore,
}

#[derive(Clone)]
pub struct ImplBuilderField {
    pub field: QueryField,
    pub field_cat: FieldCat,
}

pub struct QueryImplBuilder {
    pub struct_name: Ident,
    pub query_type: QueryType,
    pub table_name: Option<String>,
    pub fields: Vec<ImplBuilderField>,
    pub has_result: bool,
    //pub init_edgeql: String,
    pub static_const_check_statements: Vec<TokenStream>,
    pub edgeql_statements: Vec<TokenStream>,
}

impl QueryImplBuilder {
    pub fn build_struct(&self) -> TokenStream {
        let struct_name = self.struct_name.clone();

        let fields_quote = self.fields.iter().map(|field| {
            field.field.struct_field_quote()
        });

        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #(#fields_quote)*
            }
        }
    }

    pub fn build_to_edgeql_impl(&self) -> TokenStream {
        let struct_name = self.struct_name.clone();

        let query_str = String::default();

        let table_name = self.table_name.clone().unwrap_or(String::new());

        let q_ty = format!("{}", self.query_type);

        let h_r = self.has_result;

        let stmts: Vec<TokenStream> = self.edgeql_statements.clone();

        quote! {
            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> edgedb_query::EdgeQl {
                    use edgedb_query::ToEdgeScalar;
                    use edgedb_query::queries::filter::Filter;
                    use edgedb_query::EdgeResult;

                    let mut query = #query_str.to_owned();

                    #(#stmts)*

                    edgedb_query::EdgeQl {
                        query_type: edgedb_query::QueryType::from(#q_ty.to_string()),
                        table_name: #table_name.to_string(),
                        content: query,
                        has_result: #h_r
                    }
                }
            }
        }
    }

    pub fn build_to_edgevalue_impl(&self) -> TokenStream {
        let struct_name = self.struct_name.clone();

        let fields = self.fields.iter();

        let nb_field_not_ignore = fields.clone().filter(|f| f.field_cat != FieldCat::Ignore).count();

        if nb_field_not_ignore == 0 {
            quote! {
               impl edgedb_query::ToEdgeValue for #struct_name {
                    fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                        edgedb_protocol::value::Value::Nothing
                    }
                }
            }
        } else {
            let shapes = fields.clone()
                .map(|f| {
                    match f.field_cat.clone() {
                        FieldCat::Simple(param) => f.field.field_shape_quote(param),
                        FieldCat::Nested => nested_element_shape(f.field.ident.clone()),
                        FieldCat::Conflict => conflict_element_shape(f.field.ident.clone()),
                        _ => quote!()
                    }
                });

            let values = fields.clone()
                .map(|f| {
                    match f.field_cat.clone() {
                        FieldCat::Nested => nested_element_value(f.field.ident.clone()),
                        FieldCat::Conflict => conflict_element_value(f.field.ident.clone()),
                        FieldCat::Simple(_) => f.field.field_value_quote(),
                        _ => quote!()
                    }
                });

            quote! {
            impl edgedb_query::ToEdgeValue for #struct_name {
                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                        use edgedb_query::queries::filter::Filter;

                        let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];
                        let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];
                        let mut element_names: Vec<String> = vec![];
                        let mut elmt_nb: i16 = -1;

                        #(#shapes)*

                        #(#values)*

                        let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();

                        edgedb_protocol::value::Value::Object {
                            shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),
                            fields,
                        }
                }
            }
        }
        }
    }

    pub fn build(&self) -> TokenStream {
        let struct_name = self.struct_name.clone();

        let const_check_quote = self.static_const_check_statements.clone();

        let table_name = self.table_name.clone().unwrap_or(String::new());
        let struct_quote = self.build_struct();

        let edge_ql_impl_quote = self.build_to_edgeql_impl();

        let edge_value_impl_quote = self.build_to_edgevalue_impl();

        quote! {

            #(#const_check_quote)*

            #struct_quote

            #edge_ql_impl_quote

            #edge_value_impl_quote

            impl edgedb_query::ToEdgeScalar for #struct_name {
                fn scalar() -> String {
                    format!("<{}>", #table_name)
                }
            }

            impl edgedb_query::models::edge_query::ToEdgeQuery for #struct_name {}

            impl ToString for #struct_name {
                fn to_string(&self) -> String {
                    use edgedb_query::ToEdgeQl;
                    self.to_edgeql().to_string()
                }
            }
        }
    }
}

