use openmodel::geometry::{Point, Vector, Line, Plane, Color, PointCloud, LineCloud, Pline, Mesh};
use openmodel::primitives::Xform;
use openmodel::common::{JsonSerializable, FromJsonData, HasJsonData};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::collections::HashMap;

// Comprehensive geometry data structure with all geometry types
#[derive(Serialize, Deserialize, Debug)]
struct AllGeometryData {
    points: Vec<Point>,
    vectors: Vec<Vector>,
    lines: Vec<Line>,
    planes: Vec<Plane>,
    colors: Vec<Color>,
    point_clouds: Vec<PointCloud>,
    line_clouds: Vec<LineCloud>,
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



// 
// fn main() {
//     println!("=== Testing ALL Geometry Types with JSON Serialization ===\n");
    
    // Create instances of all geometry types
//     let points = vec![
//         Point::new(1.0, 0.0, 0.0),
//         Point::new(0.0, 1.0, 0.0),
//         Point::new(0.0, 0.0, 1.0),
//     ];
    
//     let vectors = vec![
//         Vector::new(1.0, 0.0, 0.0),
//         Vector::new(0.0, 1.0, 0.0),
//         Vector::new(0.0, 0.0, 1.0),
//     ];
    
//     let lines = vec![
//         Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
//         Line::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
//     ];
    
//     let planes = vec![
//         Plane::from_point_and_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
//         Plane::from_point_and_normal(Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
//     ];
    
//     let colors = vec![
//         Color::new(255, 0, 0, 255),
//         Color::new(0, 255, 0, 255),
//         Color::new(0, 0, 255, 255),
//     ];
    
//     let point_clouds = vec![
//         PointCloud::new(
//             vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)],
//             vec![Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0)],
//             vec![Color::new(255, 0, 0, 255), Color::new(0, 255, 0, 255)],
//         ),
//     ];
    
//     let line_clouds = vec![
//         LineCloud::new(
//             vec![Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)],
//             vec![Color::new(255, 0, 0, 255)],
//         ),
//     ];
    
//     let plines = vec![
//         Pline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 0.0, 0.0)]),
//     ];
    
//     let xforms = vec![
//         Xform::translation(1.0, 2.0, 3.0),
//         Xform::scaling(2.0, 2.0, 2.0),
//     ];
    
//     let meshes = vec![
//         Mesh::from_polygons(vec![
//             vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)]
//         ], None),
//     ];
    
//     // Create comprehensive geometry data
//     let all_geometry = AllGeometryData {
//         points: points.clone(),
//         vectors: vectors.clone(),
//         lines: lines.clone(),
//         planes: planes.clone(),
//         colors: colors.clone(),
//         point_clouds: point_clouds.clone(),
//         line_clouds: line_clouds.clone(),
//         plines: plines.clone(),
//         xforms: xforms.clone(),
//         meshes: meshes.clone(),
//     };
    
//     println!("✅ Created all geometry types:");
//     println!("   {} Points", all_geometry.points.len());
//     println!("   {} Vectors", all_geometry.vectors.len());
//     println!("   {} Lines", all_geometry.lines.len());
//     println!("   {} Planes", all_geometry.planes.len());
//     println!("   {} Colors", all_geometry.colors.len());
//     println!("   {} Point Clouds", all_geometry.point_clouds.len());
//     println!("   {} Line Clouds", all_geometry.line_clouds.len());
//     println!("   {} Plines", all_geometry.plines.len());
//     println!("   {} Xforms", all_geometry.xforms.len());
//     println!("   {} Meshes", all_geometry.meshes.len());
    
//     println!("\n✅ Using json_dump to save ALL geometry types...");
//     json_dump(&all_geometry, "all_geometry.json");
    
//     println!("✅ Using json_load to load ALL geometry types...");
//     let loaded_geometry: AllGeometryData = json_load("all_geometry.json");
    
//     println!("\n✅ Successfully loaded all geometry types:");
//     println!("   {} Points", loaded_geometry.points.len());
//     println!("   {} Vectors", loaded_geometry.vectors.len());
//     println!("   {} Lines", loaded_geometry.lines.len());
//     println!("   {} Planes", loaded_geometry.planes.len());
//     println!("   {} Colors", loaded_geometry.colors.len());
//     println!("   {} Point Clouds", loaded_geometry.point_clouds.len());
//     println!("   {} Line Clouds", loaded_geometry.line_clouds.len());
//     println!("   {} Plines", loaded_geometry.plines.len());
//     println!("   {} Xforms", loaded_geometry.xforms.len());
//     println!("   {} Meshes", loaded_geometry.meshes.len());
    
//     // Verify some data to ensure it's preserved correctly
//     println!("\n=== Verification ===");
//     println!("First Point: {}", loaded_geometry.points[0]);
//     println!("First Vector: {}", loaded_geometry.vectors[0]);
//     println!("First Line: {}", loaded_geometry.lines[0]);
//     println!("First Color: {}", loaded_geometry.colors[0]);
//     println!("First Point Cloud: {} points", loaded_geometry.point_clouds[0].points.len());
//     println!("First Line Cloud: {} lines", loaded_geometry.line_clouds[0].lines.len());
//     println!("First Pline: {} points", loaded_geometry.plines[0].points.len());
//     println!("First Mesh: {} vertices, {} faces", 
//              loaded_geometry.meshes[0].number_of_vertices(), 
//              loaded_geometry.meshes[0].number_of_faces());
    
//     println!("\n🎉 ALL GEOMETRY TYPES WORK PERFECTLY with json_dump/json_load!");
//     println!("🚀 You can now serialize ANY geometry type or collection!");
// }


// use openmodel::geometry::{Point, Vector, Line, Color, PointCloud, LineCloud, Pline, Mesh};


fn main() {
    println!("=== Generating Geometry JSON using OpenModel API ===\n");
    
    // Create sample cube mesh using openmodel
    let mut cube_mesh = Mesh::new();
    cube_mesh.data.set_name("sample_cube");
    
    // Add cube vertices
    let v0 = cube_mesh.add_vertex(Point::new(-0.5, -0.5, 0.5), None);
    let v1 = cube_mesh.add_vertex(Point::new(0.5, -0.5, 0.5), None);
    let v2 = cube_mesh.add_vertex(Point::new(0.5, 0.5, 0.5), None);
    let v3 = cube_mesh.add_vertex(Point::new(-0.5, 0.5, 0.5), None);
    let v4 = cube_mesh.add_vertex(Point::new(-0.5, -0.5, -0.5), None);
    let v5 = cube_mesh.add_vertex(Point::new(-0.5, 0.5, -0.5), None);
    let v6 = cube_mesh.add_vertex(Point::new(0.5, 0.5, -0.5), None);
    let v7 = cube_mesh.add_vertex(Point::new(0.5, -0.5, -0.5), None);
    
    // Add cube faces
    cube_mesh.add_face(vec![v0, v1, v2], None); // Front face 1
    cube_mesh.add_face(vec![v0, v2, v3], None); // Front face 2
    cube_mesh.add_face(vec![v4, v5, v6], None); // Back face 1
    cube_mesh.add_face(vec![v4, v6, v7], None); // Back face 2
    
    // Create octahedron mesh using openmodel
    let mut octahedron_mesh = Mesh::new();
    octahedron_mesh.data.set_name("octahedron");
    
    let o0 = octahedron_mesh.add_vertex(Point::new(4.0, 0.0, 0.0), None);
    let o1 = octahedron_mesh.add_vertex(Point::new(2.0, 0.0, 0.0), None);
    let o2 = octahedron_mesh.add_vertex(Point::new(3.0, 1.0, 0.0), None);
    let o3 = octahedron_mesh.add_vertex(Point::new(3.0, -1.0, 0.0), None);
    let o4 = octahedron_mesh.add_vertex(Point::new(3.0, 0.0, 1.0), None);
    let o5 = octahedron_mesh.add_vertex(Point::new(3.0, 0.0, -1.0), None);
    
    octahedron_mesh.add_face(vec![o0, o2, o4], None);
    octahedron_mesh.add_face(vec![o2, o1, o4], None);
    octahedron_mesh.add_face(vec![o1, o3, o4], None);
    octahedron_mesh.add_face(vec![o3, o0, o4], None);
    
    // Create sample points using openmodel
    let mut sample_points = PointCloud::new(
        vec![
            Point::new(0.0, -5.0, 0.0),
            Point::new(1.0, -5.0, 0.0),
            Point::new(2.0, -5.0, 0.0),
            Point::new(3.0, -5.0, 0.0),
            Point::new(4.0, -5.0, 0.0),
        ],
        vec![], // No normals
        vec![
            Color::new(255, 0, 0, 255),   // Red
            Color::new(0, 255, 0, 255),   // Green
            Color::new(0, 0, 255, 255),   // Blue
            Color::new(255, 255, 0, 255), // Yellow
            Color::new(0, 0, 0, 255),     // Black
        ],
    );
    sample_points.data.set_name("sample_points");
    
    // Create sample lines using openmodel
    let mut sample_lines = LineCloud::new(
        vec![
            Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            Line::new(1.0, 0.0, 0.0, 1.0, 1.0, 0.0),
            Line::new(1.0, 1.0, 0.0, 0.0, 1.0, 0.0),
            Line::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
        ],
        vec![
            Color::new(255, 0, 0, 255),   // Red
            Color::new(0, 255, 0, 255),   // Green
            Color::new(0, 0, 255, 255),   // Blue
            Color::new(255, 255, 0, 255), // Yellow
        ],
    );
    sample_lines.data.set_name("sample_lines");
    
    // Create sample pipes using openmodel (as pipe meshes)
    let pipe1 = Mesh::create_pipe(
        Point::new(0.0, 5.0, 0.0),
        Point::new(1.0, 5.0, 0.0),
        0.05
    );
    
    let pipe2 = Mesh::create_pipe(
        Point::new(1.0, 5.0, 0.0),
        Point::new(1.0, 5.0, 1.0),
        0.05
    );
    
    // Create sample pline using openmodel
    let mut sample_pline = Pline::new(vec![
        Point::new(-3.0, 0.0, 0.0),
        Point::new(-4.0, 0.0, 0.0),
        Point::new(-4.0, 1.0, 0.0),
        Point::new(-3.0, 1.0, 0.0),
    ]);
    sample_pline.data.set_name("sample_pline");
    
    // Create a collection of all geometry using HashMap
    let mut geometry_collection = HashMap::new();
    
    // Add meshes (using to_json_data for automatic dtype)
    geometry_collection.insert("cube_mesh".to_string(), cube_mesh.to_json_data(false));
    geometry_collection.insert("octahedron_mesh".to_string(), octahedron_mesh.to_json_data(false));
    geometry_collection.insert("pipe1_mesh".to_string(), pipe1.to_json_data(false));
    geometry_collection.insert("pipe2_mesh".to_string(), pipe2.to_json_data(false));
    
    // Add point cloud (using to_json_data for automatic dtype)
    geometry_collection.insert("sample_points".to_string(), sample_points.to_json_data(false));
    
    // Add line cloud (using to_json_data for automatic dtype)
    geometry_collection.insert("sample_lines".to_string(), sample_lines.to_json_data(false));
    
    // Add pline (using to_json_data for automatic dtype)
    geometry_collection.insert("sample_pline".to_string(), sample_pline.to_json_data(false));
    
    // Add metadata
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), serde_json::Value::String("1.0".to_string()));
    metadata.insert("description".to_string(), serde_json::Value::String("Sample geometry data generated with openmodel API".to_string()));
    metadata.insert("created".to_string(), serde_json::Value::String("2025-07-28".to_string()));
    geometry_collection.insert("metadata".to_string(), serde_json::Value::Object(metadata.into_iter().collect()));
    
    // Serialize to JSON
    let json_string = serde_json::to_string_pretty(&geometry_collection)
        .expect("Failed to serialize geometry data");
    
    // Write to file
    fs::write("geometry_data.json", &json_string)
        .expect("Failed to write JSON file");
    
    println!("✅ Generated geometry_data.json using pure openmodel API!");
    println!("📊 Contains {} geometry objects", geometry_collection.len());
    println!("🎯 All geometry created using openmodel types: Mesh, PointCloud, LineCloud, Pline");
    println!("🔧 Used optimized create_pipe method for pipe generation");
}
