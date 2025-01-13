use openmodel::geometry::Point;
use serde_json;
use std::fs::File;
use std::io::BufReader;

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

    // Serialize the point to a JSON string
    let serialized = serde_json::to_string(&p).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON string back to a point
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.x, p.x);
    assert_eq!(deserialized.y, p.y);
    assert_eq!(deserialized.z, p.z);
    assert_eq!(deserialized.data.name, p.data.name);
    assert_eq!(deserialized.data.guid, p.data.guid);

    // Serialize the point to a file
    let file = File::create("point.json").unwrap();
    serde_json::to_writer(file, &p).unwrap();

    // Deserialize the point from the file
    let file = File::open("point.json").unwrap();
    let reader = BufReader::new(file);
    let deserialized_from_file: Point = serde_json::from_reader(reader).unwrap();
    assert_eq!(deserialized_from_file.x, p.x);
    assert_eq!(deserialized_from_file.y, p.y);
    assert_eq!(deserialized_from_file.z, p.z);
    assert_eq!(deserialized_from_file.data.name, p.data.name);
    assert_eq!(deserialized_from_file.data.guid, p.data.guid);
}