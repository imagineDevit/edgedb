use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::ItemStruct;
use syn::parse::{Parse, ParseStream};
use crate::constants::{__TABLENAME__, INVALID_FILTERS_TAG};
use crate::statements::filters::{FilterRequiredQuery, filters_from_fields, FilterStatement};

pub struct EdgedbFilters {
    pub ident: Ident,
    pub filter_statement: FilterStatement,
}


impl EdgedbFilters {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            filter_statement: FilterStatement::NoFilter,
        }
    }

    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {
        let struct_name = self.ident.clone();

        let fields_quote = self.filter_statement.struct_field_quote();

        let filters = self.filter_statement.edgeql_statements(__TABLENAME__, true);

        let shapes = self.filter_statement.shape_quote();

        let values = self.filter_statement.value_quote();

        let tokens = quote! {

            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #fields_quote
            }

            impl edgedb_query::queries::filter::Filter for #struct_name {
                fn to_edgeql(&self, table_name: &str) -> String {
                    use edgedb_query::{ToEdgeQl, EdgeQl};
                    let mut query = String::new();
                    #(#filters)*
                    query
                }


                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
                    use edgedb_query::ToEdgeValue;

                    let mut fields: Vec<Option<edgedb_protocol::value::Value>> = vec![];

                    let mut shapes:  Vec<edgedb_protocol::descriptors::ShapeElement> = vec![];

                    let mut element_names: Vec<String> = vec![];

                     let mut elmt_nb: i16 = -1;

                    #shapes

                    let shape_slices: &[edgedb_protocol::descriptors::ShapeElement] = shapes.as_slice();

                    #values

                    edgedb_protocol::value::Value::Object {

                        shape: edgedb_protocol::codec::ObjectShape::from(shape_slices),

                        fields,
                    }
                }
            }
        };

        Ok(tokens.into())
    }
}

impl Parse for EdgedbFilters {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let strukt = input.parse::<ItemStruct>()?;

        let mut filters = EdgedbFilters::new(strukt.ident.clone());

        filters.filter_statement = filters_from_fields(strukt.fields.iter(), vec![], FilterRequiredQuery::Other, INVALID_FILTERS_TAG)?;

        filters.filter_statement.check_duplicate_parameter_labels()?;
        
        Ok(filters)
    }
}