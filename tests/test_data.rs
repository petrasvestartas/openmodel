use openmodel::common::Data;

#[test]
fn test_data_creation() {
    let data = Data::with_name("MyObject");
    
    assert!(data.guid().is_nil() == false);
    assert_eq!(data.name(), "MyObject");
}

#[test]
#[should_panic(expected = "Name is too long")]
fn test_data_creation_with_long_name() {
    let _data = Data::with_name("ThisNameIsWayTooLongForTheFixedSizeArray");
}