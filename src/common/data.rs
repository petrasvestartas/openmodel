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
    /// Parent element's guid (optional, empty for top-level elements)
    parent: Option<Uuid>,
    /// List of guids of adjacent elements (optional, mostly empty)
    adjacency_indices: Vec<Uuid>,
    /// List of strings representing adjacency types
    adjacency_types: Vec<String>,
    /// Transformation as a flattened 4x4 matrix (column-major, 16 f64 values)
    /// Default is identity matrix
    transformation: [f64; 16],
}

// Custom serialization to make JSON more readable
impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Data", 6)?;
        state.serialize_field("name", self.name())?; // Serialize as string
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("adjacency_indices", &self.adjacency_indices)?;
        state.serialize_field("adjacency_types", &self.adjacency_types)?;
        state.serialize_field("transformation", &self.transformation)?;
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
            Parent,
            AdjacencyIndices,
            AdjacencyTypes,
            Transformation,
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
                let mut parent: Option<Option<Uuid>> = None;
                let mut adjacency_indices: Option<Vec<Uuid>> = None;
                let mut adjacency_types: Option<Vec<String>> = None;
                let mut transformation: Option<[f64; 16]> = None;

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
                        Field::Parent => {
                            parent = Some(map.next_value()?);
                        }
                        Field::AdjacencyIndices => {
                            adjacency_indices = Some(map.next_value()?);
                        }
                        Field::AdjacencyTypes => {
                            adjacency_types = Some(map.next_value()?);
                        }
                        Field::Transformation => {
                            // Handle both array and vector formats
                            let value = map.next_value::<serde_json::Value>()?;
                            if let serde_json::Value::Array(arr) = value {
                                let mut matrix = [0.0; 16];
                                for (i, val) in arr.iter().enumerate().take(16) {
                                    if let Some(num_val) = val.as_f64() {
                                        matrix[i] = num_val;
                                    }
                                }
                                transformation = Some(matrix);
                            } else {
                                return Err(de::Error::custom("transformation must be an array of 16 numbers"));
                            }
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let guid = guid.ok_or_else(|| de::Error::missing_field("guid"))?;
                let parent = parent.unwrap_or(None);
                let adjacency_indices = adjacency_indices.unwrap_or_else(Vec::new);
                let adjacency_types = adjacency_types.unwrap_or_else(Vec::new);
                let transformation = transformation.unwrap_or_else(Data::identity_matrix);

                Ok(Data {
                    name: Data::string_to_array(&name),
                    guid,
                    parent,
                    adjacency_indices,
                    adjacency_types,
                    transformation,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "guid", "parent", "adjacency_indices", "adjacency_types", "transformation"];
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
            parent: None,
            adjacency_indices: Vec::new(),
            adjacency_types: Vec::new(),
            transformation: Self::identity_matrix(),
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
    
    /// Get the parent GUID
    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }
    
    /// Set the parent GUID
    pub fn set_parent(&mut self, parent: Option<Uuid>) {
        self.parent = parent;
    }
    
    /// Get the adjacency indices
    pub fn adjacency_indices(&self) -> &[Uuid] {
        &self.adjacency_indices
    }
    
    /// Get the adjacency types
    pub fn adjacency_types(&self) -> &[String] {
        &self.adjacency_types
    }
    
    /// Add an adjacency relationship
    pub fn add_adjacency(&mut self, guid: Uuid, adjacency_type: &str) {
        self.adjacency_indices.push(guid);
        self.adjacency_types.push(adjacency_type.to_string());
    }
    
    /// Clear all adjacencies
    pub fn clear_adjacencies(&mut self) {
        self.adjacency_indices.clear();
        self.adjacency_types.clear();
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
    
    /// Create a 4x4 identity matrix flattened to 16 values (column-major)
    pub fn identity_matrix() -> [f64; 16] {
        [
            1.0, 0.0, 0.0, 0.0,  // First column
            0.0, 1.0, 0.0, 0.0,  // Second column
            0.0, 0.0, 1.0, 0.0,  // Third column
            0.0, 0.0, 0.0, 1.0,  // Fourth column
        ]
    }
    
    /// Get the transformation matrix
    pub fn transformation(&self) -> &[f64; 16] {
        &self.transformation
    }
    
    /// Set the transformation matrix
    pub fn set_transformation(&mut self, matrix: [f64; 16]) {
        self.transformation = matrix;
    }
    
    /// Reset transformation to identity matrix
    pub fn reset_transformation(&mut self) {
        self.transformation = Self::identity_matrix();
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
                "name": self.name(),
                "parent": self.parent,
                "adjacency_indices": self.adjacency_indices,
                "adjacency_types": self.adjacency_types,
                "transformation": self.transformation
            })
        }
    }
    
    /// Create a copy of the data with optional GUID copying
    pub fn copy(&self, copy_guid: bool) -> Self {
        Self {
            name: self.name,
            guid: if copy_guid { self.guid } else { Uuid::new_v4() },
            parent: self.parent,
            adjacency_indices: self.adjacency_indices.clone(),
            adjacency_types: self.adjacency_types.clone(),
            transformation: self.transformation,
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Data {{ name: {}, guid: {}, parent: {:?}, adjacencies: {}, has_transform: {} }}", 
            self.name(), 
            self.guid,
            self.parent,
            self.adjacency_indices.len(),
            !self.transformation.iter().enumerate().all(|(i, &val)| {
                (i % 5 == 0 && (val - 1.0).abs() < f64::EPSILON) || // Diagonal elements are 1.0
                (i % 5 != 0 && val.abs() < f64::EPSILON) // Non-diagonal elements are 0.0
            })
        )
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
        assert_eq!(data.parent(), None);
        assert_eq!(data.adjacency_indices().len(), 0);
        assert_eq!(data.adjacency_types().len(), 0);
        assert_eq!(data.transformation(), &Data::identity_matrix());
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
        assert!(json["adjacency_indices"].is_array());
        assert!(json["adjacency_types"].is_array());
        assert!(json["transformation"].is_array());
        assert_eq!(json["transformation"][0], 1.0); // First element of identity matrix
    }
    
    #[test]
    fn test_adjacency() {
        let mut data = Data::with_name("test");
        let other_guid = Uuid::new_v4();
        
        // Add adjacency
        data.add_adjacency(other_guid, "connected_to");
        assert_eq!(data.adjacency_indices().len(), 1);
        assert_eq!(data.adjacency_types().len(), 1);
        assert_eq!(data.adjacency_indices()[0], other_guid);
        assert_eq!(data.adjacency_types()[0], "connected_to");
        
        // Clear adjacencies
        data.clear_adjacencies();
        assert_eq!(data.adjacency_indices().len(), 0);
        assert_eq!(data.adjacency_types().len(), 0);
    }
    
    #[test]
    fn test_parent() {
        let mut data = Data::with_name("test");
        let parent_guid = Uuid::new_v4();
        
        // Initially no parent
        assert_eq!(data.parent(), None);
        
        // Set parent
        data.set_parent(Some(parent_guid));
        assert_eq!(data.parent(), Some(parent_guid));
        
        // Clear parent
        data.set_parent(None);
        assert_eq!(data.parent(), None);
    }
    
    #[test]
    fn test_transformation() {
        let mut data = Data::with_name("test");
        
        // Default should be identity matrix
        let identity = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        assert_eq!(data.transformation(), &identity);
        
        // Set custom transformation
        let custom_transform = [
            2.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0,
            1.0, 2.0, 3.0, 1.0,
        ];
        data.set_transformation(custom_transform);
        assert_eq!(data.transformation(), &custom_transform);
        
        // Reset to identity
        data.reset_transformation();
        assert_eq!(data.transformation(), &identity);
    }
}