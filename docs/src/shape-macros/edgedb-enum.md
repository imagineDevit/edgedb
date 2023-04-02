# Edgedb Enum

    #[edgedb_enum]{
        #[value]
    }

An enum decorated #[edgedb_enum] is a representation of an edgeDB scalar enum type.

The `value` argument is used when the rust enum variant name does not match the edgeDB enum variant name.


### Usage

The following scalar enum types 👇

```sql
    scalar type Gender extending enum<Man, Woman>;
    scalar type Status extending enum<Opened, InProgress, Done, Closed>;
```

can then be represented by 👇 

```rust
    #[edgedb_enum]
    pub enum Gender {
        #[value("Man")]
        Male,
        #[value("Woman")]
        Female,
    }

    #[edgedb_enum]
    pub enum Status {
        Opened,
        InProgress,
        #[value("Done")]
        Finished,
        Closed
    }
```