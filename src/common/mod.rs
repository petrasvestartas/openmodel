pub mod data;
pub mod json_serialization;
pub mod universal_json;

// Re-export commonly used types and functions
pub use data::Data;
pub use json_serialization::{JsonSerializable, JsonData, json_dump, json_load, HasJsonData, FromJsonData};
pub use universal_json::{UniversalValue, universal_json_dump, universal_json_load};