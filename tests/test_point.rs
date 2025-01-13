use openmodel::geometry::Point;

#[test]
fn test_new() {
    let p = Point::new(1.0, 2.0, 3.0);
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
}

#[test]
fn test_distance() {
    let p1 = Point::new(1.0, 2.0, 2.0);
    let p2 = Point::new(4.0, 6.0, 6.0);
    assert_eq!(p1.distance(&p2), 6.4031242374328485);
}

#[test]
fn test_translate() {
    let mut p = Point::new(1.0, 2.0, 3.0);
    p.translate(1.0, 2.0, 3.0);
    assert_eq!(p.x, 2.0);
    assert_eq!(p.y, 4.0);
    assert_eq!(p.z, 6.0);
}

#[test]
fn test_translated() {
    let p = Point::new(1.0, 2.0, 3.0);
    let p2 = p.translated(1.0, 2.0, 3.0);
    assert_eq!(p2.x, 2.0);
    assert_eq!(p2.y, 4.0);
    assert_eq!(p2.z, 6.0);
}

#[test]
fn test_default() {
    let p = Point::default();
    assert_eq!(p.x, 0.0);
    assert_eq!(p.y, 0.0);
    assert_eq!(p.z, 0.0);
}

#[test]
fn test_operators() {
    let p = Point::new(5.0, 2.4, 3.0);
    assert_eq!(p[0], 5.0);
    assert_eq!(p[1], 2.4);
    assert_eq!(p[2], 3.0);
}

#[test]
fn test_point_serialization() {
    let p = Point::new(1.0, 2.0, 3.0);

    // Serialize the vector to a JSON string
    let serialized = serde_json::to_string(&p).unwrap();
    assert_eq!(serialized, r#"{"x":1.0,"y":2.0,"z":3.0}"#);

    // Deserialize the JSON string back to a vector
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, p);
}
