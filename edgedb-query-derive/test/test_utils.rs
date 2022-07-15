use edgedb_protocol::codec::ObjectShape;

pub fn check_shape(shape: &ObjectShape, expected_elements: Vec<&str>) {
    let elements = &shape.elements;

    let vars = elements.iter()
        .map(|elmt| elmt.name.clone())
        .collect::<Vec<String>>();

    assert_eq!(vars, expected_elements);
}