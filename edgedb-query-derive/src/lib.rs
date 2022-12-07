//! Edgedb-query-derive provide a bunch procedural macros in order to facilitate writing of queries when using edgedb-rust crate.

extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod constants;
mod shapes;
mod helpers;
mod insert;
mod select;
mod utils;
mod delete;
mod update;

/// InsertQuery creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeValue
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeScalar
///  * edgedb_query::models::edge_query::ToEdgeQuery
///  * ToString
///
///
/// ## Usage
///
/// ```rust
///  use edgedb_protocol::value::Value;
///  use edgedb_protocol::codec::EnumValue;
///  use edgedb_query::{*, ToEdgeShape, models::{ edge_query::*, query_result::BasicResult}};
///
///  #[derive(Default, EdgedbResult)]
///  pub struct UserResult {
///      pub id: String,
///      pub name: NameResult,
///  }
///
///  #[derive(EdgedbEnum)]
///  pub enum Sex {
///      #[value("male")]
///      Male,
///      #[value("female")]
///      Female,
///  }
///
///  #[derive(InsertQuery)]
///  pub struct InsertUser {
///      #[meta(module = "users", table = "User")]
///      #[result(type = "UserResult")]
///      __meta__: (),
///
///      pub name: String,
///      pub surname: Option<String>,
///      pub age: i32,
///      pub major: bool,
///      pub vs: Vec<String>,
///      #[scalar(type = "enum", module = "users", name = "Gender")]
///      pub gender: Sex,
///      #[nested_query]
///      pub wallet: Wallet,
///  }
///  #[derive(InsertQuery)]
///  pub struct Wallet {
///      #[meta(module = "users", table = "Wallet")]
///      __meta__: (),
///      pub money: i16,
///  }
///
///  fn main() {
///     let insert_user = InsertUser {
///             __meta__: (),
///             name: "Joe".to_string(),
///             surname: Some("sj".to_string()),
///             age: 35,
///             major: true,
///             vs: vec!["vs1".to_string()],
///             gender: Sex::Male,
///             wallet: Wallet {
///                 __meta__: (),
///                 money: 0 }
///         };
///
///         let query: EdgeQuery = insert_user.to_edge_query();
///
///         println!("{:#?}", query.query);
///
///         let expected = r#"
///            select (
///               insert users::User {
///                 name := (select <str>$name),
///                 surname := (select <str>$surname),
///                 age := (select <int32>$age),
///                 major := (select <bool>$major),
///                 vs := (select <array<str>>$vs),
///                 gender := (select <users::Gender>$gender),
///                 wallet := (
///                     insert users::Wallet{
///                         money := (select <int16>$money),
///                     }),
///                 })
///                 {
///                     id,
///                     name : { name }
///                 }
///         "#.to_owned().replace("\n", "");
///
///         assert_eq!(query.query.replace(" ", ""), expected.replace(" ", ""));
///
///         if let Some(Value::Object { shape, mut fields}) = query.args {
///
///             crate::test_utils::check_shape(&shape, vec!["name", "surname", "age", "major", "vs", "gender", "wallet"]);
///
///             let wallet_field = fields.pop();
///
///             let vs_val = &insert_user.vs[0];
///
///             assert_eq!(fields, vec![
///                 Some(Value::Str(insert_user.name)),
///                 Some(Value::Str(insert_user.surname.unwrap())),
///                 Some(Value::Int32(insert_user.age as i32)),
///                 Some(Value::Bool(insert_user.major)),
///                 Some(Value::Array(vec![Value::Str(vs_val.clone())])),
///                 Some(Value::Enum(EnumValue::from("male")))
///             ]);
///
///             if let Some(Some(Value::Object { shape, fields})) = wallet_field {
///                 let w_elmts = &shape.elements;
///                 assert_eq!(w_elmts.len(), 1);
///                 assert_eq!(fields, vec![Some(Value::Int16(insert_user.wallet.money as i16))])
///             }
///         } else {
///             assert!(false)
///         }
///  }
/// ```
#[proc_macro_derive(InsertQuery, attributes(meta, result, conflict_on, conflict_else,  scalar, nested_query))]
pub fn insert_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = insert::insert_query::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// SelectQuery creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeValue
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeScalar
///  * edgedb_query::models::edge_query::ToEdgeQuery
///  * ToString
///
/// ## Usage
///
/// ```rust
/// use edgedb_protocol::value::Value;
///  use edgedb_protocol::codec::EnumValue;
///  use edgedb_query::{*, ToEdgeShape, models::{ edge_query::*, query_result::BasicResult}};
///  use edgedb_query::queries::{select::{OrderDir, OrderOptions, SelectOptions} , filter::Filter};
///
///  #[derive(SelectQuery)]
///  pub struct FindMajorUsersWithOptions {
///      #[meta(module = "users", table = "User")]
///      #[result(type = "UserResult")]
///      __meta__: (),
///
///      #[options]
///      options: SelectOptions<'static>,
///
///      #[filter(operator = "GreaterThanOrEqual")]
///      pub age: i8,
///  }
///
///  #[derive(Default, EdgedbResult)]
///  pub struct UserResult {
///      pub id: String,
///      pub name: String,
///  }
///
///  fn main() {
///     let q = FindMajorUsersWithOptions {
///             __meta__ : (),
///             options: SelectOptions {
///                 table_name: "User",
///                 module: Some("users"),
///                 order_options: Some(OrderOptions {
///                     order_by: "name".to_string(),
///                     order_direction: Some(OrderDir::Desc)
///                 }),
///                 page_options: None
///             },
///             age: 18
///         };
///
///         let edge_query : EdgeQuery = q.to_edge_query();
///
///         let expected = "select users::User {id,name,age} filter users::User.age >= (select <int16>$age) order by users::User.name desc";
///
///         assert_eq!(edge_query.query, expected);
///
///         if let Some(Value::Object { shape, fields }) = edge_query.args {
///             crate::test_utils::check_shape(&shape, vec!["age"]);
///             assert_eq!(fields, vec![
///                 Some(Value::Int16(q.age as i16))
///             ])
///         } else {
///             assert!(false)
///         }
///  }
/// ```
#[proc_macro_derive(SelectQuery, attributes(meta, result, filter, filters, options))]
pub fn select_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);

    let result = select::select_query::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// UpdateQuery creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeValue
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeScalar
///  * edgedb_query::models::edge_query::ToEdgeQuery
///  * ToString
///
/// ## Usage
///
/// ```rust
///  use edgedb_protocol::value::Value;
///  use edgedb_query_derive::{EdgedbSet, UpdateQuery, EdgedbFilters};
///  use edgedb_query::{ToEdgeScalar, ToEdgeQl, ToEdgeValue, queries::filter::Filter, models::edge_query::{ToEdgeQuery}};
///  use crate::test_utils::check_shape;
///
///  #[derive(EdgedbSet)]
///  pub struct MySet {
///      pub name: String,
///  }
///
///  #[derive(EdgedbFilters)]
///  pub struct MyFilter {
///      #[filter(operator="=", column_name="identity.first_name", wrapper_fn="str_lower")]
///      pub first_name: String,
///      #[filter(operator=">=",  conjunctive="And")]
///      pub age: i8,
///  }
///
///  #[derive(UpdateQuery)]
///  pub struct UpdateUserName {
///      #[meta(module = "users", table="User")]
///      __meta__: (),
///      #[set]
///      pub set: MySet,
///      #[filters]
///      pub filter: MyFilter,
///  }
///
///     pub fn main() {
///         let q = UpdateUserName {
///             __meta__: (),
///             set: MySet {
///                 name: "Joe".to_string()
///             },
///             filter: MyFilter {
///                 first_name :"Henri".to_string(),
///                 age : 18
///             }
///         };
///
///         let eq = q.to_edge_query();
///
///         let expected_query = r#"
///             update users::User
///             filter str_lower(users::User.identity.first_name) = (select <str>$first_name)
///             and users::User.age >= (select <int16>$age)
///             set {
///                 name := (select <str>$name)
///             }
///         "#.to_owned().replace("\n", "");
///
///         assert_eq!(eq.query.replace(" ", ""), expected_query.replace(" ", ""));
///
///         if let Some(Value::Object { shape, fields}) = eq.args {
///             check_shape(&shape, vec!["first_name", "age", "name"]);
///
///             assert_eq!(fields, vec![
///                 Some(Value::Str(q.filter.first_name)),
///                 Some(Value::Int16(q.filter.age as i16)),
///                 Some(Value::Str(q.set.name)),
///             ]);
///         }
///
///     }
/// ```
#[proc_macro_derive(UpdateQuery, attributes(meta, result, set, filters))]
pub fn update_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = update::update_query::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// UpdateQuery creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeValue
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeScalar
///  * edgedb_query::models::edge_query::ToEdgeQuery
///  * ToString
///
/// ## Usage
///
/// ```rust
/// use edgedb_protocol::value::Value;
/// use edgedb_query_derive::{DeleteQuery, EdgedbFilters};
/// use edgedb_query::{*, queries::filter::Filter, models::{edge_query::{ToEdgeQuery, EdgeQuery}}};
///
/// #[derive(DeleteQuery)]
///  pub struct DeleteUsersByName {
///    #[meta(module="users", table="User")]
///    __meta__: (),
///    #[filters]
///    pub filters: NameFilter
///  }
///
///  #[derive(EdgedbFilters)]
///  pub struct NameFilter {
///    #[filter(operator="=")]
///    pub name: String
///  }
///
///  fn main() {
///   let del_users = DeleteUsersByName {
///             __meta__: (),
///             filters: NameFilter {
///                 name: "Joe".to_owned()
///             }
///         };
///
///         let edge_query: EdgeQuery = del_users.to_edge_query();
///
///         assert_eq!(edge_query.query, "delete users::User filter users::User.name = (select <str>$name)");
///
///         if let Some(Value::Object { shape, fields }) = edge_query.args {
///             crate::test_utils::check_shape(&shape, vec!["name"]);
///             assert_eq!(fields, vec![
///                 Some(Value::Str(del_users.filters.name ))
///             ])
///         } else {
///             assert!(false)
///         }
///  }
///
/// ```
#[proc_macro_derive(DeleteQuery, attributes(meta, filter, filters))]
pub fn delete_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = delete::delete_query::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// EdgedbEnum creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeValue
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeScalar
///  * ToString
///
/// ## Usage
///
/// ```rust
/// #[EdgedbEnum]
/// pub enum TodoStatus {
///     #[value("TodoReady")]
///     Ready,
///     #[value("TodoComplete")]
///     Complete
/// }
///
/// ```
#[proc_macro_derive(EdgedbEnum, attributes(value))]
pub fn edgedb_enum(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = shapes::edgedb_enum::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// EdgedbResult creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeShape
///  * edgedb_query::ToEdgeScalar
///
/// ## Usage
///
/// ``` rust
///  #[derive(Default, EdgedbResult)]
///  pub struct UserResult {
///     pub id: String,
///     pub name: String,
/// }
/// ```
#[proc_macro_derive(EdgedbResult, attributes(field, back_link))]
pub fn edgedb_result(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = shapes::edgedb_result::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// EdgedbFilters creates implementation of following traits for the annotated struct :
///  * edgedb_query::queries::filter::Filter
///
/// ## Usage
///
/// ```rust
/// #[derive(EdgedbFilters)]
/// pub struct NameFilter {
///     #[filter(Is)]
///     pub name: String,
/// }
/// ```
#[proc_macro_derive(EdgedbFilters, attributes(filter))]
pub fn edgedb_filters(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = shapes::edgedb_filter::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}

/// EdgedbSet creates implementations of following traits for the annotated struct :
///  * edgedb_query::ToEdgeQl
///  * edgedb_query::ToEdgeValue
///
///
/// ## Usage
///
/// ```rust
///  #[derive(EdgedbSet)]
///  pub struct MySet {
///      #[field(column_name="first_name", assignment = "Concat")]
///      #[scalar(type="str")]
///      pub name: String,
///      #[scalar(type="enum", name="State", module="default")]
///      pub status: Status
///  }
///
///  #[derive(EdgedbEnum)]
///  pub enum Status {
///      Open, _Closed
///  }
/// ```
#[proc_macro_derive(EdgedbSet, attributes(scalar, field))]
pub fn edgedb_set(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = shapes::edgedb_set::do_derive(&ast_struct);
    result.unwrap_or_else(|e| e.to_compile_error().into())
}