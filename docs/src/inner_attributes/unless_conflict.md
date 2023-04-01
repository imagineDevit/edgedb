# Unless Conflict 

_**Unless Conflict**_ attribute represents a[unless conflict else ](https://www.edgedb.com/docs/edgeql/insert#conflicts) statement.<br> The decorated field must by of type 
[edgedb_query::queries::conflict::UnlessConflict](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/queries/conflict.rs) or [edgedb_query::queries::conflict::UnlessConflictElse<T: ToEdgeQuery>](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/queries/conflict.rs).

    #[unless_conflict(on)]
    
_**on**_ attribute (optional) lists conflict column's names separated by a comma.

### Usage 

```rust
    #[insert_query(table="Users")]
    struct InsertUser {
        #[field(column_name="firstname")]
        pub first_name: String,
        #[field(column_name="lastname")]
        pub last_name: String,
        pub age: u8,
        #[unless_conflict(on="firstname, lastname")]
        pub conflict: UnlessConflictElse<FindUserName>
    }
    
    #[select_query(table="Users")]
    struct FindUserName {
       
    }
````