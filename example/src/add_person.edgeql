insert default::Person {
    name := <str>$name,
    places_visited := (
        insert default::City {
            name := <str>$city_name,
        }
    )
}