use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;
use std::str;

const MAX_NAME_LEN: usize = 32;

/// A struct representing metadata associated with a geometric object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    /// The name of the object as a fixed-size array.
    pub name: [u8; MAX_NAME_LEN],
    /// The globally unique identifier (GUID) of the object.
    pub guid: Uuid,
}

impl Data {
    /// Creates a new `Data` instance with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the object.
    ///
    /// # Returns
    ///
    /// A `Data` instance with the specified name and a newly generated GUID.
    ///
    /// # Panics
    ///
    /// Panics if the name is longer than `MAX_NAME_LEN`.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::common::Data;
    /// let data = Data::new("MyObject");
    /// ```
    pub fn new(name: &str) -> Self {
        let mut name_array = [0u8; MAX_NAME_LEN];
        let name_bytes = name.as_bytes();
        assert!(name_bytes.len() <= MAX_NAME_LEN, "Name is too long");
        name_array[..name_bytes.len()].copy_from_slice(name_bytes);

        Data {
            name: name_array,
            guid: Uuid::new_v4(),
        }
    }

    /// Returns the name as a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::common::Data;
    /// let data = Data::new("MyObject");
    /// assert_eq!(data.name(), "MyObject");
    /// ```
    pub fn name(&self) -> &str {
        let end = self.name.iter().position(|&c| c == 0).unwrap_or(MAX_NAME_LEN);
        str::from_utf8(&self.name[..end]).unwrap()
    }

    /// Creates a new `Data` instance with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name of the object.
    ///
    /// # Returns
    ///
    /// A `Data` instance with the specified name and a newly generated GUID.
    ///
    /// # Panics
    ///
    /// Panics if the name is longer than `MAX_NAME_LEN`.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::common::Data;
    /// let data = Data::with_name("MyObject");
    /// ```
    pub fn with_name(name: &str) -> Self {
        Self::new(name)
    }
}

impl Default for Data {
    /// Creates a default `Data` instance with the name "data".
    ///
    /// # Returns
    ///
    /// A `Data` instance with the name "data" and a newly generated GUID.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::common::Data;
    /// let data = Data::default();
    /// ```
    fn default() -> Self {
        Data::new("data")
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data {{ name: {}, guid: {} }}", self.name(), self.guid)
    }
}