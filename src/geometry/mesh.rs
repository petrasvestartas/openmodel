use crate::geometry::Point;
use crate::geometry::Vector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A simple mesh data structure for representing surface meshes.
/// Based on the COMPAS mesh implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Vertex data: maps vertex key to vertex attributes (including position)
    vertices: HashMap<usize, VertexData>,
    /// Face data: maps face key to list of vertex keys
    faces: HashMap<usize, Vec<usize>>,
    /// Face attributes: maps face key to face attributes
    face_attributes: HashMap<usize, HashMap<String, f64>>,
    /// Edge attributes: maps edge tuple to edge attributes
    edge_attributes: HashMap<(usize, usize), HashMap<String, f64>>,
    /// Next available vertex key
    max_vertex: usize,
    /// Next available face key
    max_face: usize,
}

/// Vertex data containing position and attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    /// 3D position of the vertex
    pub position: Point,
    /// Custom attributes for the vertex
    pub attributes: HashMap<String, f64>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    /// Create a new empty mesh.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Mesh;
    /// let mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// assert!(mesh.is_empty());
    /// ```
    pub fn new() -> Self {
        Mesh {
            vertices: HashMap::new(),
            faces: HashMap::new(),
            face_attributes: HashMap::new(),
            edge_attributes: HashMap::new(),
            max_vertex: 0,
            max_face: 0,
        }
    }

    /// Create a mesh from a collection of polygons.
    /// 
    /// Each polygon is defined as a list of XYZ coordinates of its corners.
    /// The method automatically handles vertex sharing and connectivity.
    /// 
    /// # Arguments
    /// * `polygons` - A list of polygons, where each polygon is a list of Points
    /// * `precision` - Optional precision for vertex merging (default: 1e-10)
    /// 
    /// # Returns
    /// A new Mesh object constructed from the polygons
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// 
    /// let polygons = vec![
    ///     // Triangle 1
    ///     vec![
    ///         Point::new(0.0, 0.0, 0.0),
    ///         Point::new(1.0, 0.0, 0.0),
    ///         Point::new(0.0, 1.0, 0.0),
    ///     ],
    ///     // Triangle 2 (shares an edge with triangle 1)
    ///     vec![
    ///         Point::new(1.0, 0.0, 0.0),
    ///         Point::new(1.0, 1.0, 0.0),
    ///         Point::new(0.0, 1.0, 0.0),
    ///     ],
    /// ];
    /// 
    /// let mesh = Mesh::from_polygons(polygons, None);
    /// assert_eq!(mesh.number_of_vertices(), 4); // Shared vertices are merged
    /// assert_eq!(mesh.number_of_faces(), 2);
    /// ```
    pub fn from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f64>) -> Self {
        let mut mesh = Mesh::new();
        let precision = precision.unwrap_or(1e-10);
        
        // Map to store unique vertices and their keys
        let mut vertex_map: HashMap<String, usize> = HashMap::new();
        
        for polygon in polygons {
            if polygon.len() < 3 {
                continue; // Skip invalid polygons
            }
            
            let mut face_vertices = Vec::new();
            
            for point in polygon {
                // Create a key for the point based on its coordinates with precision
                let key = format!(
                    "{:.10}_{:.10}_{:.10}", 
                    (point.x / precision).round() * precision,
                    (point.y / precision).round() * precision,
                    (point.z / precision).round() * precision
                );
                
                let vertex_key = if let Some(&existing_key) = vertex_map.get(&key) {
                    // Vertex already exists, reuse it
                    existing_key
                } else {
                    // New vertex, add it to the mesh
                    let vertex_key = mesh.add_vertex(point, None);
                    vertex_map.insert(key, vertex_key);
                    vertex_key
                };
                
                face_vertices.push(vertex_key);
            }
            
            // Add the face to the mesh
            mesh.add_face(face_vertices, None);
        }
        
        mesh
    }

    /// Create a new empty mesh (deprecated alias for new).
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Mesh;
    /// let mesh = Mesh::create();
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// assert!(mesh.is_empty());
    /// ```
    #[deprecated(note = "Use Mesh::new() instead")]
    pub fn create() -> Self {
        Mesh {
            vertices: HashMap::new(),
            faces: HashMap::new(),
            face_attributes: HashMap::new(),
            edge_attributes: HashMap::new(),
            max_vertex: 0,
            max_face: 0,
        }
    }

    /// Add a vertex to the mesh.
    /// 
    /// # Arguments
    /// * `position` - The 3D position of the vertex
    /// * `key` - Optional specific key for the vertex. If None, auto-generates.
    /// 
    /// # Returns
    /// The key of the added vertex
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// assert_eq!(v1, 0);
    /// assert_eq!(mesh.number_of_vertices(), 1);
    /// ```
    pub fn add_vertex(&mut self, position: Point, key: Option<usize>) -> usize {
        let vertex_key = key.unwrap_or_else(|| {
            let k = self.max_vertex;
            self.max_vertex += 1;
            k
        });
        
        self.vertices.insert(vertex_key, VertexData {
            position,
            attributes: HashMap::new(),
        });
        
        if vertex_key >= self.max_vertex {
            self.max_vertex = vertex_key + 1;
        }
        
        vertex_key
    }

    /// Add a face to the mesh.
    /// 
    /// # Arguments
    /// * `vertices` - List of vertex keys that form the face
    /// * `key` - Optional specific key for the face. If None, auto-generates.
    /// 
    /// # Returns
    /// The key of the added face, or None if the face is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face = mesh.add_face(vec![v1, v2, v3], None);
    /// assert!(face.is_some());
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn add_face(&mut self, vertices: Vec<usize>, key: Option<usize>) -> Option<usize> {
        // Check if all vertices exist and face has at least 3 vertices
        if vertices.len() < 3 || !vertices.iter().all(|v| self.vertices.contains_key(v)) {
            return None;
        }
        
        // Check for duplicate vertices in the face
        let mut unique_vertices = std::collections::HashSet::new();
        for vertex in &vertices {
            if !unique_vertices.insert(*vertex) {
                return None; // Duplicate vertex found
            }
        }
        
        let face_key = key.unwrap_or_else(|| {
            let k = self.max_face;
            self.max_face += 1;
            k
        });
        
        self.faces.insert(face_key, vertices);
        self.face_attributes.insert(face_key, HashMap::new());
        
        if face_key >= self.max_face {
            self.max_face = face_key + 1;
        }
        
        Some(face_key)
    }

    /// Get the number of vertices in the mesh.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert_eq!(mesh.number_of_vertices(), 1);
    /// ```
    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of faces in the mesh.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn number_of_faces(&self) -> usize {
        self.faces.len()
    }

    /// Get the number of edges in the mesh.
    /// Each edge is counted once, even if it's shared by multiple faces.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// assert_eq!(mesh.number_of_edges(), 3);
    /// ```
    pub fn number_of_edges(&self) -> usize {
        let mut edges = std::collections::HashSet::new();
        
        for face_vertices in self.faces.values() {
            for i in 0..face_vertices.len() {
                let v1 = face_vertices[i];
                let v2 = face_vertices[(i + 1) % face_vertices.len()];
                let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                edges.insert(edge);
            }
        }
        
        edges.len()
    }

    /// Calculate Euler characteristic (V - E + F).
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// // For a single triangle: V=3, E=3, F=1, so V-E+F = 1
    /// assert_eq!(mesh.euler(), 1);
    /// ```
    pub fn euler(&self) -> i32 {
        self.number_of_vertices() as i32 - self.number_of_edges() as i32 + self.number_of_faces() as i32
    }

    /// Get vertex position by key.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// let pos = mesh.vertex_position(v1).unwrap();
    /// assert_eq!(pos.x, 1.0);
    /// assert_eq!(pos.y, 2.0);
    /// assert_eq!(pos.z, 3.0);
    /// ```
    pub fn vertex_position(&self, key: usize) -> Option<&Point> {
        self.vertices.get(&key).map(|v| &v.position)
    }

    /// Get face vertices by key.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
    /// let vertices = mesh.face_vertices(face).unwrap();
    /// assert_eq!(vertices.len(), 3);
    /// assert!(vertices.contains(&v1));
    /// ```
    pub fn face_vertices(&self, key: usize) -> Option<&Vec<usize>> {
        self.faces.get(&key)
    }

    /// Get all vertex keys.
    pub fn vertices(&self) -> impl Iterator<Item = usize> + '_ {
        self.vertices.keys().copied()
    }

    /// Get all face keys.
    pub fn faces(&self) -> impl Iterator<Item = usize> + '_ {
        self.faces.keys().copied()
    }

    /// Set vertex attribute.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert!(mesh.set_vertex_attribute(v1, "weight", 1.5));
    /// assert_eq!(mesh.get_vertex_attribute(v1, "weight"), Some(1.5));
    /// ```
    pub fn set_vertex_attribute(&mut self, vertex: usize, name: &str, value: f64) -> bool {
        if let Some(vertex_data) = self.vertices.get_mut(&vertex) {
            vertex_data.attributes.insert(name.to_string(), value);
            true
        } else {
            false
        }
    }

    /// Get vertex attribute.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// mesh.set_vertex_attribute(v1, "weight", 2.5);
    /// assert_eq!(mesh.get_vertex_attribute(v1, "weight"), Some(2.5));
    /// assert_eq!(mesh.get_vertex_attribute(v1, "nonexistent"), None);
    /// ```
    pub fn get_vertex_attribute(&self, vertex: usize, name: &str) -> Option<f64> {
        self.vertices.get(&vertex)
            .and_then(|v| v.attributes.get(name))
            .copied()
    }

    /// Set face attribute.
    pub fn set_face_attribute(&mut self, face: usize, name: &str, value: f64) -> bool {
        if self.faces.contains_key(&face) {
            self.face_attributes.entry(face)
                .or_insert_with(HashMap::new)
                .insert(name.to_string(), value);
            true
        } else {
            false
        }
    }

    /// Get face attribute.
    pub fn get_face_attribute(&self, face: usize, name: &str) -> Option<f64> {
        self.face_attributes.get(&face)
            .and_then(|attrs| attrs.get(name))
            .copied()
    }

    /// Check if the mesh is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// assert!(mesh.is_empty());
    /// mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert!(!mesh.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Clear all mesh data.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert!(!mesh.is_empty());
    /// mesh.clear();
    /// assert!(mesh.is_empty());
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.faces.clear();
        self.face_attributes.clear();
        self.edge_attributes.clear();
        self.max_vertex = 0;
        self.max_face = 0;
    }

    /// Compute the normal vector of a face.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face to compute the normal for
    /// 
    /// # Returns
    /// The unit normal vector of the face, or None if the face doesn't exist or has invalid geometry
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
    /// let normal = mesh.face_normal(face).unwrap();
    /// // Normal should point in +Z direction for this triangle
    /// assert!((normal.z - 1.0).abs() < 1e-10);
    /// ```
    pub fn face_normal(&self, face_key: usize) -> Option<Vector> {
        let face_vertices = self.faces.get(&face_key)?;
        
        if face_vertices.len() < 3 {
            return None;
        }
        
        // Get the first three vertices to compute the normal
        let v0 = self.vertex_position(face_vertices[0])?;
        let v1 = self.vertex_position(face_vertices[1])?;
        let v2 = self.vertex_position(face_vertices[2])?;
        
        // Create vectors from v0 to v1 and v0 to v2
        let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        
        // Compute cross product to get normal
        let mut normal = edge1.cross(&edge2);
        
        // Unitize the normal vector
        if normal.unitize() {
            Some(normal)
        } else {
            None // Degenerate face (zero area)
        }
    }

    /// Compute the normal vector of a vertex by averaging adjacent face normals.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex to compute the normal for
    /// 
    /// # Returns
    /// The unit normal vector of the vertex, or None if the vertex doesn't exist or has no adjacent faces
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// let normal = mesh.vertex_normal(v1).unwrap();
    /// // Vertex normal should be a unit vector
    /// assert!((normal.length() - 1.0).abs() < 1e-10);
    /// ```
    pub fn vertex_normal(&self, vertex_key: usize) -> Option<Vector> {
        if !self.vertices.contains_key(&vertex_key) {
            return None;
        }
        
        let mut normal_sum = Vector::new(0.0, 0.0, 0.0);
        let mut face_count = 0;
        
        // Find all faces that contain this vertex
        for (face_key, face_vertices) in &self.faces {
            if face_vertices.contains(&vertex_key) {
                if let Some(face_normal) = self.face_normal(*face_key) {
                    normal_sum += &face_normal;
                    face_count += 1;
                }
            }
        }
        
        if face_count == 0 {
            return None;
        }
        
        // Average the normal
        normal_sum /= face_count as f64;
        
        // Unitize the averaged normal
        if normal_sum.unitize() {
            Some(normal_sum)
        } else {
            None
        }
    }

    /// Compute all face normals and return them as a HashMap.
    /// 
    /// # Returns
    /// A HashMap mapping face keys to their normal vectors
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// let normals = mesh.face_normals();
    /// assert_eq!(normals.len(), 1);
    /// ```
    pub fn face_normals(&self) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        
        for face_key in self.faces.keys() {
            if let Some(normal) = self.face_normal(*face_key) {
                normals.insert(*face_key, normal);
            }
        }
        
        normals
    }

    /// Compute all vertex normals and return them as a HashMap.
    /// 
    /// # Returns
    /// A HashMap mapping vertex keys to their normal vectors
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v1, v2, v3], None);
    /// let normals = mesh.vertex_normals();
    /// assert_eq!(normals.len(), 3);
    /// ```
    pub fn vertex_normals(&self) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        
        for vertex_key in self.vertices.keys() {
            if let Some(normal) = self.vertex_normal(*vertex_key) {
                normals.insert(*vertex_key, normal);
            }
        }
        
        normals
    }

    /// Compute the area of a face.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face to compute the area for
    /// 
    /// # Returns
    /// The area of the face, or None if the face doesn't exist or has invalid geometry
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
    /// let area = mesh.face_area(face).unwrap();
    /// // Area of a triangle with base=1, height=1 should be 0.5
    /// assert!((area - 0.5).abs() < 1e-10);
    /// ```
    pub fn face_area(&self, face_key: usize) -> Option<f64> {
        let face_vertices = self.faces.get(&face_key)?;
        
        if face_vertices.len() < 3 {
            return None;
        }
        
        // For triangular faces, use cross product magnitude / 2
        if face_vertices.len() == 3 {
            let v0 = self.vertex_position(face_vertices[0])?;
            let v1 = self.vertex_position(face_vertices[1])?;
            let v2 = self.vertex_position(face_vertices[2])?;
            
            let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
            let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
            
            let cross = edge1.cross(&edge2);
            Some(cross.length() * 0.5)
        } else {
            // For polygonal faces, triangulate and sum areas
            let mut total_area = 0.0;
            let v0 = self.vertex_position(face_vertices[0])?;
            
            for i in 1..face_vertices.len() - 1 {
                let v1 = self.vertex_position(face_vertices[i])?;
                let v2 = self.vertex_position(face_vertices[i + 1])?;
                
                let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                
                let cross = edge1.cross(&edge2);
                total_area += cross.length() * 0.5;
            }
            
            Some(total_area)
        }
    }
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mesh {{ vertices: {}, faces: {}, edges: {} }}",
            self.number_of_vertices(),
            self.number_of_faces(),
            self.number_of_edges()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mesh = Mesh::new();
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert_eq!(mesh.number_of_edges(), 0);
        assert_eq!(mesh.euler(), 0);
        assert!(mesh.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        
        // Test adding vertex with auto-generated key
        let v1 = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
        assert_eq!(v1, 0);
        assert_eq!(mesh.number_of_vertices(), 1);
        assert!(!mesh.is_empty());
        
        // Test adding vertex with specific key
        let v2 = mesh.add_vertex(Point::new(4.0, 5.0, 6.0), Some(10));
        assert_eq!(v2, 10);
        assert_eq!(mesh.number_of_vertices(), 2);
        
        // Test vertex position retrieval
        let pos1 = mesh.vertex_position(v1).unwrap();
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 2.0);
        assert_eq!(pos1.z, 3.0);
        
        let pos2 = mesh.vertex_position(v2).unwrap();
        assert_eq!(pos2.x, 4.0);
        assert_eq!(pos2.y, 5.0);
        assert_eq!(pos2.z, 6.0);
        
        // Test non-existent vertex
        assert!(mesh.vertex_position(999).is_none());
    }

    #[test]
    fn test_add_vertices() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(v1, 0);
        assert_eq!(v2, 1);
        assert_eq!(v3, 2);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let face = mesh.add_face(vec![v1, v2, v3], None);
        assert!(face.is_some());
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
    }

    #[test]
    fn test_euler_characteristic() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v1, v2, v3], None);
        
        // For a single triangle: V=3, E=3, F=1, so V-E+F = 3-3+1 = 1
        assert_eq!(mesh.euler(), 1);
    }

    #[test]
    fn test_vertex_attributes() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        
        assert!(mesh.set_vertex_attribute(v1, "weight", 1.5));
        assert_eq!(mesh.get_vertex_attribute(v1, "weight"), Some(1.5));
        assert_eq!(mesh.get_vertex_attribute(v1, "nonexistent"), None);
    }

    #[test]
    fn test_face_normal() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
        let normal = mesh.face_normal(face).unwrap();
        
        // Normal should point in +Z direction for this triangle
        assert!((normal.x).abs() < 1e-10);
        assert!((normal.y).abs() < 1e-10);
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!((normal.length() - 1.0).abs() < 1e-10); // Should be unit vector
    }

    #[test]
    fn test_vertex_normal() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v4 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        
        // Create two triangular faces sharing vertex v2
        mesh.add_face(vec![v1, v2, v3], None);
        mesh.add_face(vec![v2, v4, v3], None);
        
        let normal = mesh.vertex_normal(v2).unwrap();
        
        // Vertex normal should be in +Z direction (average of two +Z face normals)
        assert!((normal.x).abs() < 1e-10);
        assert!((normal.y).abs() < 1e-10);
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!((normal.length() - 1.0).abs() < 1e-10); // Should be unit vector
    }

    #[test]
    fn test_face_area() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
        let area = mesh.face_area(face).unwrap();
        
        // Area of a triangle with base=1, height=1 should be 0.5
        assert!((area - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_face_normals_batch() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v4 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        
        let f1 = mesh.add_face(vec![v1, v2, v3], None).unwrap();
        let f2 = mesh.add_face(vec![v2, v4, v3], None).unwrap();
        
        let normals = mesh.face_normals();
        
        assert_eq!(normals.len(), 2);
        assert!(normals.contains_key(&f1));
        assert!(normals.contains_key(&f2));
        
        // Both normals should point in +Z direction
        for normal in normals.values() {
            assert!((normal.z - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_vertex_normals_batch() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v1, v2, v3], None);
        
        let normals = mesh.vertex_normals();
        
        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));
        assert!(normals.contains_key(&v3));
        
        // All vertex normals should point in +Z direction
        for normal in normals.values() {
            assert!((normal.z - 1.0).abs() < 1e-10);
            assert!((normal.length() - 1.0).abs() < 1e-10); // Should be unit vectors
        }
    }

    #[test]
    fn test_face_vertices() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let face = mesh.add_face(vec![v1, v2, v3], None).unwrap();
        let vertices = mesh.face_vertices(face).unwrap();
        
        assert_eq!(vertices.len(), 3);
        assert!(vertices.contains(&v1));
        assert!(vertices.contains(&v2));
        assert!(vertices.contains(&v3));
        
        // Test non-existent face
        assert!(mesh.face_vertices(999).is_none());
    }

    #[test]
    fn test_number_of_edges() {
        let mut mesh = Mesh::new();
        assert_eq!(mesh.number_of_edges(), 0);
        
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v4 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        
        // Add first triangle
        mesh.add_face(vec![v1, v2, v3], None);
        assert_eq!(mesh.number_of_edges(), 3);
        
        // Add second triangle sharing an edge
        mesh.add_face(vec![v2, v4, v3], None);
        assert_eq!(mesh.number_of_edges(), 5); // 3 + 2 new edges (shared edge not double-counted)
    }

    #[test]
    fn test_clear_and_empty() {
        let mut mesh = Mesh::new();
        assert!(mesh.is_empty());
        
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v1, v2, v3], None);
        
        assert!(!mesh.is_empty());
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(mesh.number_of_faces(), 1);
        
        mesh.clear();
        assert!(mesh.is_empty());
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert_eq!(mesh.number_of_edges(), 0);
    }

    #[test]
    fn test_invalid_face_creation() {
        let mut mesh = Mesh::new();
        let v1 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        
        // Test face with too few vertices
        assert!(mesh.add_face(vec![v1, v2], None).is_none());
        
        // Test face with non-existent vertex
        assert!(mesh.add_face(vec![v1, v2, 999], None).is_none());
        
        // Test face with duplicate vertices
        assert!(mesh.add_face(vec![v1, v1, v2], None).is_none());
    }

    #[test]
    fn test_error_cases() {
        let mut mesh = Mesh::new();
        
        // Test normal computation on non-existent faces
        assert!(mesh.face_normal(999).is_none());
        assert!(mesh.vertex_normal(999).is_none());
        assert!(mesh.face_area(999).is_none());
        
        // Test attribute operations on non-existent vertex
        assert!(!mesh.set_vertex_attribute(999, "test", 1.0));
        assert!(mesh.get_vertex_attribute(999, "test").is_none());
    }
}