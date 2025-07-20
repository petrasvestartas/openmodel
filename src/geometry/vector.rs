use crate::geometry::Point;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use crate::common::{Data, HasJsonData, FromJsonData};
use std::fmt;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    /// The x component of the vector.
    pub x: f64,
    /// The y component of the vector.
    pub y: f64,
    /// The z component of the vector.
    pub z: f64,
    /// Associated data - guid and name.
    pub data: Data,
}

impl Vector {
    /// Creates a new `Vector` with default `Data`.
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
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            x,
            y,
            z,
            data: Data::with_name("Vector"),
        }
    }

    /// Creates a new `Vector` with a specified name for `Data`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the `Data`.
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::with_name("MyVector".to_string(), 1.0, 2.0, 3.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// ```
    pub fn with_name(name: String, x: f64, y: f64, z: f64) -> Self {
        Vector {
            x,
            y,
            z,
            data: Data::with_name(&name),
        }
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

    /// Computes the cross product of two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.cross(&v2);
    /// assert_eq!(v3.x, -3.0);
    /// assert_eq!(v3.y, 6.0);
    /// assert_eq!(v3.z, -3.0);
    /// ```
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            data: Data::with_name("Vector"),
        }
    }

    /// Computes the dot product of two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(4.0, 5.0, 6.0);
    /// let v3 = v1.dot(&v2);
    /// assert_eq!(v3, 32.0);
    /// ```
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Unitizes the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let mut v = Vector::new(1.0, 2.0, 3.0);
    /// v.unitize();
    /// assert_eq!(v.x, 0.2672612419124244);
    /// assert_eq!(v.y, 0.5345224838248488);
    /// assert_eq!(v.z, 0.8017837257372732);
    /// ```
    pub fn unitize(&mut self) -> bool {
        let mut rc = false;
        let d = self.length();
        if d > 0.0 {
            self.x /= d;
            self.y /= d;
            self.z /= d;
            rc = true;
        }
        return rc;
    }
    
    /// Returns a unit vector along the positive X axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::x_axis();
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 0.0);
    /// ```
    pub fn x_axis() -> Self {
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            data: Data::with_name("Vector"),
        }
    }
    
    /// Returns a unit vector along the positive Y axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::y_axis();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 1.0);
    /// assert_eq!(v.z, 0.0);
    /// ```
    pub fn y_axis() -> Self {
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            data: Data::with_name("Vector"),
        }
    }
    
    /// Returns a unit vector along the positive Z axis.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::z_axis();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 1.0);
    /// ```
    pub fn z_axis() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            data: Data::with_name("Vector"),
        }
    }
    

}

impl Default for Vector {
    /// Creates a zero length `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::default();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 0.0);
    /// ```
    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            data: Data::with_name("Vector"),
        }
    }
}

impl From<Point> for Vector {
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
    fn from(point: Point) -> Self {
        Vector {
            x: point.x,
            y: point.y,
            z: point.z,
            data: Data::with_name("Vector"),
        }
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
            data: Data::with_name("Point"),
        }
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
            data: Data::with_name("Vector"),
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

impl fmt::Display for Vector {
    /// Log vector.
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// println!("{}", v);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {}, z: {}, Data: {} }}", self.x, self.y, self.z, self.data)
    }
}

/// Implementation of DataObject trait for Vector to support COMPAS-style JSON serialization
impl Vector {
    /// Create a structured JSON representation similar to COMPAS
    ///
    /// # Arguments
    ///
    /// * `minimal` - If true, only includes dtype and data fields
    ///
    /// # Returns
    ///
    /// A JSON value with the vector's data in COMPAS format
    ///
    /// # Example
    ///
    /// ```
    /// use openmodel::geometry::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// let json = vector.to_json_data(false);
    /// ```
    pub fn to_json_data(&self, minimal: bool) -> serde_json::Value {
        let geometric_data = serde_json::json!({
            "x": self.x,
            "y": self.y,
            "z": self.z
        });
        
        self.data.to_json_data("openmodel.geometry/Vector", geometric_data, minimal)
    }
}

/// Implementation of HasJsonData trait for ultra-simple API
impl HasJsonData for Vector {
    fn to_json_data(&self, minimal: bool) -> Value {
        self.to_json_data(minimal)
    }
}

/// Implementation of FromJsonData trait for direct deserialization
impl FromJsonData for Vector {
    /// Create Vector directly from JSON data - no casting needed!
    /// 
    /// # Example
    /// ```
    /// use openmodel::geometry::Vector;
    /// use openmodel::common::FromJsonData;
    /// use serde_json::json;
    /// 
    /// let json = json!({"data": {"x": 1.0, "y": 2.0, "z": 3.0}, "name": "MyVector"});
    /// let vector = Vector::from_json_data(&json).unwrap();
    /// assert_eq!(vector.x, 1.0);
    /// ```
    fn from_json_data(data: &Value) -> Option<Self> {
        let x = data["data"]["x"].as_f64()?;
        let y = data["data"]["y"].as_f64()?;
        let z = data["data"]["z"].as_f64()?;
        
        let mut vector = Vector::new(x, y, z);
        
        // Set name if available
        if let Some(name) = data["name"].as_str() {
            vector.data.set_name(name);
        }
        
        Some(vector)
    }
}