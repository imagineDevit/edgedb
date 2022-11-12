use crate::constants::{TARGET_COLUMN, CONJUNCTIVE, SCALAR_TYPE, META, ENUM, FILTER, INF_SIGN, LIMIT, MODULE, NAME, OPERATOR, OPTIONS, ORDER_BY, ORDER_DIR, RESULT, SELECT, SUP_SIGN, TABLE, BACKLINK, TYPE, VALUE, TARGET_TABLE, SOURCE_TABLE, FILTERS, COLUMN_NAME, WRAPPER_FN, FIELD, DEFAULT_VALUE, SCALAR, ASSIGNMENT};
use crate::utils::field_utils::get_field_ident;
use crate::utils::path_utils::path_ident_equals;
use crate::utils::type_utils::is_type_name;
use proc_macro2::{Ident, Span};
use std::collections::HashMap;
use quote::ToTokens;

use syn::{Attribute, Field, Meta, MetaNameValue, NestedMeta, Type, Variant};

pub struct EdgeDbMeta {
    pub module: Option<String>,
    pub table: Option<String>,
}

pub struct EdgeDbType {
    pub exist: bool,
    pub ty: Option<String>,
    pub name: Option<String>,
    pub module: Option<String>,
}

pub struct EdgeEnumValue {
    pub value: Option<String>,
}

#[derive(Clone)]
pub struct OrderOption {
    pub order_by: String,
    pub order_dir: Option<String>,
}

#[derive(Clone, Default)]
pub struct QueryResult {
    pub result_type: Option<String>,
    pub order: Option<OrderOption>,
    pub limit: Option<u32>,
}

pub enum Operator {
    Exists,
    NotExists,
    Is,
    IsNot,
    Like,
    ILike,
    In,
    NotIn,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqual,
    LesserThanOrEqual,
}

pub enum Conjunctive {
    And,
    Or,
}

pub struct Filter {
    pub operator: Operator,
    pub conjunctive: Option<Conjunctive>,
    pub column_name: Option<String>,
    pub wrapper_fn: Option<String>
}

pub struct Filters;

pub struct Options;

pub struct QueryShape {
    pub module: String,
    pub source_table: String,
    pub target_table: String,
    pub target_column: String,
    pub result: String,
}

pub struct ResultField {
    pub column_name: Option<String>,
    pub wrapper_fn: Option<String>,
    pub default_value: Option<String>
}

pub enum SetOption {
    Assign,
    Concat,
    Push,
}

pub struct SetField {
    pub column_name: Option<String>,
    pub option: SetOption
}

