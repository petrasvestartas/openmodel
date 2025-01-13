use openmodel::geometry::Vector;

#[test]
fn test_new() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_length() {
    let v = Vector::new(1.0, 2.0, 2.0);
    assert_eq!(v.length(), 3.0);
}

#[test]
fn test_default() {
    let v = Vector::default();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_vector_serialization() {
    let v = Vector::new(1.0, 2.0, 3.0);

    // Serialize the vector to a JSON string
    let serialized = serde_json::to_string(&v).unwrap();


    // Deserialize the JSON string back to a vector
    let deserialized: Vector = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, v);
    assert_eq!(deserialized.x, v.x);
    assert_eq!(deserialized.y, v.y);
    assert_eq!(deserialized.z, v.z);
    assert_eq!(deserialized.data.name, v.data.name);
    assert_eq!(deserialized.data.guid, v.data.guid);
}
