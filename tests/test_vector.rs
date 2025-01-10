use openmodel::vector::Vector;

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