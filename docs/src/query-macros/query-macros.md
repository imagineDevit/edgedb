# Query macros

[**Edgedb-query-derive**](https://github.com/imagineDevit/edgedb) crate provide 5 types of query macros :

- [**_InsertQuery_**](./insert-query.md)
- [**_SelectQuery_**](./select-query.md)
- [**_UpdateQuery_**]()
- [**_DeleteQuery_**]()
- [**_CustomQuery_**]()

Each of those derive macros generate under the hood implementations of the five following traits for the decorated struct:

* **ToEdgeQl**
* **ToEdgeValue**
* **ToEdgeQuery**
* **ToEdgeScalar**
* **ToString**