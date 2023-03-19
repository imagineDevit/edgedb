use std::convert::TryFrom;

use quote::{quote, ToTokens};
use syn::{Field, Ident};
use syn::punctuated::Iter;

use crate::constants::*;
use crate::builders::impl_builder::{FieldCat, ImplBuilderField};
use crate::queries::{check_duplicate_parameter_labels, QueryField};
use crate::tags::{build_tags_from_field, Tagged};
use crate::tags::field_tag::{FieldTag, FieldTagBuilder};
use crate::tags::filter_tag::{FilterTagBuilder, FilterTags};
use crate::tags::TagBuilders::{Field, Filter};
use crate::utils::attributes_utils::{has_any_attribute, has_attribute, has_none_attribute};
use crate::utils::derive_utils::{nested_element_shape, nested_element_value};
use crate::utils::type_utils::get_scalar;

// region QueryFilter
#[derive(Debug, Clone)]
pub struct QueryFilter {
    pub field: QueryField,
    pub field_tag: FieldTag,
    pub filter_tag: FilterTags,
}

impl QueryFilter {
    pub fn build_statement(&self, table_name: impl Into<String>) -> syn::Result<String> {
        let filter_operator = self.filter_tag.operator();
        let symbol = filter_operator.statement();
        let ty = get_scalar(&self.field.ty)?;
        let column_name = self.field_tag.column_name.clone();
        let param = self.field_tag.parameter_label.clone();
        let table_name = table_name.into();
        let wrapped_field_name = if let Some(wfn) = &self.filter_tag.wrapper_fn() {
            format!("{wfn}({table_name}.{column_name})")
        } else {
            format!("{table_name}.{column_name}")
        };

        let conjunctive = self.filter_tag.conjunctive();

        if filter_operator.check_exist() {
            Ok(format!("{conjunctive}{SPACE}{symbol}{SPACE}{table_name}.{column_name}"))
        } else {
            Ok(format!("{conjunctive}{SPACE}{wrapped_field_name}{SPACE}{symbol}{SPACE}({SELECT}{SPACE}{ty}${param})"))
        }
    }

    pub fn push_to_query_quote(&self, filter_stmt: String,  from_filters: bool) -> proc_macro2::TokenStream {

        if from_filters {
            let __tablename__ = __TABLENAME__;
            quote! {
                query.push_str(#filter_stmt.replace(#__tablename__, table_name).as_str());
            }
        } else {
            quote! {
                 query.push_str(#filter_stmt);
            }

        }
    }
}

impl TryFrom<(&Field, bool)> for QueryFilter {
    type Error = syn::Error;

    fn try_from((field, first): (&Field, bool)) -> Result<Self, Self::Error> {
        let mut field_tag_builder = Field(FieldTagBuilder::default());
        let mut filter_tag_builder = Filter(FilterTagBuilder::default());

        build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut field_tag_builder, &mut filter_tag_builder])?;

        let field_tag_builder: FieldTagBuilder = field_tag_builder.into();
        let filter_tag_builder: FilterTagBuilder = filter_tag_builder.into();

        Ok(Self {
            field: QueryField::try_from((field, vec![FIELD, FILTER, AND_FILTER, OR_FILTER]))?,
            field_tag: field_tag_builder.build(field)?,
            filter_tag: filter_tag_builder.build(field, first)?,
        })
    }
}

// endregion QueryFilter

// region QueryFilters
#[derive(Debug, Clone)]
pub struct QueryFilters {
    pub field: QueryField,
}

impl TryFrom<&Field> for QueryFilters {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        Ok(Self {
            field: QueryField::try_from((field, vec![FILTERS]))?,
        })
    }
}

// endregion QueryFilters

// region FilterStatement
#[derive(Debug, Clone)]
pub enum FilterStatement {
    NoFilter,
    ManyFilter(Vec<QueryFilter>),
    OneFilters(QueryFilters),
}

impl FilterStatement {

