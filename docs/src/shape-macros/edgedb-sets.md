# EdgedbSet

    #[edgedb_sets]{
        #[field]
        #[set]
        #[nested_query]
    }


**_#[edgedb_sets]_** macro attribute indicates that a struct groups a list of #[set].

⚠️
- a #[set] statement can be a nested query.

The decorated type can be used as type of a query struct field. In this case, the field is decorated with a _**#[sets]**_ attribute.


### Usage

```rust
    #[edgedb_sets]
    pub struct PersonSets {
        #[field(column_name="first_name", param = "user_name", scalar="<str>")]
        #[set(option="Concat")]
        pub name: String,
        #[field(scalar="default::State")]
        pub status: Status,
        #[nested_query]
        pub users: FindUsers
    }
      
    #[edged_enum]
    pub enum Status {
        #[value("started")] On,
        #[value("finished")] Off
    }

    #[select_query(table="Users")]
    pub struct FindUsers {
        #[filter(operator="Is")]
        pub name: String
    }

    #[update_query(table="Person")]
    pub struct UpdatePerson {
        #[sets]
        pub sets: PersonSets
    }
      
```