# Queries attribute macros

[**Edgedb-query-derive**](https://github.com/imagineDevit/edgedb) crate provide 5 macro attributes that represent a edgeDB query:

- [#[insert_query]](./insert-query.md)
- [#[select_query]](./select-query.md)
- [#[update_query]](./update-query.md)
- [#[delete_query]](./delete-query.md)
- [#[file_query]](./delete-query.md)

Queries attributes (except #[file_query] ) take three arguments 👇

| Argument | Optional | Description                                                                                                                                               |
|----------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| module   | yes      | The name of the edgeDB module on which the query is executed.<br> <br/> _**By default**_: 'default'                                                       |
| table    | no       | The name of the edgeDB table on which the query is executed.<br>                                                                                          |
| result   | yes      | The query result type.<br> <br/>_**By default**_: [BasicResult](https://github.com/imagineDevit/edgedb/blob/main/edgedb-query/src/models/query_result.rs) | 

