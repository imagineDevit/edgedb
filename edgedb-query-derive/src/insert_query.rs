use std::convert::TryFrom;

use quote::{quote, ToTokens};
use syn::{Field, Ident, ItemStruct};
use syn::parse::{Parse, ParseStream};

use crate::constants::{EDGEQL, FIELD, INVALID_INSERT_TAG, NESTED_QUERY, OPTION, SCALAR_TYPE, SELECT, UNLESS_CONFLICT, VEC, INSERT};
use crate::queries::{Query, QueryField};
use crate::meta_data::{QueryMetaData, try_get_meta};
use crate::builders::impl_builder::{FieldCat, QueryImplBuilder, ImplBuilderField};
use crate::statements::nested_query::NestedQueryField;
use crate::tags::{build_tags_from_field, Tagged};
use crate::tags::field_tag::{FieldTag, FieldTagBuilder};
use crate::tags::TagBuilders::{FieldBuilder, UnlessConfictBuilder};
use crate::tags::unless_conflict_tag::{UnlessConflictTag, UnlessConflictTagBuilder};
use crate::utils::attributes_utils::has_attribute;
use crate::utils::derive_utils::format_scalar;
use crate::utils::type_utils::{get_type, is_type_name};

// region InsertQuery
#[derive(Debug, Clone)]
pub struct InsertQuery {
    pub ident: Ident,
    pub meta: Option<QueryMetaData>,
    pub statements: Vec<InsertStatement>,
    pub unless_conflict_statement: Option<UnlessConflictElseStatement>,
}

impl InsertQuery {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            meta: None,
            statements: vec![],
            unless_conflict_statement: None,
        }
    }

    pub fn with_meta(&mut self, meta: QueryMetaData) -> &mut Self {
        self.meta = Some(meta);
        self
    }
}

impl Query for InsertQuery {
    fn get_param_labels(&self) -> Vec<(Ident, String)> {
        self.statements.iter().clone()
            .filter_map(|stmt| stmt.param_field())
            .collect()
    }

    fn to_impl_builder(&self) -> syn::Result<QueryImplBuilder> {
        let meta = try_get_meta(&self.ident.clone(), || self.meta.clone())?;

        let table_name = meta.table_name();

        let has_result = meta.has_result();

        let init_edgeql = if has_result {
            format!("{SELECT} ( {INSERT} {table_name} {{")
        } else {
            format!("{INSERT} {table_name} {{")
        };

        let stmts = self.statements.iter();

        let mut fields: Vec<ImplBuilderField> = stmts.clone()
            .map(|stmt| {
                match stmt {
                    InsertStatement::SimpleField(f) => ImplBuilderField {
                        field: f.field.clone(),
                        field_cat: FieldCat::Simple(f.tag.parameter_label.clone())
                    },
                    InsertStatement::NestedQuery(f) => ImplBuilderField {
                        field: f.field.clone(),
                        field_cat: FieldCat::Nested
                    }
                }
            }).collect();

        let mut edgeql_statements = stmts.clone()
            .map(|stmt| stmt.query_statement_quote())
            .collect::<Vec<proc_macro2::TokenStream>>();

        edgeql_statements.push(quote! {  query.push_str("}"); });

        if let Some(uce) = self.unless_conflict_statement.clone() {
            fields.push(ImplBuilderField{
                field: uce.field.clone(),
                field_cat: FieldCat::Conflict
            });

            edgeql_statements.push(uce.query_statement_quote())
        }

        if has_result {
            edgeql_statements.push(quote! { query.push_str(" )"); });
        }

        edgeql_statements.push(meta.result_quote());

        let const_check_impl_conflict = if let Some(stmt) = self.unless_conflict_statement.clone() {
            stmt.static_check_bloc_quote()
        } else {
            quote!()
        };

        Ok(QueryImplBuilder {
            struct_name: self.ident.clone(),
            fields,
            init_edgeql,
            static_const_check_statements : vec![const_check_impl_conflict],
            edgeql_statements,
        })
    }
}

impl Parse for InsertQuery {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        // region create new InsertQuery
        let strukt = input.parse::<ItemStruct>()?;

        let mut query = InsertQuery::new(strukt.ident.clone());

        let field_iter = strukt.fields.iter();
        // endregion create new InsertQuery

        // region handle unless_conflict if exists
        let unless_fields: Vec<&Field> = field_iter.clone()
            .filter(|f| has_attribute(f, UNLESS_CONFLICT))
            .collect::<Vec<&Field>>();

        let mut unless_conflict_field: Option<&Field> = None;

        match unless_fields.len() {
            0 => {}
            1 => {
                unless_conflict_field = Some(unless_fields.first().unwrap());
            }
            _ => {
                return Err(syn::Error::new_spanned(unless_fields[1].to_token_stream(), "InsertQuery can only have one unless_conflict field"));
            }
        }
        // endregion handle unless_conflict if exists

        // region add insert_statements
        for field in &strukt.fields {
            if !has_attribute(field, UNLESS_CONFLICT) {
                query.statements.push(InsertStatement::try_from(field)?);
            }
        }

        let column_names = query.statements
            .iter()
            .filter_map(|stmt| stmt.column_name())
            .collect::<Vec<String>>();


        if let Some(field) = unless_conflict_field  {
            query.unless_conflict_statement = Some(UnlessConflictElseStatement::try_from((field, column_names))?);
        }

