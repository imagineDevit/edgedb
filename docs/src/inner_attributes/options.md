#Options 

_**Options**_ attribute marks a field as a select option (order and pagination options). The decorated fied must be of type [edgedb_query::queries::select::SelectOptions](https://docs.rs/edgedb-query/0.2.2/edgedb_query/queries/select/struct.SelectOptions.html)

    #[options]
    

### Usage 

```rust
    #[select_query(table="Users")]
    struct FindUser {
        #[options]
        pub options: SelectOptions
    }
````