# EdgedbEnum

### List of attributes

<table>
    <thead>
        <tr>
            <th>Attributes</th>
            <th>Optional</th>
            <th>Description </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> <strong style="color: #008200">value</strong> </td>
            <td>Yes</td>
            <td>The EdgeDB enum's corresponding value. If the attribute is missing, the Rust enum value is considered to be the same as the EdgeDB scalar type.</td>
        </tr>
    </tbody>
</table>
<br><br>

The following scalar enum types 👇

```sql
    scalar type Gender extending enum<Man, Woman>;
    scalar type Status extending enum<Opened, InProgress, Done, Closed>;
```

can then be represented by 👇 

```rust
    #[derive(EdgedbEnum)]
    pub enum Gender {
        #[value("Man")]
        Male,
        #[value("Woman")]
        Female,
    }

    #[derive(EdgedbEnum)]
    pub enum Status {
        Opened,
        InProgress,
        #[value("Done")]
        Finished,
        Closed
    }
```