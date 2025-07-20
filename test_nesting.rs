use openmodel::geometry::{Point, Vector};
use openmodel::common::{json_dump, json_load, JsonSerializable, FromJsonData};
use serde::{Serialize, Deserialize};

// Test different nesting levels
#[derive(Serialize, Deserialize, Debug)]
struct SingleNested {
    points: Vec<Point>,
    vectors: Vec<Vector>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DoubleNested {
    point_groups: Vec<Vec<Point>>,
    vector_groups: Vec<Vec<Vector>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TripleNested {
    point_collections: Vec<Vec<Vec<Point>>>,
    vector_collections: Vec<Vec<Vec<Vector>>>,
}

// Implement traits for SingleNested
impl JsonSerializable for SingleNested {
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

impl FromJsonData for SingleNested {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(data.clone()).ok()
    }
}

// Implement traits for DoubleNested
impl JsonSerializable for DoubleNested {
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

impl FromJsonData for DoubleNested {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(data.clone()).ok()
    }
}

// Implement traits for TripleNested
impl JsonSerializable for TripleNested {
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

impl FromJsonData for TripleNested {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(data.clone()).ok()
    }
}

fn main() {
    println!("=== Testing Different Nesting Levels ===\n");

    // 1. Single nested: Vec<Point>
    println!("1. Single Nested (Vec<Point>):");
    let single = SingleNested {
        points: vec![Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
        vectors: vec![Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)],
    };
    
    json_dump(&single, "single_nested.json");
    let loaded_single: SingleNested = json_load("single_nested.json");
    println!("✅ Single nested works! Loaded {} points, {} vectors\n", 
             loaded_single.points.len(), loaded_single.vectors.len());

    // 2. Double nested: Vec<Vec<Point>> (current GeometryData)
    println!("2. Double Nested (Vec<Vec<Point>>):");
    let double = DoubleNested {
        point_groups: vec![
            vec![Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0)],
        ],
        vector_groups: vec![
            vec![Vector::new(1.0, 0.0, 0.0)],
            vec![Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0)],
        ],
    };
    
    json_dump(&double, "double_nested.json");
    let loaded_double: DoubleNested = json_load("double_nested.json");
    println!("✅ Double nested works! Loaded {} point groups, {} vector groups\n", 
             loaded_double.point_groups.len(), loaded_double.vector_groups.len());

    // 3. Triple nested: Vec<Vec<Vec<Point>>>
    println!("3. Triple Nested (Vec<Vec<Vec<Point>>>):");
    let triple = TripleNested {
        point_collections: vec![
            vec![
                vec![Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
                vec![Point::new(0.0, 1.0, 0.0)],
            ],
            vec![
                vec![Point::new(3.0, 0.0, 0.0)],
            ],
        ],
        vector_collections: vec![
            vec![
                vec![Vector::new(1.0, 0.0, 0.0)],
            ],
        ],
    };
    
    json_dump(&triple, "triple_nested.json");
    let loaded_triple: TripleNested = json_load("triple_nested.json");
    println!("✅ Triple nested works! Loaded {} point collections, {} vector collections\n", 
             loaded_triple.point_collections.len(), loaded_triple.vector_collections.len());

    // Show some data to verify
    println!("=== Verification ===");
    println!("Single: First point = {}", loaded_single.points[0]);
    println!("Double: First point group has {} points", loaded_double.point_groups[0].len());
    println!("Triple: First collection has {} point groups", loaded_triple.point_collections[0].len());
}