    pub fn get_parameter_labels(&self) -> Vec<(Ident, String)> {
        if let FilterStatement::ManyFilter(filters) = self {
            filters.iter()
                .map(|filter| (filter.field.ident.clone(), filter.field_tag.parameter_label.clone()))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn to_impl_builder_field(&self) -> Vec<ImplBuilderField> {
       match self {
            FilterStatement::NoFilter => vec![],
            FilterStatement::ManyFilter(filters) => {
                filters.iter().map(|f| ImplBuilderField {
                    field: f.field.clone(),
                    field_cat: FieldCat::Simple(f.field_tag.parameter_label.clone()),
                }).collect()
            }
            FilterStatement::OneFilters(filter) => vec![
                ImplBuilderField {
                    field: filter.field.clone(),
                    field_cat: FieldCat::Nested,
                }
            ]
        }
    }

    pub fn edgeql_statements(&self, table_name: impl Into<String>, from_filters: bool) -> Vec<proc_macro2::TokenStream> {

        let table_name = table_name.into();

        match self {
            FilterStatement::NoFilter => vec![],
            FilterStatement::ManyFilter(filters) => {

                let filter_q = format!(" {FILTER}");
                let query_filters = filters.iter()
                    .map(|filter| {
                        Ok(filter.push_to_query_quote(filter.build_statement(table_name.clone())?, from_filters))
                    })
                    .map(|r: syn::Result<_>| r.unwrap_or_else(|e| e.to_compile_error()));

               vec![quote! {
                    query.push_str(#filter_q);
                    #(#query_filters)*
                }]
            }
            FilterStatement::OneFilters(filters) => {
                let f_name = filters.field.ident.clone();
                vec![
                    quote!{
                        let filter_q = self.#f_name.to_edgeql(#table_name);
                        query.push_str(filter_q.as_str());
                    }
                ]
            }
        }
    }

    pub fn struct_field_quote(&self) -> proc_macro2::TokenStream {
        match self {
            FilterStatement::NoFilter => {
                quote!()
            }
            FilterStatement::ManyFilter(filters) => {
                let fields = filters.iter().map(|f| f.field.struct_field_quote());
                quote! {
                   #(#fields)*
                }
            }
            FilterStatement::OneFilters(filters) => {
                filters.field.struct_field_quote()
            }
        }

    }

    pub fn shape_quote(&self) -> proc_macro2::TokenStream {
        match self {
            FilterStatement::NoFilter => quote!(),
            FilterStatement::ManyFilter(filters) => {
                let shapes = filters.iter()
                    .map(|f| f.field.field_shape_quote(f.field_tag.parameter_label.clone()));

                quote! {
                    #(#shapes)*
                }

            }
            FilterStatement::OneFilters(filters) => nested_element_shape(filters.field.ident.clone())
        }
    }

    pub fn value_quote(&self) -> proc_macro2::TokenStream {
        match self {
            FilterStatement::NoFilter => quote!(),
            FilterStatement::ManyFilter(filters) => {
                let shapes = filters.iter()
                    .map(|f| f.field.field_value_quote());

                quote! {
                    #(#shapes)*
                }

            }
            FilterStatement::OneFilters(filters) => nested_element_value(filters.field.ident.clone())
        }
    }

    pub fn check_duplicate_parameter_labels(&self) -> syn::Result<()> {
        check_duplicate_parameter_labels(self.get_parameter_labels())
    }
}

// endregion FilterStatement

// region functions
#[derive(Eq, PartialEq)]
pub enum FilterRequiredQuery {
    Select, Update, Delete, Other
}

pub fn filters_from_fields(field_iter: Iter<Field>, exclude_tags: Vec<&str>, query: FilterRequiredQuery, error_msg: &str) -> syn::Result<FilterStatement> {

    let mut stmt = FilterStatement::NoFilter;

    match query {
        FilterRequiredQuery::Other => {}
        _ =>{
            let filters_fields: Vec<&Field> = field_iter.clone()
                .filter(|f| has_attribute(f, FILTERS))
                .collect::<Vec<&Field>>();

            match filters_fields.len() {
                0 => {}
                1 => {
                    stmt = FilterStatement::OneFilters(QueryFilters::try_from(filters_fields[0])?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(filters_fields[1].to_token_stream(), ONLY_ONE_FILTERS_TAG_EXPECTED));
                }
            }
        }
    }


    let filters = get_query_filters(field_iter, exclude_tags, query, error_msg, &mut stmt)?;

    if !filters.is_empty() {
        stmt = FilterStatement::ManyFilter(filters);
    }

    Ok(stmt)
}

pub fn get_query_filters(field_iter: Iter<Field>, exclude_tags: Vec<&str>, query: FilterRequiredQuery, error_msg: &str, stmt: &mut FilterStatement) -> syn::Result<Vec<QueryFilter>> {
    let mut filters: Vec<QueryFilter> = vec![];

    for field in field_iter {
        if !field.attrs.is_empty() && has_none_attribute(field, exclude_tags.clone()) && !has_attribute(field, FILTERS) {
            if query == FilterRequiredQuery::Update && field.attrs.len() == 1 && has_attribute(field, FIELD) {
                continue
            } else if has_any_attribute(field, vec![FILTER, AND_FILTER, OR_FILTER]) {

                if let FilterStatement::OneFilters(_) = stmt {
                    return Err(syn::Error::new_spanned(field, EITHER_ONE_FILTERS_OR_FILTER_TAG_EXPECTED));
                }

                filters.push(QueryFilter::try_from((field, filters.is_empty()))?);
            } else {
                return Err(syn::Error::new_spanned(field, error_msg));
            }
        }
    }
    Ok(filters)
}
// endregion functions
