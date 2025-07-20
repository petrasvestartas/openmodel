use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use uuid::Uuid;
use std::fmt;

/// Enhanced Data struct that combines metadata and serialization capabilities
/// Similar to COMPAS Data class for all serializable geometric objects
#[derive(Debug, Clone)]
pub struct Data {
    /// Object name (32 bytes max)
    name: [u8; 32],
    /// Unique identifier
    guid: Uuid,
}

// Custom serialization to make JSON more readable
impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Data", 2)?;
        state.serialize_field("name", self.name())?; // Serialize as string
        state.serialize_field("guid", &self.guid)?;
        state.end()
    }
}

// Custom deserialization to handle both string and byte array formats
impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Guid,
        }

        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Data")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Data, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name: Option<String> = None;
                let mut guid: Option<Uuid> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            // Handle both string and byte array formats
                            let value = map.next_value::<serde_json::Value>()?;
                            match value {
                                serde_json::Value::String(s) => {
                                    name = Some(s);
                                }
                                serde_json::Value::Array(arr) => {
                                    // Convert byte array back to string for backward compatibility
                                    let mut bytes = [0u8; 32];
                                    for (i, val) in arr.iter().enumerate().take(32) {
                                        if let Some(byte_val) = val.as_u64() {
                                            bytes[i] = byte_val as u8;
                                        }
                                    }
                                    name = Some(Data::array_to_string(&bytes).to_string());
                                }
                                _ => return Err(de::Error::custom("name must be string or byte array")),
                            }
                        }
                        Field::Guid => {
                            guid = Some(map.next_value()?);
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let guid = guid.ok_or_else(|| de::Error::missing_field("guid"))?;

                Ok(Data {
                    name: Data::string_to_array(&name),
                    guid,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "guid"];
        deserializer.deserialize_struct("Data", FIELDS, DataVisitor)
    }
}

impl Data {
    /// Create a new Data instance with given name
    /// 
    /// # Panics
    /// 
    /// Panics if the name is longer than 32 bytes.
    pub fn with_name(name: &str) -> Self {
        let name_bytes = name.as_bytes();
        assert!(name_bytes.len() <= 32, "Name is too long");
        Self {
            name: Self::string_to_array(name),
            guid: Uuid::new_v4(),
        }
    }
    
    /// Create a new Data instance with default name
    pub fn new() -> Self {
        Self::with_name("")
    }
    
    /// Get the object's GUID
    pub fn guid(&self) -> Uuid {
        self.guid
    }
    
    /// Get the object's name
    pub fn name(&self) -> &str {
        Self::array_to_string(&self.name)
    }
    
    /// Set the object's name
    /// 
    /// # Panics
    /// 
    /// Panics if the name is longer than 32 bytes.
    pub fn set_name(&mut self, name: &str) {
        let name_bytes = name.as_bytes();
        assert!(name_bytes.len() <= 32, "Name is too long");
        self.name = Self::string_to_array(name);
    }
    
    /// Convert string to fixed-size array
    fn string_to_array(s: &str) -> [u8; 32] {
        let mut array = [0; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }
    
    /// Convert fixed-size array to string
    fn array_to_string(array: &[u8; 32]) -> &str {
        let end = array.iter().position(|&b| b == 0).unwrap_or(array.len());
        std::str::from_utf8(&array[..end]).unwrap_or("")
    }
    
    /// Create a structured JSON representation similar to COMPAS
    pub fn to_json_data(&self, dtype: &'static str, data: Value, minimal: bool) -> serde_json::Value {
        if minimal {
            serde_json::json!({
                "dtype": dtype,
                "data": data
            })
        } else {
            serde_json::json!({
                "dtype": dtype,
                "data": data,
                "guid": self.guid,
                "name": self.name()
            })
        }
    }
    
    /// Create a copy of the data with optional GUID copying
    pub fn copy(&self, copy_guid: bool) -> Self {
        Self {
            name: self.name,
            guid: if copy_guid { self.guid } else { Uuid::new_v4() },
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data {{ name: {}, guid: {} }}", self.name(), self.guid)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

/// Test module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_creation() {
        let data = Data::new();
        assert_eq!(data.name(), "");
        assert!(!data.guid().is_nil());
    }
    
    #[test]
    fn test_data_with_name() {
        let data = Data::with_name("test");
        assert_eq!(data.name(), "test");
    }
    
    #[test]
    fn test_set_name() {
        let mut data = Data::new();
        data.set_name("modified");
        assert_eq!(data.name(), "modified");
    }
    
    #[test]
    fn test_to_json_data() {
        let data = Data::with_name("test");
        let json = data.to_json_data("test_type", serde_json::json!({"value": 42}), false);
        assert_eq!(json["dtype"], "test_type");
        assert_eq!(json["data"]["value"], 42);
        assert_eq!(json["name"], "test");
    }
}