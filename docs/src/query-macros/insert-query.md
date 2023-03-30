# InsertQuery

        #[insert_query(module, table, result)] {
            #[field]
            #[nested_query]
            #[unless_conflict]
        }

**_insert_query_** attribute macro indicates that the struct represents an edgeDB insert query.

**_insert_query_** attribute takes three arguments 👇

| Argument | Optional | Description                                                                                                                                               |
|----------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| module   | yes      | The name of the edgeDB module on which the query is executed.<br> <br/> _**By default**_: 'default'                                                       |
| table    | no       | The name of the edgeDB table on which the query is executed.<br>                                                                                          |
| result   | yes      | The query result type.<br> <br/>_**By default**_: [BasicResult](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/models/query_result.rs) | 


Its fields can be decorated with tag [#[field]](../inner_attributes/field.md),  [#[nested_query]](../inner_attributes/nested_query.md) or  [#[unless_conflict]](../inner_attributes/unless_conflict.md).
<br>

---

### Usage

Consider the following edgeDB schema 👇

````sql
    module models {
       scalar type Gender extending enum<Male, Female>
        
       type Person {
            required property user_name -> str {
                constraint exclusive;
            }
            required property age -> int16;
            required property gender -> Gender;
            link address -> Address;
       }
        
       type Address {
            required property num -> int16;
            required property street -> str;
            required property city -> str;
            required property zipcode -> int32;
       }
   }
````

Let's write a struct that represents query to insert a new Person into the database

`````rust
    #[insert_query(module="models", table="Person", result="Person")]
    pub struct InsertPerson {
        #[field(column_name="user_name")]
        pub name: String,    
        #[field(scalar="<int16>")]   
        pub age: u8,
        #[field(column_name="gender", scalar="<models::Gender>")]
        pub sex: Sex,
    }

    #[edgedb_enum]
    pub enum Sex {
        #[value("Male")]
        Man,
        #[value("Female")]
        Woman
    }

    #[query_result]
    pub struct Person {
        pub user_name: String,
        pub age: u8
    }
`````
___
#### 🤷‍♀️ But what about the person's address❔
___

Since the address is stored in a separate database table we need to insert a new Address while creating a new Person, right ?

Ok, so let's write the address's insert query corresponding struct.

`````rust
    #[insert_query(module="models", table="Address")]
    pub struct InsertAddress {
        #[field(column_name="num", scalar="<int16>")]
        pub number: u16,
        pub street: String,
        pub city: String,
        #[field(column_name="zipcode", scalar="<int32>")]
        pub zip_code: u32
    }
`````
To insert the two entities with a single query, add the insert address query as a nested query of the person insert query.

````rust
    #[insert_query(module="models", table="Person", result="Person")]
    pub struct InsertPerson {
        ...
        #[nested_query]
        pub address: InsertAddress
    }
````

Okay, great! Now we can persist a Person with address.

___
#### 🤷‍♀️ But what if a Person already exists with the same name ❔
___


Remember !!!

In the edgeDB schema, the _type Person_ has an exclusive constraint on its field _user_name_.

To handle this case we need to use an _unless conflict_ statement.


````rust
    #[insert_query(module="models", table="Person", result="Person")]
    pub struct InsertPerson {
        ...
        #[unless_conflict(on="user_name")]
        pub handle_conflict: edgedb_query::queries::conflict::UnlessConflict
    }
````

The new field _handle_conflict_ decorated with [#[unless_conflict]](../inner_attributes/unless_conflict.md) tag add a ``` unless conflict on .user_name``` statement to the query.

It is possible to add an else query to the unless conflict statement by using a [edgedb_query::queries::conflict::UnlessConflictElse<T: ToEdgeQuery>](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/queries/conflict.rs) type instead of an UnlessConflict type.


