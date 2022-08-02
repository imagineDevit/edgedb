# InsertQuery

#### List of possibles attributes :

<table>
    <thead>
        <tr>
            <th>Attributes</th>
            <th>Options</th>
            <th>Optional (default value) </th>
            <th>Meaning </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=2> <strong style="color: #008200">meta</strong> </td>
            <td><i style="color: yellow">module</i></td>
            <td>Yes ("default") </td>
            <td>The edgedb module name </td>
        </tr>
        <tr>
            <td><i style="color: yellow">table</i></td>
            <td>No</td>
            <td>The edgedb table name </td>
        </tr>
        <tr>
            <td rowspan=3> <strong style="color: #008200">scalar</strong> </td>
            <td><i style="color: yellow">type</i></td>
            <td>No</td>
            <td>The field scalar type </td>
        </tr>
        <tr>
            <td><i style="color: yellow">module</i></td>
            <td>Yes ("default")</td>
            <td>The scalar type module </td>
        </tr>
         <tr>
            <td><i style="color: yellow">name</i></td>
            <td>No when type = "enum"</td>
            <td>The enum name </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">query</strong> </td>
            <td><i style="color: yellow">result</i></td>
            <td>Yes ("BasicResult") </td>
            <td>The struct representing the expected result shape </td>
        </tr>
        <tr>
            <td> <strong style="color: #008200">nested_query</strong> </td>
            <td></td>
            <td>Yes</td>
            <td> Indicates if the field references an edgedb query</td>
        </tr>
    </tbody>
</table>
<br>


**InsertQuery** derive macro generates a insert edgeql query.

Let's see how to use it:

Consider the following edgeDB schema 👇

```sql
   module models {
        type Person {
            required property user_name -> str;
            required property age -> int16;
            required link address -> Address;
        }
           
        type Address {
            required property nber -> int16;
            required property street -> str;
            required property city -> str;
            required property zipcode -> int32;
        }
   }
```

```rust

    #[derive(InsertQuery)]
    pub struct InsertPerson {
        #[meta(module="models", table="Person")]
        __meta__: (),
        pub user_name: String,
        pub age: u8,
        #[nested_query]
        pub address: InsertAddress,
        
    }
    
    #[derive(InsertQuery)]
    pub struct InsertAddress {
        #[meta(module="models", table="Address")]
        __meta__: (),
        pub number: u16,
        pub street: String,
        pub city: String,
        pub zipcode: u32,
    }

    
```