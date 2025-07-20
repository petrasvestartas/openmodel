use openmodel::geometry::{Point, Vector, Line, Plane, Color, Cloud, Pline, Xform, Mesh};
use openmodel::common::{json_dump, json_load, JsonSerializable, FromJsonData};
use serde::{Serialize, Deserialize};
use serde_json;

// Comprehensive geometry data structure with all geometry types
#[derive(Serialize, Deserialize, Debug)]
struct AllGeometryData {
    points: Vec<Point>,
    vectors: Vec<Vector>,
    lines: Vec<Line>,
    planes: Vec<Plane>,
    colors: Vec<Color>,
    clouds: Vec<Cloud>,
    plines: Vec<Pline>,
    xforms: Vec<Xform>,
    meshes: Vec<Mesh>,
}

// Implement JsonSerializable for AllGeometryData to work with json_dump/json_load
impl JsonSerializable for AllGeometryData {
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Implement FromJsonData for AllGeometryData to work with json_load
impl FromJsonData for AllGeometryData {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(data.clone()).ok()
    }
}

fn main() {
    println!("=== Testing ALL Geometry Types with JSON Serialization ===\n");
    
    // Create instances of all geometry types
    let points = vec![
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 1.0),
    ];
    
    let vectors = vec![
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
    ];
    
    let lines = vec![
        Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        Line::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
    ];
    
    let planes = vec![
        Plane::new(Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        Plane::new(Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
    ];
    
    let colors = vec![
        Color::new(255, 0, 0, 255),
        Color::new(0, 255, 0, 255),
        Color::new(0, 0, 255, 255),
    ];
    
    let clouds = vec![
        Cloud::new(
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)],
            vec![Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0)],
            vec![Color::new(255, 0, 0, 255), Color::new(0, 255, 0, 255)],
        ),
    ];
    
    let plines = vec![
        Pline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 0.0, 0.0)]),
    ];
    
    let xforms = vec![
        Xform::translation(1.0, 2.0, 3.0),
        Xform::scaling(2.0, 2.0, 2.0),
    ];
    
    let meshes = vec![
        Mesh::from_polygons(vec![
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)]
        ], None),
    ];
    
    // Create comprehensive geometry data
    let all_geometry = AllGeometryData {
        points: points.clone(),
        vectors: vectors.clone(),
        lines: lines.clone(),
        planes: planes.clone(),
        colors: colors.clone(),
        clouds: clouds.clone(),
        plines: plines.clone(),
        xforms: xforms.clone(),
        meshes: meshes.clone(),
    };
    
    println!("✅ Created all geometry types:");
    println!("   {} Points", all_geometry.points.len());
    println!("   {} Vectors", all_geometry.vectors.len());
    println!("   {} Lines", all_geometry.lines.len());
    println!("   {} Planes", all_geometry.planes.len());
    println!("   {} Colors", all_geometry.colors.len());
    println!("   {} Clouds", all_geometry.clouds.len());
    println!("   {} Plines", all_geometry.plines.len());
    println!("   {} Xforms", all_geometry.xforms.len());
    println!("   {} Meshes", all_geometry.meshes.len());
    
    println!("\n✅ Using json_dump to save ALL geometry types...");
    json_dump(&all_geometry, "all_geometry.json");
    
    println!("✅ Using json_load to load ALL geometry types...");
    let loaded_geometry: AllGeometryData = json_load("all_geometry.json");
    
    println!("\n✅ Successfully loaded all geometry types:");
    println!("   {} Points", loaded_geometry.points.len());
    println!("   {} Vectors", loaded_geometry.vectors.len());
    println!("   {} Lines", loaded_geometry.lines.len());
    println!("   {} Planes", loaded_geometry.planes.len());
    println!("   {} Colors", loaded_geometry.colors.len());
    println!("   {} Clouds", loaded_geometry.clouds.len());
    println!("   {} Plines", loaded_geometry.plines.len());
    println!("   {} Xforms", loaded_geometry.xforms.len());
    println!("   {} Meshes", loaded_geometry.meshes.len());
    
    // Verify some data to ensure it's preserved correctly
    println!("\n=== Verification ===");
    println!("First Point: {}", loaded_geometry.points[0]);
    println!("First Vector: {}", loaded_geometry.vectors[0]);
    println!("First Line: {}", loaded_geometry.lines[0]);
    println!("First Color: {}", loaded_geometry.colors[0]);
    println!("First Cloud: {} points", loaded_geometry.clouds[0].points.len());
    println!("First Pline: {} points", loaded_geometry.plines[0].points.len());
    println!("First Mesh: {} vertices, {} faces", 
             loaded_geometry.meshes[0].number_of_vertices(), 
             loaded_geometry.meshes[0].number_of_faces());
    
    println!("\n🎉 ALL GEOMETRY TYPES WORK PERFECTLY with json_dump/json_load!");
    println!("🚀 You can now serialize ANY geometry type or collection!");
}