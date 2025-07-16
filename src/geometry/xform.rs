use crate::geometry::Point;
use crate::geometry::Vector;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut, Mul, MulAssign};
use crate::common::Data;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xform {
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

impl Xform {
    /// Creates a new identity `Xform` with default `Data`.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Xform;
    /// let xform = Xform::new();
    /// assert_eq!(xform.m00, 1.0);
    /// assert_eq!(xform.m11, 1.0);
    /// assert_eq!(xform.m22, 1.0);
    /// assert_eq!(xform.m33, 1.0);
    /// ```
    pub fn new() -> Self {
        Xform {
            m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
            data: Data::with_name("Xform"),
        }
    }
    
    /// Creates a translation transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `tx` - Translation along the x-axis.
    /// * `ty` - Translation along the y-axis.
    /// * `tz` - Translation along the z-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Xform;
    /// let xform = Xform::translation(2.0, 3.0, 4.0);
    /// assert_eq!(xform.m03, 2.0); // tx
    /// assert_eq!(xform.m13, 3.0); // ty
    /// assert_eq!(xform.m23, 4.0); // tz
    /// // Check that other values are correct for a translation matrix
    /// assert_eq!(xform.m00, 1.0);
    /// assert_eq!(xform.m11, 1.0);
    /// assert_eq!(xform.m22, 1.0);
    /// assert_eq!(xform.m33, 1.0)
    /// ```
    pub fn translation(tx: f64, ty: f64, tz: f64) -> Self {
        let mut result = Xform::new();
        result.m03 = tx;
        result.m13 = ty;
        result.m23 = tz;
        result
    }
    
    /// Creates a scaling transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `sx` - Scale factor along the x-axis.
    /// * `sy` - Scale factor along the y-axis.
    /// * `sz` - Scale factor along the z-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Xform;
    /// let xform = Xform::scaling(2.0, 3.0, 4.0);
    /// assert_eq!(xform.m00, 2.0); // sx
    /// assert_eq!(xform.m11, 3.0); // sy
    /// assert_eq!(xform.m22, 4.0); // sz
    /// // Check that other values are correct for a scaling matrix
    /// assert_eq!(xform.m03, 0.0);
    /// assert_eq!(xform.m13, 0.0);
    /// assert_eq!(xform.m23, 0.0);
    /// assert_eq!(xform.m33, 1.0)
    /// ```
    pub fn scaling(sx: f64, sy: f64, sz: f64) -> Self {
        Xform {
            m00: sx,  m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: sy,  m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: sz,  m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
            data: Data::with_name("Xform"),
        }
    }

    /// Creates a transformation matrix that maps one plane to another.
    ///
    /// # Arguments
    ///
    /// * `origin_0` - The origin of the first plane.
    /// * `x_axis_0` - The x-axis of the first plane.
    /// * `y_axis_0` - The y-axis of the first plane.
    /// * `origin_1` - The origin of the second plane.
    /// * `x_axis_1` - The x-axis of the second plane.
    /// * `y_axis_1` - The y-axis of the second plane.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Xform, Point, Vector};
    /// let origin_0 = Point::new(0.0, 0.0, 0.0);
    /// let x_axis_0 = Vector::x_axis();
    /// let y_axis_0 = Vector::y_axis();
    /// let origin_1 = Point::new(1.0, 1.0, 1.0);
    /// let x_axis_1 = Vector::x_axis();
    /// let y_axis_1 = Vector::y_axis();
    /// let xform = Xform::plane_to_plane(&origin_0, &x_axis_0, &y_axis_0, &origin_1, &x_axis_1, &y_axis_1);
    /// // Test that the transformation correctly maps points from one plane to the other
    /// let test_point = Point::new(1.0, 1.0, 0.0); // A point on the first plane
    /// let transformed = xform.apply(&test_point);
    /// // The point should be mapped to the second plane at (2.0, 2.0, 1.0)
    /// assert!((transformed.x - 2.0).abs() < 1e-10);
    /// assert!((transformed.y - 2.0).abs() < 1e-10);
    /// assert!((transformed.z - 1.0).abs() < 1e-10)
    /// ```
    pub fn plane_to_plane(origin_0: &Point, x_axis_0: &Vector, y_axis_0: &Vector, 
                          origin_1: &Point, x_axis_1: &Vector, y_axis_1: &Vector) -> Self {
        let z_axis_0 = x_axis_0.cross(y_axis_0);
        let z_axis_1 = x_axis_1.cross(y_axis_1);
        
        let mut _x_axis_0 = x_axis_0.clone();
        let mut _y_axis_0 = y_axis_0.clone();
        let mut _z_axis_0 = z_axis_0;
        _x_axis_0.unitize();
        _y_axis_0.unitize();
        _z_axis_0.unitize();
        
        let mut _x_axis_1 = x_axis_1.clone();
        let mut _y_axis_1 = y_axis_1.clone();
        let mut _z_axis_1 = z_axis_1;
        _x_axis_1.unitize();
        _y_axis_1.unitize();
        _z_axis_1.unitize();
        
        let t0 = Xform::translation(-origin_0.x, -origin_0.y, -origin_0.z);
        
        // Create transformation matrix for first coordinate system
        let mut f0 = Xform::new();
        f0.m00 = _x_axis_0.x; f0.m01 = _x_axis_0.y; f0.m02 = _x_axis_0.z;
        f0.m10 = _y_axis_0.x; f0.m11 = _y_axis_0.y; f0.m12 = _y_axis_0.z;
        f0.m20 = _z_axis_0.x; f0.m21 = _z_axis_0.y; f0.m22 = _z_axis_0.z;
        
        // Create transformation matrix for second coordinate system
        let mut f1 = Xform::new();
        f1.m00 = _x_axis_1.x; f1.m01 = _y_axis_1.x; f1.m02 = _z_axis_1.x;
        f1.m10 = _x_axis_1.y; f1.m11 = _y_axis_1.y; f1.m12 = _z_axis_1.y;
        f1.m20 = _x_axis_1.z; f1.m21 = _y_axis_1.z; f1.m22 = _z_axis_1.z;
        
        // Compute rotation
        let r = &f1 * &f0;
        
        // Final translation
        let t1 = Xform::translation(origin_1.x, origin_1.y, origin_1.z);
        
        // Combine transformations
        &t1 * &(&r * &t0)
    }
    
    /// Creates a transformation matrix that maps a plane to the XY plane.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the plane.
    /// * `x_axis` - The x-axis of the plane.
    /// * `y_axis` - The y-axis of the plane.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Xform, Point, Vector};
    /// let origin = Point::new(1.0, 1.0, 1.0);
    /// let x_axis = Vector::x_axis();
    /// let y_axis = Vector::y_axis();
    /// let xform = Xform::plane_to_xy(&origin, &x_axis, &y_axis);
    /// // Test that the transformation correctly maps a point on the plane to the XY plane
    /// let test_point = Point::new(2.0, 1.0, 1.0); // A point on the plane
    /// let transformed = xform.apply(&test_point);
    /// // The point should be mapped to the XY plane at (1.0, 0.0, 0.0)
    /// assert!((transformed.x - 1.0).abs() < 1e-10);
    /// assert!((transformed.y - 0.0).abs() < 1e-10);
    /// assert!((transformed.z - 0.0).abs() < 1e-10)
    /// ```
    pub fn plane_to_xy(origin: &Point, x_axis: &Vector, y_axis: &Vector) -> Self {
        let z_axis = x_axis.cross(y_axis);
        
        let mut _x_axis = x_axis.clone();
        let mut _y_axis = y_axis.clone();
        let mut _z_axis = z_axis;
        _x_axis.unitize();
        _y_axis.unitize();
        _z_axis.unitize();
        
        // Translation to origin
        let t = Xform::translation(-origin.x, -origin.y, -origin.z);
        
        // Create transformation matrix
        let mut f = Xform::new();
        f.m00 = _x_axis.x; f.m01 = _x_axis.y; f.m02 = _x_axis.z;
        f.m10 = _y_axis.x; f.m11 = _y_axis.y; f.m12 = _y_axis.z;
        f.m20 = _z_axis.x; f.m21 = _z_axis.y; f.m22 = _z_axis.z;
        
        // Combine transformations
        &f * &t
    }
    
    /// Creates a transformation matrix that maps the XY plane to another plane.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the target plane.
    /// * `x_axis` - The x-axis of the target plane.
    /// * `y_axis` - The y-axis of the target plane.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Xform, Point, Vector};
    /// let origin = Point::new(1.0, 1.0, 1.0);
    /// let x_axis = Vector::x_axis();
    /// let y_axis = Vector::y_axis();
    /// let xform = Xform::xy_to_plane(&origin, &x_axis, &y_axis);
    /// // Test that the transformation correctly maps a point on the XY plane to the target plane
    /// let test_point = Point::new(1.0, 0.0, 0.0); // A point on the XY plane
    /// let transformed = xform.apply(&test_point);
    /// // The point should be mapped to the target plane at (2.0, 1.0, 1.0)
    /// assert!((transformed.x - 2.0).abs() < 1e-10);
    /// assert!((transformed.y - 1.0).abs() < 1e-10);
    /// assert!((transformed.z - 1.0).abs() < 1e-10)
    /// ```
    pub fn xy_to_plane(origin: &Point, x_axis: &Vector, y_axis: &Vector) -> Self {
        let z_axis = x_axis.cross(y_axis);
        
        let mut _x_axis = x_axis.clone();
        let mut _y_axis = y_axis.clone();
        let mut _z_axis = z_axis;
        _x_axis.unitize();
        _y_axis.unitize();
        _z_axis.unitize();
        
        // Create transformation matrix
        let mut f = Xform::new();
        f.m00 = _x_axis.x; f.m01 = _y_axis.x; f.m02 = _z_axis.x;
        f.m10 = _x_axis.y; f.m11 = _y_axis.y; f.m12 = _z_axis.y;
        f.m20 = _x_axis.z; f.m21 = _y_axis.z; f.m22 = _z_axis.z;
        
        // Final translation
        let t = Xform::translation(origin.x, origin.y, origin.z);
        
        // Combine transformations
        &t * &f
    }
    
    // Multiply has been replaced with the * operator. Use &a * &b instead of Xform::multiply(&a, &b)
    
    /// Creates a transformation matrix for change of basis between two coordinate systems.
    ///
    /// # Arguments
    ///
    /// * `origin_1` - Origin of the first coordinate system.
    /// * `x_axis_1` - X-axis of the first coordinate system.
    /// * `y_axis_1` - Y-axis of the first coordinate system.
    /// * `z_axis_1` - Z-axis of the first coordinate system.
    /// * `origin_0` - Origin of the second coordinate system.
    /// * `x_axis_0` - X-axis of the second coordinate system.
    /// * `y_axis_0` - Y-axis of the second coordinate system.
    /// * `z_axis_0` - Z-axis of the second coordinate system.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Xform, Point, Vector};
    /// let origin_0 = Point::new(0.0, 0.0, 0.0);
    /// let x_axis_0 = Vector::x_axis();
    /// let y_axis_0 = Vector::y_axis();
    /// let z_axis_0 = Vector::z_axis();
    /// let origin_1 = Point::new(1.0, 1.0, 1.0);
    /// let x_axis_1 = Vector::x_axis();
    /// let y_axis_1 = Vector::y_axis();
    /// let z_axis_1 = Vector::z_axis();
    /// let xform = Xform::change_basis(
    ///     &origin_1, &x_axis_1, &y_axis_1, &z_axis_1,
    ///     &origin_0, &x_axis_0, &y_axis_0, &z_axis_0
    /// );
    /// // Test that the transformation correctly changes basis
    /// let test_point = Point::new(1.0, 1.0, 1.0); // A point in the first coordinate system
    /// let transformed = xform.apply(&test_point);
    /// // For this specific transformation (with identical axes but translated origin),
    /// // we expect the point to be mapped to (0.0, 0.0, 0.0) in the second coordinate system
    /// assert!((transformed.x - 0.0).abs() < 1e-10);
    /// assert!((transformed.y - 0.0).abs() < 1e-10);
    /// assert!((transformed.z - 0.0).abs() < 1e-10)
    /// ```
    pub fn change_basis(
        origin_1: &Point, x_axis_1: &Vector, y_axis_1: &Vector, z_axis_1: &Vector,
        origin_0: &Point, x_axis_0: &Vector, y_axis_0: &Vector, z_axis_0: &Vector
    ) -> Self {
        // Compute dot products
        let a = x_axis_1.dot(y_axis_1);
        let b = x_axis_1.dot(z_axis_1);
        let c = y_axis_1.dot(z_axis_1);
        
        // Set up the linear system
        let mut r = [
            [x_axis_1.dot(x_axis_1), a, b, x_axis_1.dot(x_axis_0), x_axis_1.dot(y_axis_0), x_axis_1.dot(z_axis_0)],
            [a, y_axis_1.dot(y_axis_1), c, y_axis_1.dot(x_axis_0), y_axis_1.dot(y_axis_0), y_axis_1.dot(z_axis_0)],
            [b, c, z_axis_1.dot(z_axis_1), z_axis_1.dot(x_axis_0), z_axis_1.dot(y_axis_0), z_axis_1.dot(z_axis_0)]
        ];
        
        // Row reduction (Gaussian elimination)
        // Find the pivot row
        let mut i0 = if r[0][0] >= r[1][1] { 0 } else { 1 };
        if r[2][2] > r[i0][i0] { i0 = 2; }
        let mut i1 = (i0 + 1) % 3;
        let mut i2 = (i1 + 1) % 3;
        
        // Check if the matrix is invertible
        if r[i0][i0] == 0.0 {
            return Xform::new(); // Return identity if not invertible
        }
        
        // First row reduction
        let d = 1.0 / r[i0][i0];
        for j in 0..6 {
            r[i0][j] *= d;
        }
        r[i0][i0] = 1.0;
        
        if r[i1][i0] != 0.0 {
            let d = -r[i1][i0];
            for j in 0..6 {
                r[i1][j] += d * r[i0][j];
            }
            r[i1][i0] = 0.0;
        }
        
        if r[i2][i0] != 0.0 {
            let d = -r[i2][i0];
            for j in 0..6 {
                r[i2][j] += d * r[i0][j];
            }
            r[i2][i0] = 0.0;
        }
        
        // Second row reduction
        if r[i1][i1].abs() < r[i2][i2].abs() {
            let temp = i1;
            i1 = i2;
            i2 = temp;
        }
        
        if r[i1][i1] == 0.0 {
            return Xform::new(); // Return identity if not invertible
        }
        
        let d = 1.0 / r[i1][i1];
        for j in 0..6 {
            r[i1][j] *= d;
        }
        r[i1][i1] = 1.0;
        
        if r[i0][i1] != 0.0 {
            let d = -r[i0][i1];
            for j in 0..6 {
                r[i0][j] += d * r[i1][j];
            }
            r[i0][i1] = 0.0;
        }
        
        if r[i2][i1] != 0.0 {
            let d = -r[i2][i1];
            for j in 0..6 {
                r[i2][j] += d * r[i1][j];
            }
            r[i2][i1] = 0.0;
        }
        
        // Third row reduction
        if r[i2][i2] == 0.0 {
            return Xform::new(); // Return identity if not invertible
        }
        
        let d = 1.0 / r[i2][i2];
        for j in 0..6 {
            r[i2][j] *= d;
        }
        r[i2][i2] = 1.0;
        
        if r[i0][i2] != 0.0 {
            let d = -r[i0][i2];
            for j in 0..6 {
                r[i0][j] += d * r[i2][j];
            }
            r[i0][i2] = 0.0;
        }
        
        if r[i1][i2] != 0.0 {
            let d = -r[i1][i2];
            for j in 0..6 {
                r[i1][j] += d * r[i2][j];
            }
            r[i1][i2] = 0.0;
        }
        
        // Create transformation matrix
        let mut m_xform = Xform::new();
        m_xform.m00 = r[0][3];
        m_xform.m01 = r[0][4];
        m_xform.m02 = r[0][5];
        m_xform.m10 = r[1][3];
        m_xform.m11 = r[1][4];
        m_xform.m12 = r[1][5];
        m_xform.m20 = r[2][3];
        m_xform.m21 = r[2][4];
        m_xform.m22 = r[2][5];
        
        // Apply translations
        let t0 = Xform::translation(-origin_1.x, -origin_1.y, -origin_1.z);
        let t2 = Xform::translation(origin_0.x, origin_0.y, origin_0.z);
        
        // Combine transformations
        &t2 * &(&m_xform * &t0)
    }
    
    /// Applies a transformation to a point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to transform.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Xform, Point};
    /// let xform = Xform::translation(1.0, 2.0, 3.0);
    /// let point = Point::new(0.0, 0.0, 0.0);
    /// let transformed = xform.apply(&point);
    /// assert_eq!(transformed.x, 1.0);
    /// assert_eq!(transformed.y, 2.0);
    /// assert_eq!(transformed.z, 3.0);
    /// 
    /// // Test with non-zero point
    /// let point2 = Point::new(2.0, 3.0, 4.0);
    /// let transformed2 = xform.apply(&point2);
    /// assert_eq!(transformed2.x, 3.0);
    /// assert_eq!(transformed2.y, 5.0);
    /// assert_eq!(transformed2.z, 7.0);
    /// 
    /// // Test with a scaling transformation
    /// let scale_xform = Xform::scaling(2.0, 3.0, 4.0);
    /// let scaled = scale_xform.apply(&point2);
    /// assert_eq!(scaled.x, 4.0);
    /// assert_eq!(scaled.y, 9.0);
    /// assert_eq!(scaled.z, 16.0);
    /// ```
    pub fn apply(&self, point: &Point) -> Point {
        let x = point.x;
        let y = point.y;
        let z = point.z;
        
        Point {
            x: self.m00 * x + self.m01 * y + self.m02 * z + self.m03,
            y: self.m10 * x + self.m11 * y + self.m12 * z + self.m13,
            z: self.m20 * x + self.m21 * y + self.m22 * z + self.m23,
            data: Data::with_name("Point")
        }
    }
}

/// Implementation of the Index trait to allow accessing matrix elements with [row, col] syntax
impl Index<(usize, usize)> for Xform {
    type Output = f64;
    
    /// Returns a reference to the matrix element at the specified position.
    /// 
    /// # Arguments
    /// 
    /// * `index` - A tuple (row, column) representing the position in the matrix.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Xform;
    /// let xform = Xform::new();
    /// assert_eq!(xform[(0, 0)], 1.0); // m00
    /// assert_eq!(xform[(1, 1)], 1.0); // m11
    /// ```
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        
        match (row, col) {
            (0, 0) => &self.m00,
            (0, 1) => &self.m01,
            (0, 2) => &self.m02,
            (0, 3) => &self.m03,
            (1, 0) => &self.m10,
            (1, 1) => &self.m11,
            (1, 2) => &self.m12,
            (1, 3) => &self.m13,
            (2, 0) => &self.m20,
            (2, 1) => &self.m21,
            (2, 2) => &self.m22,
            (2, 3) => &self.m23,
            (3, 0) => &self.m30,
            (3, 1) => &self.m31,
            (3, 2) => &self.m32,
            (3, 3) => &self.m33,
            _ => panic!("Matrix index out of bounds: ({}, {})", row, col),
        }
    }
}

