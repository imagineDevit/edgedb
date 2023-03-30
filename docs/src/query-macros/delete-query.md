# DeleteQuery

        #[delete_query(module, table)] {
            #[field]
            #[filter]
            #[and_filter]
            #[or_filter]
            #[filters]
        }

**_delete_query_** attribute macro indicates that the struct represents an edgeDB delete query.

---

### Usage

Consider the following edgeDB schema 👇

```sql
    module models { 
           type Person {
                required property user_name -> str;
                required property age -> int16;
           }
    }
```

To perform a 'delete query' using edgedb-tokio we can write code as follows 👇

```rust
    
    #[delete_query(module="models", table="Person")]
    pub struct DeletePerson {
        
        #[field(column_name="user_name")]
        #[filter(operator="Is")]
        pub name: String
    }

    #[tokio::main]
    async fn main() -> Result<()> {
        let client = edgedb_tokio::create_client().await?;

        let del_person = DeletePerson {
            name: "Mark".to_owned()
        };

        let edge_query: EdgeQuery = del_person.to_edge_query();

        let args = &edge_query.args.unwrap();

        let query = edge_query.query.as_str();

        let _= client.query_json(query, args).await?;
        
        
    }

```
