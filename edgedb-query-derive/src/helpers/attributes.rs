use crate::constants::{TARGET_COLUMN, CONJUNCTIVE, SCALAR_TYPE, META, ENUM, FILTER, INF_SIGN, LIMIT, MODULE, NAME, OPERATOR, OPTIONS, ORDER_BY, ORDER_DIR, RESULT, SELECT, SUP_SIGN, TABLE, BACKLINK, TYPE, VALUE, TARGET_TABLE, SOURCE_TABLE, FILTERS, COLUMN_NAME, WRAPPER_FN, FIELD, DEFAULT_VALUE, SCALAR};
use crate::utils::field_utils::get_field_ident;
use crate::utils::path_utils::path_ident_equals;
use crate::utils::type_utils::is_type_name;
use proc_macro2::{Ident, Span};
use std::collections::HashMap;

use syn::{Field, Meta, MetaNameValue, NestedMeta, Type, Variant};

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
    pub fn has_result(&self) -> bool {
        self.result_type.is_some()
    }

    pub fn from_field(field: &Field) -> Self {
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
            result_type: result,
            order,
            limit,
        }
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
            SCALAR_TYPE.to_string()
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

impl Operator {
    fn from_str(ty: &Type, s: String) -> Operator {
        let check_type = |_: &Type| {
            if is_type_name(&ty, "()") {
                panic!("Type () is not accepted for Filter type {}", s);
            }
        };

        match s.to_lowercase().as_str() {
            "exists" => {
                if !is_type_name(&ty, "()") {
                    panic!(
                        r#"
                        Filter type Exists is only accepted for field of type ()
                    "#
                    )
                }
                Operator::Exists
            }
            "notexists" | "!exists" => {
                if !is_type_name(&ty, "()") {
                    panic!(
                        r#"
                        Filter type NotExists is only accepted for field of type ()
                    "#
                    )
                }
                Operator::NotExists
            }
            "is" | "="=> {
                check_type(ty);
                Operator::Is
            }
            "isnot" | "!=" => {
                check_type(ty);
                Operator::IsNot
            }
            "like" => {
                check_type(ty);
                if !is_type_name(&ty, "String") {
                    panic!(
                        r#"
                        Filter type Like is only accepted for field of type String
                    "#
                    )
                }
                Operator::Like
            }
            "ilike" => {
                check_type(ty);
                if !is_type_name(&ty, "String") {
                    panic!(
                        r#"
                        Filter type ILike is only accepted for field of type String
                    "#
                    )
                }
                Operator::ILike
            }
            "in" => {
                check_type(ty);
                if !is_type_name(&ty, "Vec") {
                    panic!(
                        r#"
                        Filter type In is only accepted for field of type Vec<>
                    "#
                    )
                }
                Operator::In
            }
            "notin" => {
                check_type(ty);
                if !is_type_name(&ty, "Vec") {
                    panic!(
                        r#"
                        Filter type NotIn is only accepted for field of type Vec<>
                    "#
                    )
                }
                Operator::NotIn
            }
            "greaterthan" | ">" => {
                check_type(ty);
                Operator::GreaterThan
            }
            "greaterthanorequal" | ">=" => {
                check_type(ty);
                Operator::GreaterThanOrEqual
            }
            "lesserthan" | "<" => {
                check_type(ty);
                Operator::LesserThan
            }
            "lesserthanorequal" | "<=" => {
                check_type(ty);
                Operator::LesserThanOrEqual
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

    pub fn build(&self, table_name: String, field: &Field, column_name: Option<String>, wrapper_fn: Option<String>) -> String {
        let ty = EdgeDbType::get_type(field);

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
            format!(
                " {symbol} {table}.{column_name}",
                symbol = symbol,
                table = table_name,
                column_name = column_name
            )
        } else {
            format!(
                " {wrapped_field_name} {symbol} ({select} {edge_type}${field_name})",
                symbol = symbol,
                select = SELECT,
                edge_type = ty,
                wrapped_field_name = wrapped_field_name,
                field_name = field_name
            )
        }
    }
}

impl Filter {
    pub fn from_field(field: &Field, index: usize) -> Self {
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
            panic!(
                "Please please add #[filter(operator=\"...\",)] attribute to field {}",
                get_field_ident(field)
            )
        }

        let operator = map_cloned.get(OPERATOR).unwrap().clone();

        let operator = if let Some(op) = operator {
            Operator::from_str(&field.ty, op)
        } else {
            panic!(
                r#"
                    Please please add operator attribute to field {}
                    #[filter(operator=\"...\")
                "#,
                get_field_ident(field)
            )
        };

        let conjunctive = map_cloned.get(CONJUNCTIVE).unwrap().clone();
        let column_name = map_cloned.get(COLUMN_NAME).unwrap().clone();
        let wrapper_fn = map_cloned.get(WRAPPER_FN).unwrap().clone();

        let conjunctive = if let Some(c) = conjunctive {
            match c.as_str() {
                "Or" => Some(Conjunctive::Or),
                "And" => Some(Conjunctive::And),
                _ => panic!(
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
                    c
                )
            }
        } else {
            if index > 0 {
                panic!(r#"
                    Please specify conjunctive attribute to field {}
                    #[filter(operator=\"...\"), conjunctive=\"...\"]
                "#, get_field_ident(field))
            }
            None
        };

        Filter { operator, conjunctive, column_name, wrapper_fn }
    }

    pub fn build_filter_assignment(table_name: String, field: &Field, index: usize) -> String {
        let filter = Filter::from_field(field, index);
        let q = filter.operator.build(table_name, field, filter.column_name, filter.wrapper_fn);
        if index == 0 {
            q
        } else {
            let c = filter.conjunctive.unwrap();
            format!(" {}{}", c.to_string(), q)
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
    pub fn from_field(field: &Field) -> Self {
        let field_name = get_field_ident(field).to_string();

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

        let module = Self::get_value(&map_cloned, MODULE, field_name.clone());

        let source_table = Self::get_value(&map_cloned, SOURCE_TABLE, field_name.clone());

        let target_table = Self::get_value(&map_cloned, TARGET_TABLE, field_name.clone());

        let target_column = Self::get_value(&map_cloned, TARGET_COLUMN, field_name.clone());

        let result = Self::get_value(&map_cloned, RESULT, field_name.clone());


        Self { module, source_table, target_table, target_column, result }
    }

    pub fn build_assignment(field: &Field) -> (String, String) {
        let t = QueryShape::from_field(field);
        let source = t.source_table;
        let column = t.target_column;
        let module = t.module;
        let table = t.target_table;

        (format!("select {module}::{source}.<{column}[is {module}::{table}]"), t.result.clone())
    }

    fn get_value(map_cloned: &HashMap<&str, Option<String>>, name: &str, field_name: String) -> String {
        let result = if let Some(t) = map_cloned.get(name).unwrap().clone() {
            Self::required(t, name)
        } else {
            panic!(r#"
                Please add {} attribute to query_shape macro on field {}
            "#, name, field_name)
        };
        result
    }

    fn required(value: String, name: &str) -> String {
        if value.clone().is_empty() {
            panic!(r#"
                Non empty value required for {} attribute
            "#, name)
        }
        value
    }
}

impl ResultField {
    pub fn from_field(field: &Field) -> Self {
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

        let all_nones = vec![column_name.clone(), wrapper_fn.clone()].iter().all(|o| o.is_none());

        if found && all_nones {
            panic!("#[field] must have at least column_name or wrapper_fn attribute")
        }

        if wrapper_fn.clone().is_none()  && default_value.is_some() {
            if let Some(c) = column_name.clone() {
                let f_name = get_field_ident(field).to_string();
                if c == f_name {
                    panic!("default_value cannot be applied to the field {}", f_name);
                }
            }
        }

        Self {
            column_name,
            wrapper_fn,
            default_value
        }
    }

    pub fn build_statement(field: &Field) -> String {
        let result_field = Self::from_field(field);

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

        s
    }
}
