use crate::geometry::{Point, Vector};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A halfedge mesh data structure for representing polygonal surfaces.
/// 
/// This implementation follows the COMPAS halfedge mesh design, where mesh
/// connectivity is stored using a halfedge data structure. Each edge is split
/// into two halfedges with opposite orientations, enabling efficient topological
/// queries and mesh operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Halfedge connectivity: halfedge[u][v] represents the halfedge from vertex u to vertex v
    pub halfedge: HashMap<usize, HashMap<usize, Option<usize>>>,
    /// Vertices: maps vertex key to vertex data
    pub vertex: HashMap<usize, VertexData>,
    /// Faces: maps face key to list of vertex keys in order
    pub face: HashMap<usize, Vec<usize>>,
    /// Face attributes: maps face key to face attributes
    pub facedata: HashMap<usize, HashMap<String, f64>>,
    /// Edge attributes: maps edge tuple to edge attributes  
    pub edgedata: HashMap<(usize, usize), HashMap<String, f64>>,
    /// Default vertex attributes
    pub default_vertex_attributes: HashMap<String, f64>,
    /// Default face attributes
    pub default_face_attributes: HashMap<String, f64>,
    /// Default edge attributes
    pub default_edge_attributes: HashMap<String, f64>,
    /// Next available vertex key
    max_vertex: usize,
    /// Next available face key
    max_face: usize,
}

/// Vertex data containing position and attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    /// 3D position of the vertex
    pub x: f64,
    pub y: f64, 
    pub z: f64,
    /// Custom attributes for the vertex
    pub attributes: HashMap<String, f64>,
}

