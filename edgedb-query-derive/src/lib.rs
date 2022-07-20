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

/// <h1>Insert Query</h1>
///
///
/// <h2 style = "text-decoration: underlined">Usage :</h2>
///
/// ```rust
/// #[derive(InsertQuery)]
/// pub struct User {
///     #[edgedb(module = "users", table = "User")]
///     #[query(result = "UserResult")]
///     __meta__: (),
///
///     pub name: String,
///     #[edgedb(type="int16")]
///     pub age: u8,
///     pub major: bool,
///     #[edgedb(type = "enum", name = "Gender")]
///     pub gender: String,
///     pub b: Option<u8>,
/// }
///
/// #[derive(Default, EdgedbResult)]
/// pub struct UserResult {
///     pub id: String,
///     pub name: String,
/// }
///
/// fn main() {
///     
/// }
/// ```
#[proc_macro_derive(InsertQuery, attributes(edgedb, query))]
pub fn insert_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = insert::insert_query::do_derive(&ast_struct);
    result
}

/// # Select Query
///
///
/// <h3 style = "text-decoration: underlined">Usage :</h3>
///
/// ```rust
/// #[derive(SelectQuery)]
/// pub struct FindUserByName {
///     #[edgedb(module = "users", table = "User")]
///     #[query(result = "UserResult")]
///     __meta__: (),
///
///     #[filter(Is)]
///     pub name: String,
/// }
///
/// #[derive(Default, EdgedbResult)]
/// pub struct UserResult {
///     pub id: String,
///     pub name: String,
/// }
/// ```
#[proc_macro_derive(SelectQuery, attributes(edgedb, query, filter, filters, options))]
pub fn select_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = select::select_query::do_derive(&ast_struct);
    result
}

/// # Edgedb Enum
///
/// <h3 style = "text-decoration: underlined">Usage :</h3>
///
/// ```rust
///
/// #[EdgedbEnum]
/// pub enum TodoStatus {
///     #[value("TodoReady")]
///     Ready,
///     #[value("TodoComplete")]
///     Complete
/// }
///
/// #[derive(InsertQuery)]
/// pub struct Todo {
///     #[edgedb(module = "default", table = "Todos")]
///     pub t: (),
///     pub label: String,
///     #[edgedb(type = "enum", name = "Status")]
///     pub status: Status
/// }
///
/// fn main() {
///     let todo = Todo {
///         t: (),
///         label : "Learn rust".to_string(),
///         status: TodoStatus::Complete
///     };
///     let ql = todo.to_edgeql();
///     assert_eq!(ql, "select (insert default::Todos { label := <str>\"Learn rust\", status := <Status>\"TodoComplete\" }){ id }");
/// }
/// ```
#[proc_macro_derive(EdgedbEnum, attributes(value))]
pub fn edgedb_enum(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let tokens = shapes::edgedb_enum::do_derive(&ast_struct);
    tokens.into()
}

/// # EdgeResult
///
/// <h3 style = "text-decoration: underlined">Usage :</h3>
///
/// ``` rust
/// #[derive(SelectQuery)]
/// pub struct FindUserByName {
///     #[edgedb(module = "users", table = "User")]
///     #[query(result = "UserResult")]
///     __meta__: (),
///
///     #[filter(Is)]
///     pub name: String,
/// }
///  #[derive(Default, EdgedbResult)]
///  pub struct UserResult {
///     pub id: String,
///     pub name: String,
/// }
///
/// fn main() {
///     use edgedb_query::models::edge_query::EdgeQuery;
///     let todo = FindUserByName {
///         __meta__: (),
///         name : "Joe".to_string(),
///     };
///     let eq: EdgeQuery = todo.to_edge_query();
///     
///     // assert_eq!(eq.query, "select users::User { id, name } filter .name = (select <str>$name) ");
/// }
/// ```
#[proc_macro_derive(EdgedbResult, attributes(field, query_shape))]
pub fn edgedb_result(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let tokens = shapes::edgedb_result::do_derive(&ast_struct);
    tokens.into()
}

/// # EdgedbFilters
///
/// <h3 style = "text-decoration: underlined">Usage :</h3>
///
/// ```rust
/// #[derive(SelectQuery)]
/// pub struct FindUserByName {
///     #[edgedb(module = "users", table = "User")]
///     #[query(result = "UserResult")]
///     __meta__: (),
///
///     #[filters]
///     filters: NameFilter
/// }
/// #[derive(Default, EdgedbResult)]
/// pub struct UserResult {
///     pub id: String,
///     pub name: String,
/// }
///
/// #[derive(EdgedbFilters)]
/// pub struct NameFilter {
///     #[filter(Is)]
///     pub name: String,
/// }
///
/// fn main() {
///     use edgedb_query::models::edge_query::EdgeQuery;
///     let fubn = FindUserByName {
///         __meta__: (),
///         filters: NameFilter {
///             name : "Joe".to_string(),
///         }
///     };
///     let eq: EdgeQuery = fubn.to_edge_query();
///
///     // assert_eq!(eq.query, "select users::User { id, name } filter .name = (select <str>$name) ");
/// }
/// ```
#[proc_macro_derive(EdgedbFilters, attributes(filter, edgedb))]
pub fn edgedb_filters(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let tokens = shapes::edgedb_filter::do_derive(&ast_struct);
    tokens.into()
}
