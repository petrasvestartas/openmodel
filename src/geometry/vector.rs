use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Index, IndexMut}; // For operator overloading
use crate::geometry::Point; // For conversion to Vector

#[derive(Debug, Clone)]
pub struct Vector {
    /// The x component of the vector.
    pub x: f64,
    /// The y component of the vector.
    pub y: f64,
    /// The z component of the vector.
    pub z: f64,
}

#[allow(dead_code)]
impl Vector {
    /// Creates a new `Vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    /// assert_eq!(v.length(), 3.0);
    /// ```
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Default for Vector {
    /// Creates a default `Vector` with all components set to 0.0.
    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}


impl Index<usize> for Vector {
    type Output = f64;

    /// Provides read-only access to the coordinates of the Vector using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x, 1 for y, 2 for z).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let p = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(p[0], 1.0);
    /// assert_eq!(p[1], 2.0);
    /// assert_eq!(p[2], 3.0);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector {
    /// Provides mutable access to the coordinates of the Vector using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x, 1 for y, 2 for z).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut p = Vector::new(1.0, 2.0, 3.0);
    /// p[0] = 4.0;
    /// p[1] = 5.0;
    /// p[2] = 6.0;
    /// assert_eq!(p[0], 4.0);
    /// assert_eq!(p[1], 5.0);
    /// assert_eq!(p[2], 6.0);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Converts a `Point` into a `Vector`.
///
/// This implementation allows a `Point` to be converted into a `Vector`
/// by taking the `x`, `y`, and `z` components of the `Point` and using
/// them to create a new `Vector`.
///
/// # Arguments
///
/// * `point` - The `Point` to be converted.
///
/// # Example
///
/// ```
/// use openmodel::geometry::{Point, Vector};
/// let p = Point::new(1.0, 2.0, 3.0);
/// let v: Vector = p.into();
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// assert_eq!(v.z, 3.0);
/// ```
impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Vector {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl MulAssign<f64> for Vector {
    /// Multiplies the components of the vector by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut v = Vector::new(1.0, 2.0, 3.0);
    /// v *= 2.0;
    /// assert_eq!(v.x, 2.0);
    /// assert_eq!(v.y, 4.0);
    /// assert_eq!(v.z, 6.0);
    /// ```
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl DivAssign<f64> for Vector {
    /// Divides the components of the vector by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut v = Vector::new(1.0, 2.0, 3.0);
    /// v /= 2.0;
    /// assert_eq!(v.x, 0.5);
    /// assert_eq!(v.y, 1.0);
    /// assert_eq!(v.z, 1.5);
    /// ```
    fn div_assign(&mut self, factor: f64) {
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }
}

impl AddAssign<&Vector> for Vector {
    /// Adds the components of another vector to this vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(4.0, 5.0, 6.0);
    /// v1 += &v2;
    /// assert_eq!(v1.x, 5.0);
    /// assert_eq!(v1.y, 7.0);
    /// assert_eq!(v1.z, 9.0);
    /// ```
    fn add_assign(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Vector> for Vector {
    /// Subtracts the components of another vector from this vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut v1 = Vector::new(4.0, 5.0, 6.0);
    /// let v2 = Vector::new(1.0, 2.0, 3.0);
    /// v1 -= &v2;
    /// assert_eq!(v1.x, 3.0);
    /// assert_eq!(v1.y, 3.0);
    /// assert_eq!(v1.z, 3.0);
    /// ```
    fn sub_assign(&mut self, other: &Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    /// Multiplies the components of the vector by a scalar and returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = v * 2.0;
    /// assert_eq!(v2.x, 2.0);
    /// assert_eq!(v2.y, 4.0);
    /// assert_eq!(v2.z, 6.0);
    /// ```
    fn mul(self, factor: f64) -> Vector {
        let mut result = self;
        result *= factor;
        result
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    /// Divides the components of the vector by a scalar and returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = v / 2.0;
    /// assert_eq!(v2.x, 0.5);
    /// assert_eq!(v2.y, 1.0);
    /// assert_eq!(v2.z, 1.5);
    /// ```
    fn div(self, factor: f64) -> Vector {
        let mut result = self;
        result /= factor;
        result
    }
}

impl Add<&Point> for Vector {
    type Output = Point;

    /// Adds the components of a point to this vector and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Point, Vector};
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// let p = Point::new(4.0, 5.0, 6.0);
    /// let p2 = v + &p;
    /// assert_eq!(p2.x, 5.0);
    /// assert_eq!(p2.y, 7.0);
    /// assert_eq!(p2.z, 9.0);
    /// ```
    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<&Point> for Vector {
    type Output = Vector;

    /// Subtracts the components of a point from this vector and returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Point, Vector};
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// let v2 = v - &p;
    /// assert_eq!(v2.x, 3.0);
    /// assert_eq!(v2.y, 3.0);
    /// assert_eq!(v2.z, 3.0);
    /// ```
    fn sub(self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialEq for Vector {
    /// Checks if two vectors are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(v1, v2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Vector {
    /// Compares the lengths of two vectors.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(4.0, 5.0, 6.0);
    /// assert!(v1 < v2);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.length().partial_cmp(&other.length())?)
    }
}