/// Implementation of the IndexMut trait to allow modifying matrix elements with [row, col] syntax
impl IndexMut<(usize, usize)> for Xform {
    /// Returns a mutable reference to the matrix element at the specified position.
    /// 
    /// # Arguments
    /// 
    /// * `index` - A tuple (row, column) representing the position in the matrix.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Xform;
    /// let mut xform = Xform::new();
    /// xform[(0, 3)] = 5.0; // Set m03 to 5.0
    /// assert_eq!(xform.m03, 5.0);
    /// ```
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        
        match (row, col) {
            (0, 0) => &mut self.m00,
            (0, 1) => &mut self.m01,
            (0, 2) => &mut self.m02,
            (0, 3) => &mut self.m03,
            (1, 0) => &mut self.m10,
            (1, 1) => &mut self.m11,
            (1, 2) => &mut self.m12,
            (1, 3) => &mut self.m13,
            (2, 0) => &mut self.m20,
            (2, 1) => &mut self.m21,
            (2, 2) => &mut self.m22,
            (2, 3) => &mut self.m23,
            (3, 0) => &mut self.m30,
            (3, 1) => &mut self.m31,
            (3, 2) => &mut self.m32,
            (3, 3) => &mut self.m33,
            _ => panic!("Matrix index out of bounds: ({}, {})", row, col),
        }
    }
}

