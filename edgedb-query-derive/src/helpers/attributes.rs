use crate::constants::{
    DD_SIGN, EDGEDB, ENUM, FILTER, INF_SIGN, LIMIT, MODULE, NAME, OFFSET, OPTIONS, ORDER_BY,
    ORDER_DIR, QUERY, RESULT, SELECT, SUP_SIGN, TABLE, TYPE, VALUE,
};
use crate::utils::field_utils::get_field_ident;
use crate::utils::path_utils::path_ident_equals;
use crate::utils::type_utils::is_type_name;
use proc_macro2::{Ident, Span};
use std::collections::HashMap;
use syn::{Field, Meta, MetaNameValue, NestedMeta, Variant};

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
pub struct Query {
    pub result: Option<String>,
    pub order: Option<OrderOption>,
    pub limit: Option<u32>,
}

pub enum Filter {
    Exists,
    Is,
    IsNot,
    Like,
    ILike,
    In,
    GreaterThan,
    LesserThan,
    GreaterThanOrEqual,
    LesserThanOrEqual,
}

pub struct Options {}

// impls
macro_rules! explore_field_attrs (
    (field <- $field: expr, derive_name <- $dn:expr, map <- $map: expr) => {{

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
    }}

);

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
            derive_name <- EDGEDB,
            map <- map
        );

        let module = attrs_values.get(MODULE).unwrap().clone();
        let table = attrs_values.get(TABLE).unwrap().clone();

        Self { module, table }
    }
}

impl Query {
    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    pub fn from_field(field: &Field) -> Self {
        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(RESULT, None);
        map.insert(ORDER_BY, None);
        map.insert(ORDER_DIR, None);

        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- QUERY,
            map <- map
        );

        let result = map_cloned.get(RESULT).unwrap().clone();
        let order_by = map_cloned.get(ORDER_BY).unwrap().clone();
        let order_dir = map_cloned.get(ORDER_DIR).unwrap().clone();

        let mut map: HashMap<&str, Option<String>> = HashMap::new();
        map.insert(LIMIT, None);
        let (map_cloned, _) = explore_field_attrs!(
            field <- field,
            derive_name <- QUERY,
            map <- map,
            number <- ()
        );

        let limit = map_cloned.get(LIMIT).unwrap().clone();

        let order = if let Some(o) = order_by {
            if let Some(dir) = order_dir.clone() {
                if dir != "asc".to_owned() && dir != "desc".to_owned() {
                    panic!(
                        r#"
                      Only value 'asc' or 'desc' are accepted for attribute order_dir
                    "#
                    );
                }
            }
            Some(OrderOption {
                order_by: o,
                order_dir,
            })
        } else {
            if order_dir.is_some() {
                panic!(
                    r#"
                  order_by is required when order_dir is specified.
                "#
                );
            }
            None
        };

