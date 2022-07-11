
## InsertQuery

[x] Create the InsertQuery derive macro
```rust
    #[derive(InsertQuery)]
    pub struct User {}
```

[x] Create the derive macro attributes helper

- module
- table
- query result
- type
- type enum

```rust 
    #[derive(InsertQuery)]
    pub struct User {
        /// attribute to define module and tablename
        #[edgedb(module = "users", table="User")]
        pub __meta__: (),
        
        /// attribute to define the result type
        #[query(result = "UserResult")]
        pub __result__: (),
        
        /// attriute to define edgedb type
        #[edgedb(type = "int16")]
        pub total: u16,
        
        /// attribute to define enum type
        #[edgedb(type = "enum", name = "Gender")]
        pub sex: String
    }
     
```


- check type when attribute helper is #[edgedb(type="enum", name"...")]