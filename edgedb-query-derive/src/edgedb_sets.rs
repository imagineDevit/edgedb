use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::ItemStruct;
use syn::parse::{Parse, ParseStream};
use crate::constants::INVALID_SETS_TAG;
use crate::statements::set::{sets_from_fields, UpdateSetStatement};

pub struct EdgedbSets {
    pub ident: Ident,
    pub set_statement: UpdateSetStatement,
}

impl EdgedbSets {
    fn new(ident: Ident) -> Self {
        Self{
           ident,
            set_statement: UpdateSetStatement::None
        }
    }


    pub fn to_token_stream(&self) -> syn::Result<TokenStream> {

        let struct_name = self.ident.clone();

        let fields_quote = self.set_statement.struct_field_quote();

        let add_set = self.set_statement.add_set_statement_quote();

        let shapes = self.set_statement.shape_quote();

        let values = self.set_statement.value_quote();

        let tokens =  quote!{

            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #fields_quote
            }

            impl edgedb_query::ToEdgeQl for #struct_name {
                fn to_edgeql(&self) -> String {
                    #add_set
                    set_stmt
                }
            }

            impl edgedb_query::ToEdgeValue for #struct_name {

                fn to_edge_value(&self) -> edgedb_protocol::value::Value {
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

impl Parse for EdgedbSets {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let strukt = input.parse::<ItemStruct>()?;

        let mut sets = EdgedbSets::new(strukt.ident.clone());

        sets.set_statement = sets_from_fields(strukt.fields.iter(), vec![], true, INVALID_SETS_TAG)?;

        sets.set_statement.check_duplicate_parameter_labels()?;

        Ok(sets)
    }
}