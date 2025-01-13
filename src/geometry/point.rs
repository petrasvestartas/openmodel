use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Index, IndexMut}; // For operator overloading
use crate::geometry::Vector; // For conversion to Point

#[derive(Debug, Clone)]
pub struct Point {
    /// The x coordinate of the point.
    pub x: f64,
    /// The y coordinate of the point.
    pub y: f64,
    /// The z coordinate of the point.
    pub z: f64,
}

#[allow(dead_code)]
impl Point {
    /// Creates a new `Vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate.
    /// * `y` - The y coordinate.
    /// * `z` - The z coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let v = Point::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// Computes the distance between two points.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p1 = Point::new(1.0, 2.0, 2.0);
    /// let p2 = Point::new(4.0, 6.0, 6.0);
    /// ```
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    /// Translates the point by the given amounts.
    ///
    /// # Arguments
    ///
    /// * `dx` - The offset in the x direction.
    /// * `dy` - The offset in the y direction (default is 0.0).
    /// * `dz` - The offset in the z direction (default is 0.0).
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p = Point::new(1.0, 2.0, 2.0);
    /// p.translate(1.0, 2.0, 3.0);
    /// assert_eq!(p.x, 2.0);
    /// assert_eq!(p.y, 4.0);
    /// assert_eq!(p.z, 5.0);
    /// ```
    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }


    /// Returns a new point translated by the given amounts.
    ///
    /// # Arguments
    ///
    /// * `dx` - The offset in the x direction.
    /// * `dy` - The offset in the y direction (default is 0.0).
    /// * `dz` - The offset in the z direction (default is 0.0).
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::new(1.0, 2.0, 2.0);
    /// let p2 = p.translated(1.0, 2.0, 3.0);
    /// assert_eq!(p2.x, 2.0);
    /// assert_eq!(p2.y, 4.0);
    /// assert_eq!(p2.z, 5.0);
    /// ```
    pub fn translated(&self, dx: f64, dy: f64, dz: f64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}


impl Default for Point {
    /// Creates a default `Point` with all coordinates set to 0.0.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::default();
    /// assert_eq!(p.x, 0.0);
    /// assert_eq!(p.y, 0.0);
    /// assert_eq!(p.z, 0.0);
    /// ```
    fn default() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Index<usize> for Point {
    type Output = f64;

    /// Provides read-only access to the coordinates of the point using the `[]` operator.
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
    /// use openmodel::geometry::Point;
    /// let p = Point::new(1.0, 2.0, 3.0);
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

impl IndexMut<usize> for Point {
    /// Provides mutable access to the coordinates of the point using the `[]` operator.
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
    /// use openmodel::geometry::Point;
    /// let mut p = Point::new(1.0, 2.0, 3.0);
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


/// Converts a `Vector` into a `Point`.
///
/// This implementation allows a `Vector` to be converted into a `Point`
/// by taking the `x`, `y`, and `z` components of the `Vector` and using
/// them to create a new `Point`.
///
/// # Arguments
///
/// * `vector` - The `Vector` to be converted.
///
/// # Example
///
/// ```
/// use openmodel::geometry::{Point, Vector};
/// let v = Vector::new(1.0, 2.0, 3.0);
/// let p: Point = v.into();
/// assert_eq!(p.x, 1.0);
/// assert_eq!(p.y, 2.0);
/// assert_eq!(p.z, 3.0);
/// ```
impl From<Vector> for Point {
    fn from(vector: Vector) -> Self {
        Point {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }
}


impl MulAssign<f64> for Point {
    /// Multiplies the coordinates of the point by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p = Point::new(1.0, 2.0, 3.0);
    /// p *= 2.0;
    /// assert_eq!(p.x, 2.0);
    /// assert_eq!(p.y, 4.0);
    /// assert_eq!(p.z, 6.0);
    /// ```
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl DivAssign<f64> for Point {
    /// Divides the coordinates of the point by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p = Point::new(1.0, 2.0, 3.0);
    /// p /= 2.0;
    /// assert_eq!(p.x, 0.5);
    /// assert_eq!(p.y, 1.0);
    /// assert_eq!(p.z, 1.5);
    /// ```
    fn div_assign(&mut self, factor: f64) {
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }
}

impl AddAssign<&Point> for Point {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p1 = Point::new(1.0, 2.0, 3.0);
    /// let p2 = Point::new(4.0, 5.0, 6.0);
    /// p1 += &p2;
    /// assert_eq!(p1.x, 5.0);
    /// assert_eq!(p1.y, 7.0);
    /// assert_eq!(p1.z, 9.0);
    /// ```
    fn add_assign(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<&Point> for Point {
    /// Subtracts the coordinates of another point from this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let mut p1 = Point::new(4.0, 5.0, 6.0);
    /// let p2 = Point::new(1.0, 2.0, 3.0);
    /// p1 -= &p2;
    /// assert_eq!(p1.x, 3.0);
    /// assert_eq!(p1.y, 3.0);
    /// assert_eq!(p1.z, 3.0);
    /// ```
    fn sub_assign(&mut self, other: &Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    /// Multiplies the coordinates of the point by a scalar and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// let p2 = p * 2.0;
    /// assert_eq!(p2.x, 2.0);
    /// assert_eq!(p2.y, 4.0);
    /// assert_eq!(p2.z, 6.0);
    /// ```
    fn mul(self, factor: f64) -> Point {
        let mut result = self;
        result *= factor;
        result
    }
}

impl Div<f64> for Point {
    type Output = Point;

    /// Divides the coordinates of the point by a scalar and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// let p2 = p / 2.0;
    /// assert_eq!(p2.x, 0.5);
    /// assert_eq!(p2.y, 1.0);
    /// assert_eq!(p2.z, 1.5);
    /// ```
    fn div(self, factor: f64) -> Point {
        let mut result = self;
        result /= factor;
        result
    }
}

impl Add<&Vector> for Point {
    type Output = Point;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Point, Vector};
    /// let p = Point::new(1.0, 2.0, 3.0);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let p2 = p + &v;
    /// assert_eq!(p2.x, 5.0);
    /// assert_eq!(p2.y, 7.0);
    /// assert_eq!(p2.z, 9.0);
    /// ```
    fn add(self, other: &Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<&Vector> for Point {
    type Output = Vector;

    /// Subtracts the coordinates of a vector from this point and returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::{Point, Vector};
    /// let p = Point::new(4.0, 5.0, 6.0);
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = p - &v;
    /// assert_eq!(v2.x, 3.0);
    /// assert_eq!(v2.y, 3.0);
    /// assert_eq!(v2.z, 3.0);
    /// ```
    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialEq for Point {
    /// Checks if two points are equal.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p1 = Point::new(1.0, 2.0, 3.0);
    /// let p2 = Point::new(1.0, 2.0, 3.0);
    /// assert_eq!(p1, p2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialOrd for Point {
    /// Compares the distances of two points from the origin.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Point;
    /// let p1 = Point::new(1.0, 2.0, 3.0);
    /// let p2 = Point::new(4.0, 5.0, 6.0);
    /// assert!(p1 < p2);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance(&Point::default()).partial_cmp(&other.distance(&Point::default()))?)
    }
}