/// Implementation of the Mul trait to allow matrix multiplication with the * operator
impl Mul for &Xform {
    type Output = Xform;
    
    /// Multiplies two transformation matrices using the * operator.
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The right-hand side transformation matrix.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Xform;
    /// let a = Xform::translation(1.0, 2.0, 3.0);
    /// let b = Xform::scaling(2.0, 2.0, 2.0);
    /// let c = &a * &b;
    /// assert_eq!(c.m00, a.m00 * b.m00 + a.m01 * b.m10 + a.m02 * b.m20 + a.m03 * b.m30);
    /// assert_eq!(c.m11, a.m10 * b.m01 + a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Xform::new();
        
        // Row 0
        result.m00 = self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20 + self.m03 * rhs.m30;
        result.m01 = self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21 + self.m03 * rhs.m31;
        result.m02 = self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22 + self.m03 * rhs.m32;
        result.m03 = self.m00 * rhs.m03 + self.m01 * rhs.m13 + self.m02 * rhs.m23 + self.m03 * rhs.m33;
        
        // Row 1
        result.m10 = self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20 + self.m13 * rhs.m30;
        result.m11 = self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31;
        result.m12 = self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32;
        result.m13 = self.m10 * rhs.m03 + self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33;
        
        // Row 2
        result.m20 = self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20 + self.m23 * rhs.m30;
        result.m21 = self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31;
        result.m22 = self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32;
        result.m23 = self.m20 * rhs.m03 + self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33;
        
