# Set 

_**Set**_ attribute represents a update query set statement.

    #[set(option)]
    
_**Option**_  can take following values:

- **assign** or **:=**
- **concat** or **++**   _(only for string)_
- **push** or **+=**   _(only for vec)_

### Usage 

```rust
    struct UpdateUser {
        #[set(option="assign")]
        pub name: String,
        ...
    }
````