impl VertexData {
    /// Create a new vertex from a Point
    pub fn new(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
            attributes: HashMap::new(),
        }
    }
    
    /// Get the position as a Point
    pub fn position(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
    
    /// Set the position from a Point
    pub fn set_position(&mut self, point: Point) {
        self.x = point.x;
        self.y = point.y;
        self.z = point.z;
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    /// Create a new empty halfedge mesh.
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
        let mut default_vertex_attributes = HashMap::new();
        default_vertex_attributes.insert("x".to_string(), 0.0);
        default_vertex_attributes.insert("y".to_string(), 0.0);
        default_vertex_attributes.insert("z".to_string(), 0.0);
        
        Mesh {
            halfedge: HashMap::new(),
            vertex: HashMap::new(),
            face: HashMap::new(),
            facedata: HashMap::new(),
            edgedata: HashMap::new(),
            default_vertex_attributes,
            default_face_attributes: HashMap::new(),
            default_edge_attributes: HashMap::new(),
            max_vertex: 0,
            max_face: 0,
        }
    }

    /// Check if the mesh is empty.
    /// 
    /// # Returns
    /// True if the mesh has no vertices and no faces
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Mesh;
    /// let mesh = Mesh::new();
    /// assert!(mesh.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.vertex.is_empty() && self.face.is_empty()
    }

    /// Clear the mesh, removing all vertices and faces.
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
    /// ```
    pub fn clear(&mut self) {
        self.halfedge.clear();
        self.vertex.clear();
        self.face.clear();
        self.facedata.clear();
        self.edgedata.clear();
        self.max_vertex = 0;
        self.max_face = 0;
    }

    /// Get the number of vertices in the mesh.
    /// 
    /// # Returns
    /// The total number of vertices
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
        self.vertex.len()
    }

    /// Get the number of faces in the mesh.
    /// 
    /// # Returns
    /// The total number of faces
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn number_of_faces(&self) -> usize {
        self.face.len()
    }

    /// Get the number of edges in the mesh.
    /// 
    /// # Returns
    /// The total number of edges (undirected)
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.number_of_edges(), 3);
    /// ```
    pub fn number_of_edges(&self) -> usize {
        let mut seen = HashSet::new();
        let mut count = 0;
        
        for u in self.halfedge.keys() {
            if let Some(neighbors) = self.halfedge.get(u) {
                for v in neighbors.keys() {
                    let edge = if u < v { (*u, *v) } else { (*v, *u) };
                    if seen.insert(edge) {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    /// Compute the Euler characteristic (V - E + F) of the mesh.
    /// 
    /// # Returns
    /// The Euler characteristic
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.euler(), 1); // V=3, E=3, F=1 -> 3-3+1=1
    /// ```
    pub fn euler(&self) -> i32 {
        let v = self.number_of_vertices() as i32;
        let e = self.number_of_edges() as i32;
        let f = self.number_of_faces() as i32;
        v - e + f
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
    /// let vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// assert_eq!(mesh.number_of_vertices(), 1);
    /// ```
    pub fn add_vertex(&mut self, position: Point, key: Option<usize>) -> usize {
        let vertex_key = key.unwrap_or_else(|| {
            self.max_vertex += 1;
            self.max_vertex
        });
        
        // Update max_vertex if explicit key is larger
        if vertex_key >= self.max_vertex {
            self.max_vertex = vertex_key + 1;
        }
        
        let vertex_data = VertexData::new(position);
        self.vertex.insert(vertex_key, vertex_data);
        
        // Initialize halfedge connectivity for this vertex
        self.halfedge.entry(vertex_key).or_insert_with(HashMap::new);
        
        vertex_key
    }

    /// Add a face to the mesh.
    /// 
    /// # Arguments
    /// * `vertices` - List of vertex keys defining the face in order
    /// * `fkey` - Optional specific key for the face. If None, auto-generates.
    /// 
    /// # Returns
    /// The key of the added face, or None if the face is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn add_face(&mut self, vertices: Vec<usize>, fkey: Option<usize>) -> Option<usize> {
        // Validate the face
        if vertices.len() < 3 {
            return None;
        }
        
        // Check that all vertices exist
        if !vertices.iter().all(|v| self.vertex.contains_key(v)) {
            return None;
        }
        
        // Check for duplicate vertices
        let mut unique_vertices = HashSet::new();
        for vertex in &vertices {
            if !unique_vertices.insert(*vertex) {
                return None; // Duplicate vertex found
            }
        }
        
        let face_key = fkey.unwrap_or_else(|| {
            self.max_face += 1;
            self.max_face
        });
        
        // Update max_face if explicit key is larger
        if face_key >= self.max_face {
            self.max_face = face_key + 1;
        }
        
        // Add the face
        self.face.insert(face_key, vertices.clone());
        
        // Update halfedge connectivity
        for i in 0..vertices.len() {
            let u = vertices[i];
            let v = vertices[(i + 1) % vertices.len()];
            
            // Ensure both vertices have halfedge entries
            self.halfedge.entry(u).or_insert_with(HashMap::new);
            self.halfedge.entry(v).or_insert_with(HashMap::new);
            
            // Set the halfedge from u to v to point to this face
            self.halfedge.get_mut(&u).unwrap().insert(v, Some(face_key));
            
            // Set the reverse halfedge from v to u (boundary halfedge if no face exists)
            if !self.halfedge.get(&v).unwrap().contains_key(&u) {
                self.halfedge.get_mut(&v).unwrap().insert(u, None);
            }
        }
        
        Some(face_key)
    }

    /// Get the position of a vertex.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// The position of the vertex, or None if vertex doesn't exist
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// let pos = mesh.vertex_position(v).unwrap();
    /// assert_eq!(pos.x, 1.0);
    /// assert_eq!(pos.y, 2.0);
    /// assert_eq!(pos.z, 3.0);
    /// ```
    pub fn vertex_position(&self, vertex_key: usize) -> Option<Point> {
        self.vertex.get(&vertex_key).map(|v| v.position())
    }

    /// Get the vertices of a face.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face
    /// 
    /// # Returns
    /// A list of vertex keys defining the face, or None if face doesn't exist
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let vertices = mesh.face_vertices(f).unwrap();
    /// assert_eq!(*vertices, vec![v0, v1, v2]);
    /// ```
    pub fn face_vertices(&self, face_key: usize) -> Option<&Vec<usize>> {
        self.face.get(&face_key)
    }

    /// Check if a vertex is on the boundary of the mesh.
    /// 
    /// A vertex is on the boundary if it has at least one incident halfedge
    /// that points to None (no face), indicating a boundary edge.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// True if the vertex is on the boundary
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert!(mesh.is_vertex_on_boundary(v0)); // All vertices of a single triangle are on boundary
    /// ```
    pub fn is_vertex_on_boundary(&self, vertex_key: usize) -> bool {
        if let Some(neighbors) = self.halfedge.get(&vertex_key) {
            for face_option in neighbors.values() {
                if face_option.is_none() {
                    return true; // This halfedge points to no face, so it's on the boundary
                }
            }
        }
        false
    }

    /// Get the neighbors of a vertex.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// A vector of vertex keys that are adjacent to the given vertex
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// let neighbors = mesh.vertex_neighbors(v0);
    /// assert_eq!(neighbors.len(), 2);
    /// assert!(neighbors.contains(&v1));
    /// assert!(neighbors.contains(&v2));
    /// ```
    pub fn vertex_neighbors(&self, vertex_key: usize) -> Vec<usize> {
        if let Some(neighbors) = self.halfedge.get(&vertex_key) {
            neighbors.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get all faces incident to a vertex.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// A vector of face keys that are incident to the given vertex
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let faces = mesh.vertex_faces(v0);
    /// assert_eq!(faces.len(), 1);
    /// assert_eq!(faces[0], f);
    /// ```
    pub fn vertex_faces(&self, vertex_key: usize) -> Vec<usize> {
        let mut faces = Vec::new();
        
        for (face_key, vertices) in &self.face {
            if vertices.contains(&vertex_key) {
                faces.push(*face_key);
            }
        }
        
        faces
    }

    /// Compute the normal vector of a face.
    /// 
    /// The normal is computed using the cross product of the first two edges of the face.
    /// For planar faces, this gives the unit normal vector.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face
    /// 
    /// # Returns
    /// The unit normal vector of the face, or None if the face is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point, Vector};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let normal = mesh.face_normal(f).unwrap();
    /// assert!((normal.z - 1.0).abs() < 1e-10); // Normal should point in +Z direction
    /// ```
    pub fn face_normal(&self, face_key: usize) -> Option<Vector> {
        let vertices = self.face.get(&face_key)?;
        if vertices.len() < 3 {
            return None;
        }
        
        let p0 = self.vertex_position(vertices[0])?;
        let p1 = self.vertex_position(vertices[1])?;
        let p2 = self.vertex_position(vertices[2])?;
        
        let edge1 = Vector::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
        let edge2 = Vector::new(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
        
        let mut normal = edge1.cross(&edge2);
        normal.unitize();
        Some(normal)
    }

    /// Compute the normal vector of a vertex.
    /// 
    /// The vertex normal is computed as the average of the normals of all faces
    /// incident to the vertex, weighted by face area.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// The unit normal vector of the vertex, or None if the vertex is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let normal = mesh.vertex_normal(v0).unwrap();
    /// assert!((normal.z - 1.0).abs() < 1e-10); // Normal should point in +Z direction
    /// ```
    pub fn vertex_normal(&self, vertex_key: usize) -> Option<Vector> {
        let faces = self.vertex_faces(vertex_key);
        if faces.is_empty() {
            return None;
        }
        
        let mut normal_sum = Vector::new(0.0, 0.0, 0.0);
        let mut total_area = 0.0;
        
        for face_key in faces {
            if let Some(face_normal) = self.face_normal(face_key) {
                let area = self.face_area(face_key).unwrap_or(0.0);
                normal_sum.x += face_normal.x * area;
                normal_sum.y += face_normal.y * area;
                normal_sum.z += face_normal.z * area;
                total_area += area;
            }
        }
        
        if total_area > 0.0 {
            normal_sum.x /= total_area;
            normal_sum.y /= total_area;
            normal_sum.z /= total_area;
            normal_sum.unitize();
            Some(normal_sum)
        } else {
            None
        }
    }

    /// Compute the area of a face.
    /// 
    /// For faces with more than 3 vertices, the area is computed by triangulating
    /// the face and summing the areas of the triangles.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face
    /// 
    /// # Returns
    /// The area of the face, or None if the face is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let area = mesh.face_area(f).unwrap();
    /// assert!((area - 0.5).abs() < 1e-10); // Area of triangle with base=1, height=1
    /// ```
    pub fn face_area(&self, face_key: usize) -> Option<f64> {
        let vertices = self.face.get(&face_key)?;
        if vertices.len() < 3 {
            return None;
        }
        
        let mut area = 0.0;
        
        // Triangulate the face and sum triangle areas
        for i in 1..vertices.len() - 1 {
            let p0 = self.vertex_position(vertices[0])?;
            let p1 = self.vertex_position(vertices[i])?;
            let p2 = self.vertex_position(vertices[i + 1])?;
            
            let edge1 = Vector::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
            let edge2 = Vector::new(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
            
            let cross = edge1.cross(&edge2);
            area += cross.length() * 0.5;
        }
        
        Some(area)
    }

    /// Compute normals for all faces in the mesh.
    /// 
    /// # Returns
    /// A HashMap mapping face keys to their normal vectors
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let normals = mesh.face_normals();
    /// assert_eq!(normals.len(), 1);
    /// assert!(normals.contains_key(&f));
    /// ```
    pub fn face_normals(&self) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        
        for face_key in self.face.keys() {
            if let Some(normal) = self.face_normal(*face_key) {
                normals.insert(*face_key, normal);
            }
        }
        
        normals
    }

    /// Compute normals for all vertices in the mesh.
    /// 
    /// # Returns
    /// A HashMap mapping vertex keys to their normal vectors
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let normals = mesh.vertex_normals();
    /// assert_eq!(normals.len(), 3);
    /// assert!(normals.contains_key(&v0));
    /// ```
    pub fn vertex_normals(&self) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        
        for vertex_key in self.vertex.keys() {
            if let Some(normal) = self.vertex_normal(*vertex_key) {
                normals.insert(*vertex_key, normal);
            }
        }
        
        normals
    }

    /// Create a halfedge mesh from a list of polygons.
    /// 
    /// Each polygon is defined by a list of 3D points. Vertices are merged
    /// based on coordinate precision to avoid duplicates.
    /// 
    /// # Arguments
    /// * `polygons` - List of polygons, each defined by a list of 3D points
    /// * `precision` - Precision for merging vertices (default: 1e-10)
    /// 
    /// # Returns
    /// A new halfedge mesh constructed from the polygons
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::{Mesh, Point};
    /// let triangle = vec![
    ///     Point::new(0.0, 0.0, 0.0),
    ///     Point::new(1.0, 0.0, 0.0),
    ///     Point::new(0.0, 1.0, 0.0),
    /// ];
    /// let mesh = Mesh::from_polygons(vec![triangle], None);
    /// assert_eq!(mesh.number_of_vertices(), 3);
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f64>) -> Self {
        let precision = precision.unwrap_or(1e-10);
        let mut mesh = Mesh::new();
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
                
                // Check if vertex already exists
                let vertex_key = if let Some(&existing_key) = vertex_map.get(&key) {
                    existing_key
                } else {
                    // Add new vertex
                    let new_key = mesh.add_vertex(point, None);
                    vertex_map.insert(key, new_key);
                    new_key
                };
                
                face_vertices.push(vertex_key);
            }
            
            // Add the face if it has valid vertices
            if face_vertices.len() >= 3 {
                mesh.add_face(face_vertices, None);
            }
        }
        
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_halfedge_mesh_new() {
        let mesh = Mesh::new();
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert!(mesh.is_empty());
        assert_eq!(mesh.euler(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
        assert_eq!(mesh.number_of_vertices(), 1);
        assert!(!mesh.is_empty());
        
        let pos = mesh.vertex_position(vertex_key).unwrap();
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_add_vertex_with_specific_key() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Some(42));
        assert_eq!(vertex_key, 42);
        assert_eq!(mesh.number_of_vertices(), 1);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
        assert_eq!(mesh.euler(), 1); // V=3, E=3, F=1 -> 3-3+1=1
    }

    #[test]
    fn test_add_face_invalid() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        
        // Too few vertices
        assert!(mesh.add_face(vec![v0, v1], None).is_none());
        
        // Non-existent vertex
        assert!(mesh.add_face(vec![v0, v1, 999], None).is_none());
        
        // Duplicate vertices
        assert!(mesh.add_face(vec![v0, v1, v0], None).is_none());
    }

    #[test]
    fn test_face_vertices() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let vertices = mesh.face_vertices(f).unwrap();
        assert_eq!(vertices, &vec![v0, v1, v2]);
    }

    #[test]
    fn test_vertex_neighbors() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v0, v1, v2], None);
        
        let neighbors = mesh.vertex_neighbors(v0);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&v1));
        assert!(neighbors.contains(&v2));
    }

    #[test]
    fn test_vertex_faces() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        
        let f1 = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let f2 = mesh.add_face(vec![v1, v3, v2], None).unwrap();
        
        let faces = mesh.vertex_faces(v1);
        assert_eq!(faces.len(), 2);
        assert!(faces.contains(&f1));
        assert!(faces.contains(&f2));
    }

    #[test]
    fn test_is_vertex_on_boundary() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v0, v1, v2], None);
        
        // All vertices of a single triangle are on boundary
        assert!(mesh.is_vertex_on_boundary(v0));
        assert!(mesh.is_vertex_on_boundary(v1));
        assert!(mesh.is_vertex_on_boundary(v2));
    }

    #[test]
    fn test_face_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.face_normal(f).unwrap();
        
        // Normal should point in +Z direction for this triangle
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!(normal.x.abs() < 1e-10);
        assert!(normal.y.abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal(v0).unwrap();
        
        // Normal should point in +Z direction
        assert!((normal.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_face_area() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let area = mesh.face_area(f).unwrap();
        
        // Area of triangle with base=1, height=1 should be 0.5
        assert!((area - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_face_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.face_normals();
        
        assert_eq!(normals.len(), 1);
        assert!(normals.contains_key(&f));
        let normal = normals.get(&f).unwrap();
        assert!((normal.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.vertex_normals();
        
        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v0));
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));
    }

    #[test]
    fn test_from_polygons_simple() {
        let triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
    }

    #[test]
    fn test_from_polygons_vertex_merging() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0, 0.0, 0.0), // Shared vertex
            Point::new(0.0, 1.0, 0.0), // Shared vertex
            Point::new(1.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], None);
        assert_eq!(mesh.number_of_vertices(), 4); // Should merge shared vertices
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_precision() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0000001, 0.0, 0.0), // Very close to (1,0,0)
            Point::new(0.0, 1.0000001, 0.0), // Very close to (0,1,0)
            Point::new(1.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], Some(1e-6));
        assert_eq!(mesh.number_of_vertices(), 4); // Should merge vertices within precision
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_invalid_polygons() {
        let invalid_polygon = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0), // Only 2 points
        ];
        let valid_triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![invalid_polygon, valid_triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3); // Only valid triangle should be added
        assert_eq!(mesh.number_of_faces(), 1);
    }

    #[test]
    fn test_from_polygons_cube() {
        // Create a cube using 6 faces
        let faces = vec![
            // Bottom face (z=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
            ],
            // Top face (z=1)
            vec![
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
            ],
            // Front face (y=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 0.0),
            ],
            // Back face (y=1)
            vec![
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
            ],
            // Left face (x=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(0.0, 0.0, 1.0),
            ],
            // Right face (x=1)
            vec![
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 0.0),
            ],
        ];
        
        let mesh = Mesh::from_polygons(faces, None);
        assert_eq!(mesh.number_of_vertices(), 8); // A cube has 8 vertices
        assert_eq!(mesh.number_of_faces(), 6);    // A cube has 6 faces
        assert_eq!(mesh.number_of_edges(), 12);   // A cube has 12 edges
        assert_eq!(mesh.euler(), 2);             // Euler characteristic for a cube: V-E+F = 8-12+6 = 2
    }

    #[test]
    fn test_clear() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);
        
        assert!(!mesh.is_empty());
        mesh.clear();
        assert!(mesh.is_empty());
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
    }
}
