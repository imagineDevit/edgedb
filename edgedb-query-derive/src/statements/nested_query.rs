use std::convert::TryFrom;

use syn::Field;

use crate::constants::NESTED_QUERY;
use crate::queries::QueryField;

// region NestedQueryField
#[derive(Debug, Clone)]
pub struct NestedQueryField {
    pub field: QueryField,
}

impl TryFrom<&Field> for NestedQueryField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        Ok(Self {
            field: QueryField::try_from((field, vec![NESTED_QUERY]))?,
        })
    }
}

impl NestedQueryField {
    
    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(true)
    }

    pub fn add_set_statement_quote(&self) -> proc_macro2::TokenStream {
        self.field.add_stmt_quote(false)
    }
    
}

// endregion NestedQueryField