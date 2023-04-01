# Field 

_**Field**_ attribute gives the query struct field information.

There are two types of field attributes:

### For queries field 

      #[field(column_name, param, scalar)]

<br>

| Option      | Optional | Description                                                                                                                    |
|-------------|----------|--------------------------------------------------------------------------------------------------------------------------------|
| column_name | yes      | The name edgeDB table column represented by the field.<br> <br/> _**By default**_: the name of the field                       |
| param       | yes      | The query parameter name.<br>  <br> _**By default**_: the name of the field_**By default**_: the name of the field             |
| scalar      | yes      | The field scalar type (example : "<default::str>").<br> <br/>_**By default**_: the scalar type corresponding to the field type | 

<br>


### For query result field

       #[field(column_name, wrapper_fn, default_value)]

<br>

| Option       | Optional | Description                                                                                             |
|--------------|----------|---------------------------------------------------------------------------------------------------------|
| column_name  | yes      | The name edgeDB table column represented by the field.<br> <br/>_**By default**_: the name of the field |
| wrapper_fn   | yes      | The function to apply to the field value                                                                |
| defaut_value | yes      | The result field default value                                                                          |

<br>


### Usage 

```rust
    
    ...
    struct InsertUser {
        #[field(column_name= "first_name", param = "username", scalar = "<default::str>")]
        name: String,
        ...
    }
    
    ...
    struct UserResult {
        #[field(column_name= "first_name", wrapper_fn ="str_upper", default_value="John")]
        name: String,
        ...
    }
    
````