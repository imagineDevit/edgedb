# Set 

_**Set**_ attribute represents a update query set statement.

    #[set(option)]
    
_**Option**_  can take following values:

- 'assign' or ':=' 
- 'concat' or '++' (only for string)
- 'push' or '+=' (only for vec)

### Usage 

```rust
    struct UpdateUser {
        #[set(option="assign")]
        pub name: String,
        ...
    }
````