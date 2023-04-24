use std::convert::TryFrom;
use quote::quote;

use syn::Field;

use crate::constants::{EDGEQL, NESTED_QUERY};
use crate::queries::QueryField;
use crate::statements::filters::QueryFilter;
use crate::statements::set::UpdateSet;

// region NestedQueryField
#[derive(Debug, Clone)]
pub struct NestedQueryField {
    pub field: QueryField,
    pub set: Option<UpdateSet>,
    pub filter: Option<QueryFilter>,
    pub parent_table_name: Option<String>
}


pub enum NestedQueryParentType {
    Query, Set, Filter(bool)
}

impl TryFrom<(&Field, NestedQueryParentType)> for NestedQueryField {
    type Error = syn::Error;

    fn try_from((field, parent_type): (&Field, NestedQueryParentType)) -> Result<Self, Self::Error> {

        let (set, filter, query_field) =  match parent_type {
            NestedQueryParentType::Query => {
                (None,   None, QueryField::try_from((field, vec![NESTED_QUERY]))?)
            }
            NestedQueryParentType::Set => {
                let set = UpdateSet::try_from(field)?;
                (Some(set.clone()), None, set.field)
            },
            NestedQueryParentType::Filter(first) => {
                let filter = QueryFilter::try_from((field, first))?;
                (None, Some(filter.clone()),  filter.field)
            }
        };
        Ok(Self {
            field: query_field,
            set,
            filter,
            parent_table_name: None
        })
    }
}

impl NestedQueryField {
    
    pub fn set_parent_table_name(&mut self, name: impl Into<String>) {
        self.parent_table_name = Some(name.into())
    }
    
    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        self.add_stmt_quote(true).unwrap_or_else(|e| e.to_compile_error())
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        self.add_stmt_quote(false).unwrap_or_else(|e| e.to_compile_error())
    }

    pub fn build_statement(&self) -> syn::Result<String> {
        if let Some(set) = self.set.as_ref() {
            Ok(set.build_statement(true))
        } else if let Some( filter) = self.filter.as_ref() {
            filter.build_statement(self.parent_table_name.clone().unwrap_or(String::default()), true)
        } else {
            Ok(format!("{} := ({}), ", self.field.ident, EDGEQL))
        }
    }

    pub fn add_stmt_quote(&self, to_query: bool) -> syn::Result<proc_macro2::TokenStream> {
        let f_name = self.field.ident.clone();

        let edge_ql = EDGEQL.to_string();

        let field_statement = self.build_statement()?;

        let parent_table_name = self.parent_table_name.clone().unwrap_or(String::default());

        let add_quote = if to_query {
            quote!(query.push_str(p.as_str());)
        } else {
            quote!(set_stmt.push_str(p.as_str());)
        };

        Ok(quote! {

            let mut nested_edgeql = self.#f_name.to_edgeql();
            let nested_table_name = nested_edgeql.table_name.clone();

            if #parent_table_name == nested_table_name.as_str() {
                nested_edgeql = nested_edgeql.detached();
            }

            let p = #field_statement.to_owned()
                .replace(#edge_ql, nested_edgeql.to_string().as_str());

            #add_quote
        })
    }

}

// endregion NestedQueryField