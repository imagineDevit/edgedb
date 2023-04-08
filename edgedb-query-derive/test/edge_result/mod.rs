
#[cfg(test)]
mod result {
    use edgedb_query_derive::query_result;
    use edgedb_query::{EdgeResult, ToEdgeShape};
    use uuid::Uuid;


    #[query_result]
    pub struct Identity {
        pub id: Uuid,
        pub name: String,
        pub age: i16
    }

    #[query_result]
    pub struct Friend {
        pub id: Uuid,
        pub surname: String
    }

    #[query_result]
    pub struct User {
        pub id: Uuid,
        #[field(link_property=true)]
        pub login: String,
        pub identity: Identity,
    }

    #[query_result]
    pub struct UserWithFriends {
        pub id: Uuid,
        pub login: String,
        pub identity: Identity,
        #[back_link(
            module="users",
            source_table="User",
            target_table="Friend",
            target_column="friend",
        )]
        pub friends: Vec<Friend>,
    }

    #[query_result]
    pub struct UserWithFriend {
        pub id: Uuid,
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_shape() {
        let shape = Identity::shape();
        let fields = Identity::returning_fields();
        assert_eq!(shape, "{id,name,age}");
        assert_eq!(fields, vec!["id","name", "age"]);
    }

    #[test]
    pub fn test_nested_shape() {
        let shape = User::shape();
        let fields = User::returning_fields();

        assert_eq!(shape, "{id,@login,identity : {id,name,age}}");
        assert_eq!(fields, vec!["id","login", "identity"]);
    }

    #[test]
    pub fn test_nested_query_shape_vec() {
        let shape = UserWithFriends::shape();
        let fields = UserWithFriends::returning_fields();
        assert_eq!(shape, "{id,login,identity : {id,name,age},friends := (select users::User.<friend[is users::Friend]{id,surname})}");
        assert_eq!(fields, vec!["id","login", "identity", "friends"]);
    }

    #[test]
    pub fn test_nested_query_shape() {
        let shape = UserWithFriend::shape();
        assert_eq!(shape, "{id,login,identity : {id,name,age},friend := (select users::User.<friend[is users::Friend]{id,surname} limit 1)}")
    }


    #[query_result]
    pub struct UserWithFriendAndWrapperFn {
        pub id: Uuid,
        #[field(wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",

        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_wrapperfn() {
        let shape = UserWithFriendAndWrapperFn::shape();
        assert_eq!(shape, "{id,login := (select <str>str_upper(.login)),identity : {id,name,age},friend := (select users::User.<friend[is users::Friend]{id,surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithFriendAndField {
        pub id: Uuid,
        #[field(column_name="pseudo")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field() {
        let shape = UserWithFriendAndField::shape();
        assert_eq!(shape, "{id,login := .pseudo,identity : {id,name,age},friend := (select users::User.<friend[is users::Friend]{id,surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithFriendAndFieldAndWrapperFn {
        pub id: Uuid,
        #[field(column_name="pseudo", wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field_and_wrapper() {
        let shape = UserWithFriendAndFieldAndWrapperFn::shape();
        assert_eq!(shape, "{id,login := (select <str>str_upper(.pseudo)),identity : {id,name,age},friend := (select users::User.<friend[is users::Friend]{id,surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithDefault {
        pub id: Uuid,
        #[field(column_name="pseudo", wrapper_fn="str_upper", default_value="john")]
        pub login: String,
    }

    #[test]
    pub fn test_with_default() {
        let shape = UserWithDefault::shape();
        assert_eq!(shape, "{id,login := (select <str>str_upper(.pseudo)) ?? (select <str>'john')}")
    }
}