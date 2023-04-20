use edgedb_query::QueryType;
use syn::{Ident, ItemStruct};
use syn::parse::{Parse, ParseStream};

use crate::{meta_data::{TableInfo, try_get_meta}, queries::Query};
use crate::builders::impl_builder::QueryImplBuilder;
use crate::constants::*;
use crate::statements::filters::{FilterRequiredQuery, filters_from_fields, FilterStatement, set_table_name};

#[derive(Debug, Clone)]
pub struct DeleteQuery {
    pub ident: Ident,
    pub meta: Option<TableInfo>,
    pub filter_statement: FilterStatement,
}

impl DeleteQuery {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            meta: None,
            filter_statement: FilterStatement::NoFilter,
        }
    }

    pub fn with_meta(&mut self, meta: TableInfo) -> &mut Self {
        self.meta = Some(meta.clone());
        set_table_name(&mut self.filter_statement, meta.table_name());
        self
    }
}

impl Query for DeleteQuery {
    fn get_param_labels(&self) -> Vec<(Ident, String)> {
        self.filter_statement.get_parameter_labels()
    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder> {
        let meta = try_get_meta(&self.ident, || self.meta.clone())?;

        let table_name = meta.table_name();

        let fields =  self.filter_statement.to_impl_builder_field();

        let mut edgeql_statements = vec![];

        edgeql_statements.extend(self.filter_statement.edgeql_statements(table_name.clone(), false));

        Ok(QueryImplBuilder {
            struct_name: self.ident.clone(),
            table_name: Some(table_name.clone()),
            fields,
            query_type: QueryType::Delete,
            static_const_check_statements: vec![],
            edgeql_statements,
            has_result: false
        })
    }
}

impl Parse for DeleteQuery {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        // region create new query
        let strukt = input.parse::<ItemStruct>()?;

        let mut query = DeleteQuery::new(strukt.ident.clone());

        let field_iter = strukt.fields.iter();
        // endregion create new query

        // region add filters statements
        query.filter_statement = filters_from_fields(field_iter, vec![], FilterRequiredQuery::Delete,INVALID_DELETE_TAG)?;
        // endregion add filters statements

        query.check_duplicate_parameter_labels()?;

        Ok(query)
    }
}
