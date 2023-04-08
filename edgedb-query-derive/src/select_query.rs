use std::convert::TryFrom;

use quote::{quote, ToTokens};
use syn::{Field, Ident, ItemStruct};
use syn::parse::{Parse, ParseStream};
use edgedb_query::QueryType;

use crate::constants::*;
use crate::queries::{Query, QueryField};
use crate::meta_data::{QueryMetaData, try_get_meta};
use crate::builders::impl_builder::{FieldCat, QueryImplBuilder, ImplBuilderField};
use crate::statements::filters::{FilterRequiredQuery, filters_from_fields, FilterStatement};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::type_utils::is_type_name;

// region: SelectQuery
#[derive(Debug, Clone)]
pub struct SelectQuery {
    pub ident: Ident,
    pub meta: Option<QueryMetaData>,
    pub filter_statement: FilterStatement,
    pub options: Option<SelectOptions>,
}

impl SelectQuery {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            meta: None,
            filter_statement: FilterStatement::NoFilter,
            options: None,
        }
    }

    pub fn with_meta(&mut self, meta: QueryMetaData) -> &mut Self {
        self.meta = Some(meta);
        self
    }
}

impl Query for SelectQuery {
    fn get_param_labels(&self) -> Vec<(Ident, String)> {
        self.filter_statement.get_parameter_labels()
    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder> {
        let meta = try_get_meta(&self.ident, || self.meta.clone())?;

        let table_name = meta.table_name();

        let mut fields =  self.filter_statement.to_impl_builder_field();

        let mut edgeql_statements = vec![];

        edgeql_statements.push(meta.result_quote());

        match self.filter_statement {
            FilterStatement::NoFilter => {}
            _ => {
                if meta.has_result() {
                    edgeql_statements.push(quote!(query.push_str(" ");));
                }
            }
        }

        edgeql_statements.extend(self.filter_statement.edgeql_statements(table_name.clone(), false));

        let static_const_check_statements = if let Some(options) = self.options.clone() {
            fields.push(ImplBuilderField {
                field: options.field.clone(),
                field_cat: FieldCat::Ignore,
            });

            edgeql_statements.push(options.statement_quote(table_name.clone(), &meta.result()));

            vec![options.const_check_impl_quote()]
        } else {
            vec![]
        };

        Ok(QueryImplBuilder {
            struct_name: self.ident.clone(),
            table_name: Some(table_name.clone()),
            fields,
            query_type: QueryType::Select,
            static_const_check_statements,
            edgeql_statements,
            has_result: false
        })
    }
}

impl Parse for SelectQuery {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        // region create new query
        let strukt = input.parse::<ItemStruct>()?;

        let mut query = SelectQuery::new(strukt.ident.clone());

        let field_iter = strukt.fields.iter();
        // endregion create new query

        // region handle options field if exists
        let options_fields: Vec<&Field> = field_iter.clone()
            .filter(|f| has_attribute(f, OPTIONS))
            .collect::<Vec<&Field>>();

        match options_fields.len() {
            0 => {}
            1 => {
                query.options = Some(SelectOptions::try_from(options_fields[0])?);
            }
            _ => {
                return Err(syn::Error::new_spanned(options_fields[1].to_token_stream(), ONLY_ONE_OPTIONS_TAG_EXPECTED));
            }
        }

        // endregion handle options field if exists

        // region add filters statements
        query.filter_statement = filters_from_fields(field_iter, vec![OPTIONS], FilterRequiredQuery::Select,INVALID_SELECT_TAG)?;
        // endregion add filters statements

        query.check_duplicate_parameter_labels()?;

        Ok(query)
    }
}

// endregion SelectQuery

// region: SelectOptionsStatement

#[derive(Debug, Clone)]
pub struct SelectOptions {
    pub field: QueryField,
}

impl SelectOptions {
    pub fn const_check_impl_quote(&self) -> proc_macro2::TokenStream {
        let ty = &self.field.ty;
        quote! {
            const _: () = {
                use std::marker::PhantomData;
                struct ImplToSelectOptions<T: edgedb_query::queries::select::Options>(PhantomData<T>);
                let _ = ImplToSelectOptions(PhantomData::<#ty>);
            };
        }
    }

    pub fn statement_quote(&self, table_name: String, result_type_name: &Ident) -> proc_macro2::TokenStream {
        let opt_f_ident = &self.field.ident;

        if is_type_name(&self.field.ty, OPTION) {
            quote! {
                if let Some(v) = self.#opt_f_ident {
                    let c_q = edgedb_query::queries::select::parse_options(&v, #table_name.to_owned(), #result_type_name::returning_fields());
                    query.push_str(c_q.as_str());
                }
            }
        } else {
            quote! {
                let c_q =  edgedb_query::queries::select::parse_options(&self.#opt_f_ident, #table_name.to_owned(), #result_type_name::returning_fields());
                query.push_str(c_q.as_str());
            }
        }
    }
}

impl TryFrom<&Field> for SelectOptions {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        Ok(Self {
            field: QueryField::try_from((field, vec![OPTIONS]))?,
        })
    }
}

// endregion SelectOptionsStatement