        query.check_duplicate_parameter_labels()?;

        Ok(query)
    }
}

// endregion InsertQuery

// region InsertStatement
#[derive(Debug, Clone)]
pub enum InsertStatement {
    SimpleField(InsertField),
    NestedQuery(NestedQueryField),
}

impl InsertStatement {

    pub fn param_field(&self) -> Option<(Ident, String)> {
        match self {
            InsertStatement::SimpleField(f) => Some((f.field.ident.clone(), f.tag.parameter_label.clone())),
            InsertStatement::NestedQuery(_) => None
        }
    }

    pub fn column_name(&self) -> Option<String> {
        match self {
            InsertStatement::SimpleField(f) => Some(f.tag.column_name.clone()),
            InsertStatement::NestedQuery(_) => None
        }
    }

    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        match self {
            InsertStatement::SimpleField(f) => f.query_statement_quote(),
            InsertStatement::NestedQuery(f) => f.query_statement_quote()
        }
    }
}

impl TryFrom<&Field> for InsertStatement {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if has_attribute(field, NESTED_QUERY) {
            Ok(InsertStatement::NestedQuery(NestedQueryField::try_from(field)?))
        } else if has_attribute(field, FIELD) || field.attrs.is_empty() {
            Ok(InsertStatement::SimpleField(InsertField::try_from(field)?))
        } else {
            Err(syn::Error::new_spanned(field.to_token_stream(), INVALID_INSERT_TAG))
        }
    }
}

// endregion InsertStatement

// region InsertField
#[derive(Debug, Clone)]
pub struct InsertField {
    pub field: QueryField,
    pub tag: FieldTag,
}

impl InsertField {
    pub fn build_statement(&self) -> String {
        format!(
            "{column_name} := ({select} {edge_type}${param}), ",
            select = SELECT,
            edge_type = self.tag.scalar_type,
            column_name = self.tag.column_name,
            param = self.tag.parameter_label
        )
    }

    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        let field_name = self.field.ident.clone();
        let format_scalar = format_scalar();
        let scalar_type = SCALAR_TYPE.to_string();
        let edge_ql = EDGEQL.to_string();
        let field_statement = self.build_statement();
        let field_type = self.field.ty.clone();
        let ty = get_type(&field_type);

        let scalar_quote = if is_type_name(&field_type, VEC) {
            quote! { let mut scalar = format!("<array{}>", #ty::scalar()); }
        } else {
            quote! { let mut scalar = #ty::scalar();}
        };

        let add_assignment_quote = quote! {
            #scalar_quote
            #format_scalar;
            let p = #field_statement.to_owned()
                .replace(#scalar_type, scalar.as_str())
                .replace(#edge_ql, edgeql.as_str());
            query.push_str(p.as_str());
        };

        if is_type_name(&field_type, OPTION) {
            quote! {
                if let Some(v) = &self.#field_name {
                    let edgeql = v.to_edgeql();
                    #add_assignment_quote
                }
            }
        } else {
            quote! {
                let edgeql = self.#field_name.to_edgeql();
                #add_assignment_quote
            }
        }
    }
}

impl TryFrom<&Field> for InsertField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let mut builders = FieldBuilder(FieldTagBuilder::default());

        build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut builders])?;

        let field_tag_builder: FieldTagBuilder = builders.into();

        Ok(Self {
            field: QueryField::try_from((field, vec![FIELD]))?,
            tag: field_tag_builder.build(field)?,
        })
    }
}
// endregion InsertField

// region UnlessConflictElseStatement
#[derive(Debug, Clone)]
pub struct UnlessConflictElseStatement {
    pub field: QueryField,
    pub tag: UnlessConflictTag
}

impl UnlessConflictElseStatement {
    pub fn static_check_bloc_quote(&self) -> proc_macro2::TokenStream {
        let ty = self.field.ty.clone();
        quote! {
            const _: () = {
                use std::marker::PhantomData;
                struct ImplConflict<R: edgedb_query::models::edge_query::ToEdgeQuery + Clone,T: edgedb_query::queries::conflict::Conflict<R>>((PhantomData<T>, Option<R>));
                let _ = ImplConflict((PhantomData::<#ty>, None));
            };
        }
    }

    pub fn query_statement_quote(&self) -> proc_macro2::TokenStream {
        let f_name = self.field.ident.clone();
        let on_fields_name = self.tag.on.iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>().join(",");


        quote! {
            let qn = #on_fields_name.split(",").collect::<Vec<&str>>();
            let c_q =  edgedb_query::queries::conflict::parse_conflict(&self.#f_name, qn);
            query.push_str(c_q.as_str());
        }
    }

}

impl TryFrom<(&Field, Vec<String>)> for UnlessConflictElseStatement {
    type Error = syn::Error;

    fn try_from((field, columns): (&Field, Vec<String>)) -> Result<Self, Self::Error> {

        let mut builders = UnlessConfictBuilder(UnlessConflictTagBuilder::default());

        build_tags_from_field(&Tagged::StructField(field.clone()), vec![&mut builders])?;

        let tag_builder: UnlessConflictTagBuilder = builders.into();

        Ok(Self {
            field: QueryField::try_from((field, vec![UNLESS_CONFLICT]))?,
            tag: tag_builder.build(field, columns)?,
        })
    }
}

// endregion UCEStatement