use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::{HasJsonData, FromJsonData};
use crate::geometry::{Point, Vector};

/// A geometry object that can hold different types - enables mixed collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeometryObject {
    Point(Point),
    Vector(Vector),
}

impl GeometryObject {
    /// Create from Point
    pub fn from_point(point: Point) -> Self {
        GeometryObject::Point(point)
    }
    
    /// Create from Vector  
    pub fn from_vector(vector: Vector) -> Self {
        GeometryObject::Vector(vector)
    }
    
    /// Get the type name for this geometry object
    pub fn type_name(&self) -> &'static str {
        match self {
            GeometryObject::Point(_) => "Point",
            GeometryObject::Vector(_) => "Vector",
        }
    }
}

/// Support for mixed-type JSON serialization
impl HasJsonData for GeometryObject {
    fn to_json_data(&self, minimal: bool) -> Value {
        match self {
            GeometryObject::Point(p) => p.to_json_data(minimal),
            GeometryObject::Vector(v) => v.to_json_data(minimal),
        }
    }
}

/// Support for mixed-type JSON deserialization using dtype field
impl FromJsonData for GeometryObject {
    fn from_json_data(data: &Value) -> Option<Self> {
        // Use COMPAS-style dtype field to determine type
        let dtype = data["dtype"].as_str()?;
        
        match dtype {
            "openmodel.geometry/Point" => {
                Point::from_json_data(data).map(GeometryObject::Point)
            },
            "openmodel.geometry/Vector" => {
                Vector::from_json_data(data).map(GeometryObject::Vector)
            },
            _ => None,
        }
    }
}

impl From<Point> for GeometryObject {
    fn from(point: Point) -> Self {
        GeometryObject::Point(point)
    }
}

impl From<Vector> for GeometryObject {
    fn from(vector: Vector) -> Self {
        GeometryObject::Vector(vector)
    }
}
