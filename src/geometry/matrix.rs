use crate::geometry::Point;
use crate::geometry::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix {
    // First column, first row
    pub m00 : f64,
    // First column, second row
    pub m01 : f64,
    // First column, third row
    pub m02 : f64,
    // First column, fourth row
    pub m03 : f64,
    // Second column, first row
    pub m10 : f64,
    // Second column, second row
    pub m11 : f64,
    // Second column, third row
    pub m12 : f64,
    // Second column, fourth row
    pub m13 : f64,
    // Third column, first row
    pub m20 : f64,
    // Third column, second row
    pub m21 : f64,
    // Third column, third row
    pub m22 : f64,
    // Third column, fourth row
    pub m23 : f64,
    // Fourth column, first row
    pub m30 : f64,
    // Fourth column, second row
    pub m31 : f64,
    // Fourth column, third row
    pub m32 : f64,
    // Fourth column, fourth row
    pub m33 : f64,
    /// Associated data - guid and name.
    pub data: Data,
}

impl fmt::Display for Matrix{
    /// Log matrix.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Matrix;
    /// let matrix = Matrix::default();
    /// println!("{}", matrix);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Matrix 4x4 {{ Row A: {}, {}, {}, {}, Row B: {}, {}, {}, {}, Row C:  {}, {}, {}, {}, Row D: {}, {}, {}, {}, Data: {} }}", m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33, self.data)
    }
}