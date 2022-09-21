module default {

    type Person {
        required property name -> str;
        multi link places_visited -> City;
    }

    type City {
        required property name -> str;
        property modern_name -> str;
    }

}
