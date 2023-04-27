# Queries attribute macros

[**Edgedb-query-derive**](https://github.com/imagineDevit/edgedb) crate provide 5 macro attributes that represent a edgeDB query:

- [#[insert_query]](./insert-query.md)
- [#[select_query]](./select-query.md)
- [#[update_query]](./update-query.md)
- [#[delete_query]](./delete-query.md)
- [#[query]](./query.md)
- [#[file_query]](./file-query.md)

Queries attributes (except #[file_query] ) take three arguments 👇

| Argument | Optional | Description                                                                                                                                                  |
|----------|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| module   | yes      | The name of the edgeDB module on which the query is executed.<br> <br/> _**By default**_: 'default'                                                          |
| table    | no       | The name of the edgeDB table on which the query is executed.<br>                                                                                             |
| result   | yes      | The query result type.<br> <br/>_**By default**_: [BasicResult](https://docs.rs/edgedb-query/0.2.2/edgedb_query/models/query_result/struct.BasicResult.html) | 

When a struct is decorated with one of those queries macro attributes, several trait implementations are created for this one:

 - [ToEdgeQl](https://docs.rs/edgedb-query/0.2.2/edgedb_query/trait.ToEdgeQl.html)
 - [ToEdgeValue](https://docs.rs/edgedb-query/0.2.2/edgedb_query/trait.ToEdgeValue.html)
 - [ToEdgeQuery](https://docs.rs/edgedb-query/0.2.2/edgedb_query/models/edge_query/trait.ToEdgeQuery.html)
 - [ToEdgeScalar](https://docs.rs/edgedb-query/0.2.2/edgedb_query/trait.ToEdgeScalar.html)
 - [ToString]()


The following example shows how to get a parameterized query from a struct decorated with a query macro attribute ( #[insert_query] for example) :


Consider the following struct 👇:

```rust
    #[insert_query(module="humans", table="Person")]
    pub struct InsertPerson {
        pub name: String,
        pub age: i8
    }
```

To get a parameterized query from this struct, we can write the following code:

```rust
    #[tokio::main]
    fn main() -> Result<()>{
        let insert_person = InsertPerson {
            name: "John".to_string(),
            age: 20
        };
        
        let edge_query: EdgeQuery = insert_person.to_edge_query();
    
    }

```

`to_edge_query()` ( from [ToEdgeQuery](https://docs.rs/edgedb-query/0.2.2/edgedb_query/models/edge_query/trait.ToEdgeQuery.html) trait ) method returns a [EdgeQuery](https://docs.rs/edgedb-query/0.2.2/edgedb_query/models/edge_query/struct.EdgeQuery.html) struct that contains the query string and the query parameters.

The query's cardinality (`Cardinality::Many` by default) can also be set while getting the query by calling `to_edge_query_with_cardinality()` method instead of `to_edge_query()`.

```rust
    let edge_query: EdgeQuery = insert_person.to_edge_query_with_cardinality(Cardinality::One);
```

With the got query, we can now execute request using edgedb-client like this:

```rust
     if let EdgeQuery{ query, args: Some(params), .. } = edge_query {
         let result: BasicResult = client.query_required_single(query.as_str(), params).await?;
     }
```
