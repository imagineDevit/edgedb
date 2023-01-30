insert users::User {
    name := <str>$user_name,
    age := <int16>$age,
    friend := (
        select users::User {
            name,
            age,
        }
        filter .name = <str>$friend_name
    )
}