// impls
macro_rules! explore_field_attrs (
    (field <- $field: expr, derive_name <- $dn:expr, map <- $map: expr) => {{

        let map: HashMap<&str, Option<String>> = $map;
        let field: &Field = $field;
        let d_name: &str = $dn;

        let mut map_cloned = map.clone();
        let mut exist = false;

        for att in &field.attrs {
            match att.parse_meta() {
                Ok(ref mut meta) => {
                    match meta {
                        Meta::Path(path) => {
                            if let Some((true, _)) = path_ident_equals(path, d_name) {
                                exist = true;
                            }
                        }
                        Meta::List(syn::MetaList { ref path, ref mut nested, .. }) => {
                            if let Some((true, _)) = path_ident_equals(path, d_name) {
                                exist = true;
                                for nm in nested.clone() {
                                    if let NestedMeta::Meta(syn::Meta::NameValue(MetaNameValue {
                                                                                     ref path,
                                                                                     lit: syn::Lit::Str(value),
                                                                                     ..
                                                                                 })) = nm
                                    {
                                        for (k, _) in map.iter().clone() {
                                            if let Some((true, _)) = path_ident_equals(path, k) {
                                                map_cloned.insert(k, Some(value.value()));
                                            }
                                        }
                                    }
                                }
                            }

                        }
                        Meta::NameValue(_) => {
                        }
                    }
                }
                Err(_) => {}
            }

            if map.clone().values().into_iter().all(|o| o.is_some()) {
                break;
            }
        }

        (map_cloned, exist)
    }};
    (field <- $field: expr, derive_name <- $dn:expr, map <- $map: expr, number <- $number: expr) => {{

        let map: HashMap<&str, Option<String>> = $map;
        let field: &Field = $field;
        let d_name: &str = $dn;

        let mut map_cloned = map.clone();
        let mut exist = false;

        for att in &field.attrs {
            if let Ok(syn::Meta::List(syn::MetaList {
                ref path,
                ref mut nested,
                ..
            })) = att.parse_meta()
            {
                if let Some((true, _)) = path_ident_equals(path, d_name) {
                    exist = true;
                    for nm in nested.clone() {
                        if let NestedMeta::Meta(syn::Meta::NameValue(MetaNameValue {
                            ref path,
                            lit,
                            ..
                        })) = nm
                        {

                            let val = match lit {
                                syn::Lit::Int(value) => {
                                    Some(value.base10_digits().to_string())
                                },
                                syn::Lit::Str(value) => {
                                    Some(value.value())
                                },
                                _ => None
                            };

                            if val.is_some() {
                                for (k, _) in map.iter().clone() {
                                    if let Some((true, _)) = path_ident_equals(path, k) {
                                        map_cloned.insert(k, val.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if map.clone().values().into_iter().all(|o| o.is_some()) {
                break;
            }
        }

        (map_cloned, exist)
    }};
    (field <- $field: expr, derive_name <- $dn:expr) =>{{
        let field: &Field = $field;
        let d_name: &str = $dn;
        let mut found = false;
        for attr in &field.attrs {
            if let Ok(Meta::Path(ref path)) = attr.parse_meta() {
                if let Some((true, _)) = path_ident_equals(path, d_name) {
                   found = true;
                   break;
                }
            }
        }
        found
    }}
);

impl ToString for Conjunctive {
    fn to_string(&self) -> String {
        match self {
            Conjunctive::And => "and".to_owned(),
            Conjunctive::Or => "or".to_owned()
        }
    }
}

impl EdgeDbMeta {
    pub fn is_valid(&self) -> bool {
        self.table.is_some()
    }

    pub fn value(&self) -> Option<String> {
        if self.is_valid() {
            self.module
                .clone()
                .or_else(|| Some("default".to_string()))
                .map(|module| {
                    self.table
                        .clone()
                        .map(|table| format!("{}::{}", module, table))
                        .unwrap()
                })
        } else {
            None
        }
    }

    pub fn from_field(field: &Field) -> Self {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(MODULE, None);
        map.insert(TABLE, None);

        let (attrs_values, _) = explore_field_attrs!(
             field <- field,
             derive_name <- META,
             map <- map
         );

        let module = attrs_values.get(MODULE).unwrap().clone();
        let table = attrs_values.get(TABLE).unwrap().clone();

        Self { module, table }
    }
}

impl QueryResult {
    pub fn has_result_type(&self) -> bool {
        self.result_type.is_some()
    }

    pub fn from_field(field: &Field) -> syn::Result<Self> {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(TYPE, None);
        map.insert(ORDER_BY, None);
        map.insert(ORDER_DIR, None);

        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- RESULT,
            map <- map
        );

        let result = map_cloned.get(TYPE).unwrap().clone();
        let order_by = map_cloned.get(ORDER_BY).unwrap().clone();
        let order_dir = map_cloned.get(ORDER_DIR).unwrap().clone();

        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(LIMIT, None);
        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- RESULT,
            map <- map,
            number <- ()
        );

        let limit = map_cloned.get(LIMIT).unwrap().clone();

        let attrs = field.attrs.iter().filter(|att| att.path.is_ident(RESULT)).collect::<Vec<&Attribute>>();

        let order = if let Some(o) = order_by {
            if let Some(dir) = order_dir.clone() {
                if dir != "asc".to_owned() && dir != "desc".to_owned() {
                    return Err(
                        syn::Error::new_spanned(
                            attrs[0].clone().tokens,
                            "Only value 'asc' or 'desc' are accepted for attribute order_dir"
                        )
                    );
                }
            }
            Some(OrderOption {
                order_by: o,
                order_dir,
            })
        } else {
            if order_dir.is_some() {
                return Err(
                    syn::Error::new_spanned(
                        attrs[0].clone().tokens,
                        "order_by is required when order_dir is specified."
                    )
                );
            }
            None
        };

        let limit = if let Some(l) = limit {
            if let Ok(ll) = l.parse::<u32>() {
                Some(ll)
            } else {
                return Err(
                    syn::Error::new_spanned(
                        attrs[0].clone().tokens,
                        "Limit attribute must be a number"
                    )
                );
            }
        } else {
            None
        };
        Ok(Self {
            result_type: result,
            order,
            limit,
        })
    }

    pub fn to_ident(&self, span: Span) -> Ident {
        self.result_type
            .clone()
            .or_else(|| Some("BasicResult".to_string()))
            .map(|s| Ident::new(s.as_str(), span))
            .unwrap()
    }

    pub fn complete_select_query(&self, table_name: String) -> String {
        let mut ql = String::default();

        if let Some(order) = self.order.clone() {
            ql.push_str(format!(" order by {}.{}", table_name, order.order_by).as_str());

            if let Some(dir) = order.order_dir {
                ql.push_str(format!(" {}", dir).as_str());
            }
        }

        if let Some(limit) = self.limit.clone() {
            ql.push_str(format!(" limit {}", limit).as_str());
        }

        ql
    }
}

impl EdgeDbType {
    pub fn is_valid(&self) -> bool {
        self.ty.is_some()
            && (if self.is_enum() {
            self.name.is_some()
        } else {
            !self.ty.clone().unwrap().is_empty() && self.name.is_none() && self.module.is_none()
        })
    }

    pub fn is_enum(&self) -> bool {
        self.ty.clone().unwrap() == ENUM
    }

    pub fn value(&self) -> Option<String> {
        let format_type = |i: String| {
            let mut s = i.clone();
            if !s.trim().starts_with(INF_SIGN) {
                s = format!("{}{}", INF_SIGN, s);
            }
            if !s.trim().ends_with(SUP_SIGN) {
                s = format!("{}{}", s, SUP_SIGN);
            }
            s
        };

        if self.is_valid() {
            if self.is_enum() {
                self.module
                    .clone()
                    .or_else(|| Some("default".to_string()))
                    .map(|module| {
                        self.name
                            .clone()
                            .map(|name| format!("{}::{}", module, name))
                            .unwrap()
                    })
                    .map(|s| format_type(s))
            } else {
                self.ty.clone().map(|s| format_type(s))
            }
        } else {
            None
        }
    }

    pub fn from_field(field: &Field) -> Self {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(TYPE, None);
        map.insert(NAME, None);
        map.insert(MODULE, None);

        let (map_cloned, exist) = explore_field_attrs!(
            field <- field,
            derive_name <- SCALAR,
            map <- map
        );

        let ty = map_cloned.get(TYPE).unwrap().clone();

        let name = map_cloned.get(NAME).unwrap().clone();
        let module = map_cloned.get(MODULE).unwrap().clone();

        Self {
            exist,
            ty,
            name,
            module,
        }
    }

    pub fn build_field_assignment(field: &Field) -> syn::Result<String> {
        let ty = EdgeDbType::get_type(field)?;

        Ok(format!(
            "{field_name} := ({select} {edge_type}${field_name}), ",
            select = SELECT,
            edge_type = ty,
            field_name = get_field_ident(field)
        ))
    }

    pub fn get_type(field: &Field) -> syn::Result<String> {
        let db_type = EdgeDbType::from_field(field);

        if db_type.exist {
            if db_type.ty.is_none() {
                return Err(
                    syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        r#"
                    Please specify the scalar type by adding
                    #[scalar(type="")]
                "#));
            } else if db_type.is_enum() && db_type.name.is_none() {
                return Err(
                    syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        r#"
                    If the scalar type is enum you must specify its name and its module as follow :
                    #[scalar(type="enum", name="...", module="")]
                "#));
            }
        };

        if let Some(value) = db_type.value() {
           Ok(value)
        } else {
            Ok(SCALAR_TYPE.to_string())
        }
    }
}

impl EdgeEnumValue {
    pub fn from_variant(variant: &Variant) -> syn::Result<Self> {
        let mut exist = false;
        let mut value: Option<String> = None;

        for att in &variant.attrs {
            match att.parse_meta() {
                Ok(ref mut m) => match m {
                    Meta::Path(path) => {
                        if let Some((true, _)) = path_ident_equals(path, VALUE) {
                            exist = true;
                        }
                    }
                    Meta::List(syn::MetaList {
                                   ref path,
                                   ref mut nested,
                                   ..
                               }) => {
                        if let Some((true, _)) = path_ident_equals(path, VALUE) {
                            exist = true;
                            for ne in nested.iter() {
                                if let NestedMeta::Lit(syn::Lit::Str(s)) = ne {
                                    let val = s.value();
                                    if !val.is_empty() {
                                        value = Some(val);
                                    }
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    Meta::NameValue(_) => {}
                },
                Err(_) => {}
            }
        }

        if exist && value.is_none()  {
            return Err(
                syn::Error::new_spanned(
                    variant.attrs[0].clone().into_token_stream(),
                    format!(r#" Please specify a non empty value #[value("...")] for enum variant {} "#,
                            variant.ident.to_string())
                )
            );
        }

        Ok(Self { value })
    }
}

impl Operator {
    fn from_str(field: &Field, s: String) -> syn::Result<Operator> {

        let ty = &field.ty;

        let check_not_accepted_type = |_: &Type| -> syn::Result<()>{
            if is_type_name(&ty, "()") {
                return Err(
                    syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        format!("Type () is not accepted for Filter type {}", s))
                );
            }

            Ok(())
        };

        let check_only_accepted_type = |ty: &Type, op: &str, tty: &str| -> syn::Result<()> {
            if !is_type_name(&ty, tty) {
                return Err(
                    syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        format!("Filter type {} is only accepted for field of type {}", op, tty))
                );
            }
            Ok(())
        };

        match s.to_lowercase().as_str() {
            "exists" => {
                check_only_accepted_type(ty, "Exists", "()")?;
                Ok(Operator::Exists)
            }
            "notexists" | "!exists" => {
                check_only_accepted_type(ty, "NotExists", "()")?;
                Ok(Operator::NotExists)
            }
            "is" | "="=> {
                check_not_accepted_type(ty)?;
                Ok(Operator::Is)
            }
            "isnot" | "!=" => {
                check_not_accepted_type(ty)?;
                Ok(Operator::IsNot)
            }
            "like" => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, "Like", "String")?;
                Ok(Operator::Like)
            }
            "ilike" => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, "ILike", "String")?;
                Ok(Operator::ILike)
            }
            "in" => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, "In", "Vec")?;
                Ok(Operator::In)
            }
            "notin" => {
                check_not_accepted_type(ty)?;
                check_only_accepted_type(ty, "NotIn", "Vec")?;
                Ok(Operator::NotIn)
            }
            "greaterthan" | ">" => {
                check_not_accepted_type(ty)?;
               Ok(Operator::GreaterThan)
            }
            "greaterthanorequal" | ">=" => {
                check_not_accepted_type(ty)?;
                Ok(Operator::GreaterThanOrEqual)
            }
            "lesserthan" | "<" => {
                check_not_accepted_type(ty)?;
                Ok(Operator::LesserThan)
            }
            "lesserthanorequal" | "<=" => {
                check_not_accepted_type(ty)?;
                Ok(Operator::LesserThanOrEqual)
            }
            _ => {
                return Err(
                    syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        format!(r#"
                    {} is not a valid filter type.
                    Only filter types :
                      - Exists
                      - Is
                      - IsNot
                      - Like
                      - ILike
                      - In
                      - GreaterThan
                      - GreaterThanOrEqual
                      - LesserThan
                      - LesserThanOrEqual
                    are accepted
                "#, s)));
            }
        }
    }

    pub fn build(&self, table_name: String, field: &Field, column_name: Option<String>, wrapper_fn: Option<String>) -> syn::Result<String> {
        let ty = EdgeDbType::get_type(field)?;

        let mut is_exists = false;

        let symbol = match self {
            Operator::Is => "=",
            Operator::IsNot => "!=",
            Operator::Like => "like",
            Operator::ILike => "ilike",
            Operator::In => "in",
            Operator::NotIn => "not in",
            Operator::GreaterThan => ">",
            Operator::LesserThan => "<",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LesserThanOrEqual => "<=",
            Operator::Exists => {
                is_exists = true;
                "exists"
            }
            Operator::NotExists => {
                is_exists = true;
                "not exists"
            }
        };

        let field_name = get_field_ident(field).to_string();

        let column_name = column_name.or_else(|| Some(field_name.clone())).unwrap();

        let wrapped_field_name = if let Some(wfn) = wrapper_fn {
            format!("{wfn}({table_name}.{column_name})")
        } else {
            format!("{table_name}.{column_name}")
        };

        if is_exists {
            Ok(format!(
                " {symbol} {table}.{column_name}",
                symbol = symbol,
                table = table_name,
                column_name = column_name
            ))
        } else {
            Ok(format!(
                " {wrapped_field_name} {symbol} ({select} {edge_type}${field_name})",
                symbol = symbol,
                select = SELECT,
                edge_type = ty,
                wrapped_field_name = wrapped_field_name,
                field_name = field_name
            ))
        }
    }
}

impl Filter {
    pub fn from_field(field: &Field, index: usize) -> syn::Result<Self> {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(OPERATOR, None);
        map.insert(CONJUNCTIVE, None);
        map.insert(COLUMN_NAME, None);
        map.insert(WRAPPER_FN, None);

        let (map_cloned, exist) = explore_field_attrs!(
            field <- field,
            derive_name <- FILTER,
            map <- map
        );

        if !exist {
            return Err(
                syn::Error::new_spanned(
                    field.into_token_stream(),
                    format!("Please please add #[filter(operator=\"...\",)] attribute to field {}",
                            get_field_ident(field)))
            );
        }

        let operator = map_cloned.get(OPERATOR).unwrap().clone();

        let operator = if let Some(op) = operator {
            Operator::from_str(&field, op)?
        } else {
            return Err(
                syn::Error::new_spanned(
                    field.attrs[0].clone().into_token_stream(),
                    format!(r#"
                    Please please add operator option to filter attribute
                    #[filter(operator=\"...\")
                "#)));
        };

        let conjunctive = map_cloned.get(CONJUNCTIVE).unwrap().clone();
        let column_name = map_cloned.get(COLUMN_NAME).unwrap().clone();
        let wrapper_fn = map_cloned.get(WRAPPER_FN).unwrap().clone();

        let res_conjunctive = if let Some(c) = conjunctive {
            match c.as_str() {
                "Or" => Ok(Some(Conjunctive::Or)),
                "And" => Ok(Some(Conjunctive::And)),
                _ => Err(syn::Error::new_spanned(
                    field.attrs[0].clone().into_token_stream(),
                    format!(
                        r#"
                    {} is not a valid conjunctive type.
                    Only conjunctives :
                      - And
                      - Or
                    are accepted
                "#,
                        c
                    )
                ))
            }
        } else {
            if index > 0 {
                return Err(syn::Error::new_spanned(
                    field.attrs[0].clone().into_token_stream(),
                    r#"
                    Please specify conjunctive option to filter attribute
                    #[filter(operator=\"...\"), conjunctive=\"...\"]
                "#));
            }
            Ok(None)
        };

        let conjunctive = res_conjunctive?;

        Ok(Filter { operator, conjunctive, column_name, wrapper_fn })
    }

    pub fn build_filter_assignment(table_name: String, field: &Field, index: usize) -> syn::Result<String> {
        let filter = Filter::from_field(field, index)?;

        let q = filter.operator.build(table_name, field, filter.column_name, filter.wrapper_fn)?;
        if index == 0 {
            Ok(q)
        } else {
            let c = filter.conjunctive.unwrap();
            Ok(format!(" {}{}", c.to_string(), q))
        }
    }
}

impl Filters {
    pub fn from_field(field: &Field) -> Option<Self> {
        let found = explore_field_attrs!(
            field <- field,
            derive_name <- FILTERS
        );

        if found { Some(Self {}) } else { None }
    }
}

impl Options {
    pub fn from_field(field: &Field) -> Option<Self> {
        let found = explore_field_attrs!(
            field <- field,
            derive_name <- OPTIONS
        );

        if found { Some(Self {}) } else { None }
    }
}

impl QueryShape {
    pub fn from_field(field: &Field) -> syn::Result<Self> {

        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(MODULE, None);
        map.insert(SOURCE_TABLE, None);
        map.insert(TARGET_TABLE, None);
        map.insert(TARGET_COLUMN, None);
        map.insert(RESULT, None);

        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- BACKLINK,
            map <- map
        );

        let module = Self::get_value(&map_cloned, MODULE, field)?;

        let source_table = Self::get_value(&map_cloned, SOURCE_TABLE, field)?;

        let target_table = Self::get_value(&map_cloned, TARGET_TABLE, field)?;

        let target_column = Self::get_value(&map_cloned, TARGET_COLUMN, field)?;

        let result = Self::get_value(&map_cloned, RESULT, field)?;


        Ok(Self { module, source_table, target_table, target_column, result })
    }

    pub fn build_assignment(field: &Field) -> syn::Result<(String, String)> {
        let t = QueryShape::from_field(field)?;
        let source = t.source_table;
        let column = t.target_column;
        let module = t.module;
        let table = t.target_table;

        Ok((format!("select {module}::{source}.<{column}[is {module}::{table}]"), t.result.clone()))
    }

    fn get_value(map_cloned: &HashMap<&str, Option<String>>, name: &str, field: &Field) -> syn::Result<String> {

        let field_name = get_field_ident(field).to_string();

        let result = if let Some(t) = map_cloned.get(name).unwrap().clone() {
            Self::required(field, t, name)?
        } else {
            return Err(syn::Error::new_spanned(
                field.into_token_stream(),
                format!(r#"
                Please add {} attribute to query_shape macro on field {}
            "#, name, field_name)
            ));
        };

        Ok(result)
    }

    fn required(field: &Field, value: String, name: &str) -> syn::Result<String> {
        if value.clone().is_empty() {
            return Err(syn::Error::new_spanned(
                field.into_token_stream(),
                format!("Non empty value required for {} attribute", name)
            ));
        }
        Ok(value)
    }
}

impl ResultField {
    pub fn from_field(field: &Field) -> syn::Result<Self> {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(COLUMN_NAME, None);
        map.insert(WRAPPER_FN, None);
        map.insert(DEFAULT_VALUE, None);

        let (map_cloned, found) = explore_field_attrs!(
            field <- field,
            derive_name <- FIELD,
            map <- map
        );

        let column_name = map_cloned.get(COLUMN_NAME).unwrap().clone();
        let wrapper_fn = map_cloned.get(WRAPPER_FN).unwrap().clone();
        let default_value = map_cloned.get(DEFAULT_VALUE).unwrap().clone();

        let all_nones = vec![column_name.clone(), wrapper_fn.clone(), default_value.clone()].iter().all(|o| o.is_none());

        if found && all_nones {
            return Err(syn::Error::new_spanned(
               field.attrs[0].clone().into_token_stream(),
               "#[field] must have at least column_name or wrapper_fn attribute"
            ));
        }

        if wrapper_fn.clone().is_none()  && default_value.is_some() {
            if let Some(c) = column_name.clone() {
                let f_name = get_field_ident(field).to_string();
                if c == f_name {
                    return Err(
                        syn::Error::new_spanned(
                            field.attrs[0].clone().into_token_stream(),
                            format!("default_value cannot be applied to the field {}", f_name)));
                }
            }
        }

        Ok(Self {
            column_name,
            wrapper_fn,
            default_value
        })
    }

    pub fn build_statement(field: &Field) -> syn::Result<String> {
        let result_field = Self::from_field(field)?;

        let f_name = get_field_ident(field).to_string();

        let scalar = SCALAR_TYPE;

        let mut s = match (result_field.column_name, result_field.wrapper_fn) {
            (Some(column), None) => {
                if column != f_name {
                    format!("{} := .{}", f_name, column)
                } else {
                    f_name
                }
            }

            (None, Some(wrapper_fn)) => {
                format!("{name} := (select {scalar}{func}(.{name}))",
                        scalar = scalar,
                        name = f_name,
                        func = wrapper_fn)
            }

            (Some(column), Some(wrapper_fn)) => {
                format!("{name} := (select {scalar}{func}(.{column}))",
                        name = f_name,
                        scalar = scalar,
                        func = wrapper_fn,
                        column = column)
            }

            (None, None) => {
                f_name
            }
        };

        if let Some(v) = result_field.default_value {
            s = format!("{s} ?? (select {scalar}'{v}')", s = s, scalar = scalar, v = v);
        }

        Ok(s)
    }
}

impl SetField {
    pub fn from_field(field: &Field) -> syn::Result<Self> {

        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(COLUMN_NAME, None);
        map.insert(ASSIGNMENT, None);

        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- FIELD,
            map <- map
        );

        let column_name = map_cloned.get(COLUMN_NAME).unwrap().clone();
        let res_option: syn::Result<SetOption> = if let Some(assignment)  = map_cloned.get(ASSIGNMENT).unwrap().clone() {
           match assignment.to_lowercase().as_str() {
               "concat" => Ok(SetOption::Concat),
               "assign" => Ok(SetOption::Assign),
               "push" => Ok(SetOption::Push),
               _ => {
                   return Err(syn::Error::new_spanned(
                       field.attrs[0].clone().into_token_stream(),
                       "Only 'Concat', 'Assign' or 'Push' are allowed for assignment option"
                   ));
               }
           }
        } else {
            Ok(SetOption::Assign)
        };

        let option = res_option?;

        Ok(Self {
            column_name,
            option,
        })
    }

    pub fn build_field_assignment(field: &Field) -> syn::Result<String> {
        let ty = EdgeDbType::get_type(field)?;
        let set_field = SetField::from_field(&field)?;

        let fname = get_field_ident(field);

        match set_field.option {
            SetOption::Assign => Ok(format!(
                "{column_name} := ({select} {edge_type}${field_name}), ",
                column_name = set_field.column_name.unwrap_or(fname.to_string()),
                select = SELECT,
                edge_type = ty,
                field_name = fname
            )),
            SetOption::Concat => Ok(format!(
                "{column_name} := .{column_name} ++ ({select} {edge_type}${field_name}), ",
                column_name = set_field.column_name.unwrap_or(fname.to_string()),
                select = SELECT,
                edge_type = ty,
                field_name = fname
            )),
            SetOption::Push => {
                if !is_type_name(&field.ty, "vec") {
                    return Err(syn::Error::new_spanned(
                        field.attrs[0].clone().into_token_stream(),
                        "Push option is only allowed for vec type"
                    ));
                }
                Ok(format!(
                    "{column_name} += ({select} {edge_type}${field_name}), ",
                    column_name = set_field.column_name.unwrap_or(fname.to_string()),
                    select = SELECT,
                    edge_type = ty,
                    field_name = fname
                ))
            }
        }

    }
}