use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::Frame;
use crate::geometry::Matrix;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// The collection of points.
    pub points : Point,
    /// The collection of vectors.
    pub data: Data,
}

impl fmt::Display for Mesh{
    /// Log matrix.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Mesh;
    /// let mesh = Mesh::default();
    /// println!("{}", mesh);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Mesh {{ vertices: {}, faces: {}, edges: {}, Data: {} }}", 0, 0, 0, self.data)
    }
}