extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod constants;
mod enumerations;
mod helpers;
mod insert;
mod result;
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

/// <h1>Select Query</h1>
///
///
/// <h2 style = "text-decoration: underlined">Usage :</h2>
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
#[proc_macro_derive(SelectQuery, attributes(edgedb, query, filter, options))]
pub fn select_query(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let result = select::select_query::do_derive(&ast_struct);
    result
}

/// <h1>Edgedb Enum</h1>
///
///
/// <h2 style = "text-decoration: underlined">Usage :</h2>
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
    let tokens = enumerations::edgedb_enum::do_derive(&ast_struct);
    tokens.into()
}

/// <h1>EdgeResult</h1>
///
/// <h2 style = "text-decoration: underlined">Usage :</h2>
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
///     assert_eq!(eq.query, "select users::User { id, name } filter .name = (select <str>$name) ");
/// }
/// ```
#[proc_macro_derive(EdgedbResult, attributes(query_shape))]
pub fn edgedb_result(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as DeriveInput);
    let tokens = result::edgedb_result::do_derive(&ast_struct);
    tokens.into()
}
