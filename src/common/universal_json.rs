use serde_json::{Value, Map};
use std::collections::HashMap;
use crate::geometry::{Point, Vector, GeometryObject};
use crate::common::{FromJsonData, HasJsonData};

/// Universal JSON value that can contain embedded geometry objects at any depth
#[derive(Debug, Clone)]
pub enum UniversalValue {
    // Primitive types
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    
    // Collection types (can contain geometry objects)
    Array(Vec<UniversalValue>),
    Object(HashMap<String, UniversalValue>),
    
    // Embedded geometry objects
    Geometry(GeometryObject),
}

impl UniversalValue {
    /// Convert from any serde_json::Value, detecting embedded geometry objects
    pub fn from_json_value(value: &Value) -> Self {
        match value {
            Value::Null => UniversalValue::Null,
            Value::Bool(b) => UniversalValue::Bool(*b),
            Value::Number(n) => UniversalValue::Number(n.as_f64().unwrap_or(0.0)),
            Value::String(s) => UniversalValue::String(s.clone()),
            
            Value::Array(arr) => {
                let universal_arr: Vec<UniversalValue> = arr.iter()
                    .map(|v| UniversalValue::from_json_value(v))
                    .collect();
                UniversalValue::Array(universal_arr)
            },
            
            Value::Object(obj) => {
                // Check if this is a geometry object by looking for dtype
                if let Some(dtype) = obj.get("dtype").and_then(|v| v.as_str()) {
                    if dtype.starts_with("openmodel.geometry/") {
                        // Try to deserialize as geometry object
                        if let Some(geo_obj) = GeometryObject::from_json_data(value) {
                            return UniversalValue::Geometry(geo_obj);
                        }
                    }
                }
                
                // Otherwise, treat as regular object
                let mut universal_obj = HashMap::new();
                for (key, val) in obj {
                    universal_obj.insert(key.clone(), UniversalValue::from_json_value(val));
                }
                UniversalValue::Object(universal_obj)
            }
        }
    }
    
    /// Convert back to serde_json::Value for serialization
    pub fn to_json_value(&self) -> Value {
        match self {
            UniversalValue::Null => Value::Null,
            UniversalValue::Bool(b) => Value::Bool(*b),
            UniversalValue::Number(n) => Value::Number(serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0))),
            UniversalValue::String(s) => Value::String(s.clone()),
            
            UniversalValue::Array(arr) => {
                let json_arr: Vec<Value> = arr.iter()
                    .map(|v| v.to_json_value())
                    .collect();
                Value::Array(json_arr)
            },
            
            UniversalValue::Object(obj) => {
                let mut json_obj = Map::new();
                for (key, val) in obj {
                    json_obj.insert(key.clone(), val.to_json_value());
                }
                Value::Object(json_obj)
            },
            
            UniversalValue::Geometry(geo) => {
                geo.to_json_data(false)
            }
        }
    }
    
    /// Get nested value by path (like "data.points.0.x")
    pub fn get_nested(&self, path: &str) -> Option<&UniversalValue> {
        let parts: Vec<&str> = path.split('.').collect();
        self.get_nested_parts(&parts)
    }
    
    fn get_nested_parts(&self, parts: &[&str]) -> Option<&UniversalValue> {
        if parts.is_empty() {
            return Some(self);
        }
        
        match self {
            UniversalValue::Object(obj) => {
                if let Some(value) = obj.get(parts[0]) {
                    value.get_nested_parts(&parts[1..])
                } else {
                    None
                }
            },
            UniversalValue::Array(arr) => {
                if let Ok(index) = parts[0].parse::<usize>() {
                    if let Some(value) = arr.get(index) {
                        value.get_nested_parts(&parts[1..])
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => None
        }
    }
    
    /// Convert to specific geometry type if possible
    pub fn as_point(&self) -> Option<&Point> {
        match self {
            UniversalValue::Geometry(GeometryObject::Point(p)) => Some(p),
            _ => None
        }
    }
    
    pub fn as_vector(&self) -> Option<&Vector> {
        match self {
            UniversalValue::Geometry(GeometryObject::Vector(v)) => Some(v),
            _ => None
        }
    }
}

/// Universal JSON dump - handles arbitrary nesting with embedded geometry
pub fn universal_json_dump(data: &UniversalValue, path: &str) {
    let json_value = data.to_json_value();
    match serde_json::to_string_pretty(&json_value) {
        Ok(json_str) => {
            if let Err(e) = std::fs::write(path, json_str) {
                eprintln!("Warning: Failed to write {}: {}", path, e);
            }
        },
        Err(e) => {
            eprintln!("Warning: Failed to serialize to JSON: {}", e);
        }
    }
}

/// Universal JSON load - handles arbitrary nesting with embedded geometry  
pub fn universal_json_load(path: &str) -> UniversalValue {
    let json_str = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to read {}: {}", path, e);
            return UniversalValue::Object(HashMap::new());
        }
    };
    
    let json_value: Value = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Warning: Failed to parse JSON from {}: {}", path, e);
            return UniversalValue::Object(HashMap::new());
        }
    };
    
    UniversalValue::from_json_value(&json_value)
}
