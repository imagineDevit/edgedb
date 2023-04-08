# Field 

_**Field**_ attribute gives the query struct field information.

There are two types of field attributes:

<br>

#### 👉 For queries field 
___

      #[field(column_name, param, scalar)]

<br>

| Argument             | Optional | Description                                                                                                                    |
|----------------------|----------|--------------------------------------------------------------------------------------------------------------------------------|
| column_name          | yes      | The name edgeDB table column represented by the field.<br> <br/> _**By default**_: the name of the field                       |
| param                | yes      | The query parameter name.<br>  <br> _**By default**_: the name of the field_**By default**_: the name of the field             |
| scalar               | yes      | The field scalar type (example : "<default::str>").<br> <br/>_**By default**_: the scalar type corresponding to the field type | 
| link_property (bool) | yes      | Marks a field as link property. <br> <br/> _**By default**_: false                                                             | 

<br>

### Usage

```rust
    struct InsertUser {
        #[field(column_name= "first_name", param = "username", scalar = "<default::str>")]
        name: String
    }
````

<br>

#### 👉 For query result field
___
       #[field(column_name, wrapper_fn, default_value)]

<br>

| Argument             | Optional | Description                                                                                             |
|----------------------|----------|---------------------------------------------------------------------------------------------------------|
| column_name          | yes      | The name edgeDB table column represented by the field.<br> <br/>_**By default**_: the name of the field |
| wrapper_fn           | yes      | The function to apply to the field value                                                                |
| defaut_value         | yes      | The result field default value                                                                          |
| link_property (bool) | yes      | Marks a field as link property.<br> <br/> _**By default**_: false                                       | 

<br>


### Usage 

```rust
    
    struct UserResult {
        #[field(column_name= "first_name", wrapper_fn ="str_upper", default_value="John")]
        name: String
    }
````