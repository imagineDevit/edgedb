
use quote::quote;
use syn:: Ident;
use syn::ItemStruct;
use syn::parse::{Parse, ParseStream};
use crate::constants::{AND_FILTER, EXPECTED_AT_LEAST_ONE_SET_FIELD,  FILTER, FILTERS, INVALID_UPDATE_TAG, NESTED_QUERY, OR_FILTER, SET, SETS, UPDATE};
use crate::meta_data::{TableInfo, try_get_meta};
use crate::builders::impl_builder::QueryImplBuilder;
use crate::queries::Query;
use crate::statements::filters::{FilterRequiredQuery, filters_from_fields, FilterStatement};
use crate::statements::set::{sets_from_fields, UpdateSetStatement};

pub struct UpdateQuery {
    pub ident: Ident,
    pub meta: Option<TableInfo>,
    pub filter_statement: FilterStatement,
    pub set_statement: UpdateSetStatement,
}

impl UpdateQuery {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            meta: None,
            filter_statement: FilterStatement::NoFilter,
            set_statement: UpdateSetStatement::None,
        }
    }

    pub fn with_meta(&mut self, meta: TableInfo) -> &mut Self {
        self.meta = Some(meta);
        self
    }
}

impl Query for UpdateQuery {
    fn get_param_labels(&self) -> Vec<(Ident, String)> {
        let mut p = self.filter_statement.get_parameter_labels();
        let v = self.set_statement.get_parameter_labels();
        p.extend(v);

        p
    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder> {
        let meta = try_get_meta(&self.ident.clone(), || self.meta.clone())?;

        let table_name = meta.table_name();

        let mut fields = self.filter_statement.to_impl_builder_field();


        let set_fields = self.set_statement.to_impl_builder_field();

        fields.extend(set_fields);

        let mut edgeql_statements = self.filter_statement.edgeql_statements(table_name.clone(), false);

        let add_set = self.set_statement.add_set_statement_quote();

        edgeql_statements.push(quote! {
            query.push_str(" ");
            #add_set
            query.push_str(&set_stmt);
        });

        Ok(QueryImplBuilder {
            struct_name: self.ident.clone(),
            fields,
            init_edgeql: format!("{UPDATE} {table_name}"),
            static_const_check_statements: vec![],
            edgeql_statements,
        })
    }
}

impl Parse for UpdateQuery {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let strukt = input.parse::<ItemStruct>()?;

        let mut query = UpdateQuery::new(strukt.ident.clone());

        let field_iter = strukt.fields.iter();

        query.filter_statement = filters_from_fields(field_iter.clone(), vec![SET, SETS, NESTED_QUERY], FilterRequiredQuery::Update, INVALID_UPDATE_TAG)?;

        query.set_statement = sets_from_fields(field_iter, vec![FILTER, FILTERS, AND_FILTER, OR_FILTER], false,INVALID_UPDATE_TAG)?;

        if let UpdateSetStatement::None = query.set_statement {
            return Err(syn::Error::new(
                strukt.ident.span(),
                EXPECTED_AT_LEAST_ONE_SET_FIELD,
            ));
        }

        query.check_duplicate_parameter_labels()?;

        Ok(query)
    }
}


