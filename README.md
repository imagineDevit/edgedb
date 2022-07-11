<img src="assets/logo.png" width="15%" style="margin-top: 25px" alt="https://www.edgedb.com/">  
<i style="font-weight: bold; font-size: 18px; margin-left: 5px">EdgeDB</i> is a new graph-relational database built on top of <i style="font-size: 16px
">🐘 Postgresql </i>

_____


🦾 This project aims to provide a bunch procedural macros in order to facilitate writing of edgedb queries while
using [edgedb-tokio](https://crates.io/crates/edgedb-tokio) crate.


----

#### Examples

```rust

#[EdgedbEnum]
pub enum TodoStatus {
    #[value("TodoReady")]
    Ready,
    #[value("TodoComplete")]
    Complete
}

#[derive(InsertQuery)]
pub struct Todo {
    #[edgedb(module = "default", table = "Todos")]
    pub __meta__: (),

    pub label: String,
    
    #[edgedb(type = "enum", name = "Status")]
    pub status: Status
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todo = Todo {
        __meta__: (),
        label: "Learn rust".to_string(),
        status: TodoStatus::Complete
    };
    let edge_query: edgedb_query::models::EdgeQuery = todo.to_edge_query();

    let client = edgedb_tokio::create_client().await?;

    let json = client
        .query_json(edge_query.query.as_str(), &edge_query.args.unwrap())
        .await?;

    println!("{:#?}", json.to_string());
}
```