        // Row 3
        result.m30 = self.m30 * rhs.m00 + self.m31 * rhs.m10 + self.m32 * rhs.m20 + self.m33 * rhs.m30;
        result.m31 = self.m30 * rhs.m01 + self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31;
        result.m32 = self.m30 * rhs.m02 + self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32;
        result.m33 = self.m30 * rhs.m03 + self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33;
        
        result
    }
}

/// Implementation of the MulAssign trait to allow in-place matrix multiplication with the *= operator
impl MulAssign<&Xform> for Xform {
    /// Performs in-place multiplication of this transformation matrix with another.
    /// 
    /// # Arguments
    /// 
    /// * `rhs` - The right-hand side transformation matrix.
    /// 
    /// # Example
    /// 
    /// ```
    /// use openmodel::geometry::Xform;
    /// let mut a = Xform::translation(1.0, 2.0, 3.0);
    /// let b = Xform::scaling(2.0, 2.0, 2.0);
    /// a *= &b; // a becomes a * b
    /// // Check that multiplication worked correctly
    /// let expected = &Xform::translation(1.0, 2.0, 3.0) * &b;
    /// assert_eq!(a.m00, expected.m00);
    /// assert_eq!(a.m11, expected.m11);
    /// assert_eq!(a.m22, expected.m22);
    /// ```
    fn mul_assign(&mut self, rhs: &Xform) {
        // Store original values before they're overwritten
        let orig = self.clone();
        
        // Row 0
        self.m00 = orig.m00 * rhs.m00 + orig.m01 * rhs.m10 + orig.m02 * rhs.m20 + orig.m03 * rhs.m30;
        self.m01 = orig.m00 * rhs.m01 + orig.m01 * rhs.m11 + orig.m02 * rhs.m21 + orig.m03 * rhs.m31;
        self.m02 = orig.m00 * rhs.m02 + orig.m01 * rhs.m12 + orig.m02 * rhs.m22 + orig.m03 * rhs.m32;
        self.m03 = orig.m00 * rhs.m03 + orig.m01 * rhs.m13 + orig.m02 * rhs.m23 + orig.m03 * rhs.m33;
        
        // Row 1
        self.m10 = orig.m10 * rhs.m00 + orig.m11 * rhs.m10 + orig.m12 * rhs.m20 + orig.m13 * rhs.m30;
        self.m11 = orig.m10 * rhs.m01 + orig.m11 * rhs.m11 + orig.m12 * rhs.m21 + orig.m13 * rhs.m31;
        self.m12 = orig.m10 * rhs.m02 + orig.m11 * rhs.m12 + orig.m12 * rhs.m22 + orig.m13 * rhs.m32;
        self.m13 = orig.m10 * rhs.m03 + orig.m11 * rhs.m13 + orig.m12 * rhs.m23 + orig.m13 * rhs.m33;
        
        // Row 2
        self.m20 = orig.m20 * rhs.m00 + orig.m21 * rhs.m10 + orig.m22 * rhs.m20 + orig.m23 * rhs.m30;
        self.m21 = orig.m20 * rhs.m01 + orig.m21 * rhs.m11 + orig.m22 * rhs.m21 + orig.m23 * rhs.m31;
        self.m22 = orig.m20 * rhs.m02 + orig.m21 * rhs.m12 + orig.m22 * rhs.m22 + orig.m23 * rhs.m32;
        self.m23 = orig.m20 * rhs.m03 + orig.m21 * rhs.m13 + orig.m22 * rhs.m23 + orig.m23 * rhs.m33;
        
        // Row 3
        self.m30 = orig.m30 * rhs.m00 + orig.m31 * rhs.m10 + orig.m32 * rhs.m20 + orig.m33 * rhs.m30;
        self.m31 = orig.m30 * rhs.m01 + orig.m31 * rhs.m11 + orig.m32 * rhs.m21 + orig.m33 * rhs.m31;
        self.m32 = orig.m30 * rhs.m02 + orig.m31 * rhs.m12 + orig.m32 * rhs.m22 + orig.m33 * rhs.m32;
        self.m33 = orig.m30 * rhs.m03 + orig.m31 * rhs.m13 + orig.m32 * rhs.m23 + orig.m33 * rhs.m33;
        
        // Data is kept unchanged
    }
}

impl fmt::Display for Xform{
    /// Log Xform.
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Xform;
    /// use openmodel::common::Data;
    /// let Xform = Xform {
    ///    m00 : 1.0,
    ///    m01 : 0.0,
    ///    m02 : 0.0,
    ///    m03 : 0.0,
    ///    m10 : 0.0,
    ///    m11 : 1.0,
    ///    m12 : 0.0,
    ///    m13 : 0.0,
    ///    m20 : 0.0,
    ///    m21 : 0.0,
    ///    m22 : 1.0,
    ///    m23 : 0.0,
    ///    m30 : 0.0,
    ///    m31 : 0.0,
    ///    m32 : 0.0,
    ///    m33 : 1.0,
    ///    data: Data::with_name("Xform")
    /// };
    /// println!("{}", Xform);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Xform 4x4 {{ Row A: {}, {}, {}, {}, Row B: {}, {}, {}, {}, Row C:  {}, {}, {}, {}, Row D: {}, {}, {}, {}, Data: {} }}", self.m00, self.m01, self.m02, self.m03, self.m10, self.m11, self.m12, self.m13, self.m20, self.m21, self.m22, self.m23, self.m30, self.m31, self.m32, self.m33, self.data)
    }
}