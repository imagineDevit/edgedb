# Filter 

_**Filter**_ attribute represents a filter statement in a edgeDB query.


    #[filter(operator, wrapper_fn)]
    
When several filters are applied in a query, only the first filter can be represented by attribute #[filter].
The others filters should be decorated with #[and_filter] or #[or_filter].

    #[and_filter(operator, wrapper_fn)] 

    #[or_filter(operator, wrapper_fn)]


<br>

| Argument   | Optional | Description                                                                 |
|------------|----------|-----------------------------------------------------------------------------|
| operator   | no       | The filter operator.                                                        |
| wrapper_fn | yes      | The function to apply to the edgeDB column value before applying the filter |

<br>

The **_operator_** argument can take one of following values (the case does not matter):

- **exists**
- **notexists** or **!exists**
- **is** or **=**
- **isnot** or **!=**
- **like**
- **ilike**
- **in**
- **notIn**
- **greaterthan**  or  **>**
- **lesserthan** or **<**
- **greaterthanorequal** or **>=**
- **lesserthanorequal** or **<=**

### Usage 

```rust
    struct FindUserByNameAndAge {
        #[filter(operator="Is", wrapper_fn="str_lower")]
        pub name: String,
        #[and_filter(operator=">=")]
        pub age: i8 
    }
````