        let limit = if let Some(l) = limit {
            if let Ok(ll) = l.parse::<u32>() {
                Some(ll)
            } else {
                panic!("Limit attribute must be a number")
            }
        } else {
            None
        };
        Self {
            result,
            order,
            limit,
        }
    }

    pub fn to_ident(&self, span: Span) -> Ident {
        self.result
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

    pub fn validate(&self, fields: Vec<String>) {
        if let Some(OrderOption { order_by, .. }) = self.order.clone() {
            if !fields.iter().any(|f| *f == order_by) {
                panic!(
                    "Order by field '{}' is not one of selected fields",
                    order_by
                );
            }
        }
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
            derive_name <- EDGEDB,
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

    pub fn build_field_assignment(field: &Field) -> String {
        let ty = EdgeDbType::get_type(field);

        format!(
            "{field_name} := ({select} {edge_type}${field_name}), ",
            select = SELECT,
            edge_type = ty,
            field_name = get_field_ident(field)
        )
    }

    pub fn get_type(field: &Field) -> String {
        let db_type = EdgeDbType::from_field(field);

        if db_type.exist {
            if db_type.ty.is_none() {
                panic!(
                    r#"
                    Please specify the edgedb type by adding 
                    #[edgedb(type="")]
                "#
                )
            } else if db_type.is_enum() && db_type.name.is_none() {
                panic!(
                    r#"
                    If the edgedb type is enum you must specify its name and its module as follow :
                    #[edgedb(type="enum", name="...", module="")]
                "#
                )
            }
        };

        if let Some(value) = db_type.value() {
            value
        } else {
            DD_SIGN.to_string()
        }
    }
}

impl EdgeEnumValue {
    pub fn from_variant(variant: &Variant) -> Self {
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
                                    value = Some(s.value());
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

        if exist && (value.is_none() || value.clone().unwrap().is_empty()) {
            if value.is_none() {
                panic!(
                    r#"
                    Please specify a value #[value("...")] for enum variant {} 
                "#,
                    variant.ident.to_string()
                )
            }

            if value.clone().unwrap().is_empty() {
                panic!(
                    r#"
                    Please specify a non empty value for #[value("...")] for enum variant {} 
                "#,
                    variant.ident.to_string()
                )
            }
        }

        Self { value }
    }
}

impl Filter {
    pub fn from_field(field: &Field) -> Self {
        let mut s: String = String::default();
        let mut exist = false;

        for attr in &field.attrs {
            if let Ok(Meta::List(syn::MetaList {
                ref path,
                ref mut nested,
                ..
            })) = attr.parse_meta()
            {
                if let Some((true, _)) = path_ident_equals(path, FILTER) {
                    exist = true;
                    if let Some(NestedMeta::Meta(Meta::Path(p))) = nested.iter().next() {
                        s = p.segments[0].ident.to_string();
                        break;
                    }
                }
            }
        }

        if !exist {
            panic!(
                "Please please add #[filter] attribute to field {}",
                get_field_ident(field)
            )
        }

        if s.is_empty() {
            panic!("Please specify filter type : Is, IsNot, Like or In ")
        }

        let check_type = |field: &Field| {
            if is_type_name(&field.ty, "()") {
                panic!("Type () is not accepted for Filter type {}", s);
            }
        };

        match s.as_str() {
            "Exists" => {
                if !is_type_name(&field.ty, "()") {
                    panic!(
                        r#"
                        Filter type Exists is only accepted for field of type ()
                    "#
                    )
                }
                Filter::Exists
            }
            "Is" => {
                check_type(field);
                Filter::Is
            }
            "IsNot" => {
                check_type(field);
                Filter::IsNot
            }
            "Like" => {
                check_type(field);
                if !is_type_name(&field.ty, "String") {
                    panic!(
                        r#"
                        Filter type Like is only accepted for field of type String
                    "#
                    )
                }
                Filter::Like
            }
            "ILike" => {
                check_type(field);
                if !is_type_name(&field.ty, "String") {
                    panic!(
                        r#"
                        Filter type ILike is only accepted for field of type String
                    "#
                    )
                }
                Filter::ILike
            }
            "In" => {
                check_type(field);
                if !is_type_name(&field.ty, "Vec") {
                    panic!(
                        r#"
                        Filter type In is only accepted for field of type Vec<>
                    "#
                    )
                }
                Filter::In
            }
            "GreaterThan" => {
                check_type(field);
                Filter::GreaterThan
            }
            "GreaterThanOrEqual" => {
                check_type(field);
                Filter::GreaterThanOrEqual
            }
            "LesserThan" => {
                check_type(field);
                Filter::LesserThan
            }
            "LesserThanOrEqual" => {
                check_type(field);
                Filter::LesserThanOrEqual
            }
            _ => {
                panic!(
                    r#"
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
                "#,
                    s
                )
            }
        }
    }

    pub fn build_filter_assignment(table_name: String, field: &Field) -> String {
        let ty = EdgeDbType::get_type(field);

        let mut is_exists = false;

        let symbol = match Filter::from_field(field) {
            Filter::Is => "=",
            Filter::IsNot => "!=",
            Filter::Like => "like",
            Filter::ILike => "ilike",
            Filter::In => "in",
            Filter::GreaterThan => ">",
            Filter::LesserThan => "<",
            Filter::GreaterThanOrEqual => ">=",
            Filter::LesserThanOrEqual => "<=",
            Filter::Exists => {
                is_exists = true;
                "exists"
            }
        };

        if is_exists {
            format!(
                " {symbol} {table}.{field_name}",
                symbol = symbol,
                table = table_name,
                field_name = get_field_ident(field)
            )
        } else {
            format!(
                " {table}.{field_name} {symbol} ({select} {edge_type}${field_name})",
                table = table_name,
                symbol = symbol,
                select = SELECT,
                edge_type = ty,
                field_name = get_field_ident(field)
            )
        }
    }
}

impl Options {
    pub fn from_field(field: &Field) -> Option<Self> {
        for attr in &field.attrs {
            if let Ok(Meta::Path(ref path)) = attr.parse_meta() {
                if let Some((true, _)) = path_ident_equals(path, OPTIONS) {
                    return Some(Self {});
                }
            }
        }

        None
    }
}
