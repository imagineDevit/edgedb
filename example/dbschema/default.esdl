module default {

    type Person {
        required property name -> str;
        property places_visited -> array<str>;
    }

    type City {
        required property name -> str;
        property modern_name -> str;
    }

}
